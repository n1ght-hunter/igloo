//! Typed guest tasks, erased only by the application adapter.
use crate::wire_task;
use futures::{FutureExt, StreamExt};

enum Work<M> {
    Output(M),
    Future(futures::future::BoxFuture<'static, M>),
    Stream(futures::stream::BoxStream<'static, M>),
    Effect(wire_task::Action),
    Clipboard(Box<dyn Fn(Option<String>) -> M + Send>),
    Sleep(u64, Box<dyn Fn() -> M + Send>),
    Sequence(Task<M>, Task<M>),
}
/// Work returned by a typed plugin application.
pub struct Task<M> {
    steps: Vec<Work<M>>,
}
impl<M: Send + 'static> Task<M> {
    pub fn none() -> Self {
        Self { steps: vec![] }
    }
    pub fn done(value: M) -> Self {
        Self {
            steps: vec![Work::Output(value)],
        }
    }
    /// Runs a future and emits its result.
    pub fn future(f: impl std::future::Future<Output = M> + Send + 'static) -> Self {
        Self {
            steps: vec![Work::Future(f.boxed())],
        }
    }
    /// Emits each value produced by a stream.
    pub fn stream(s: impl futures::Stream<Item = M> + Send + 'static) -> Self {
        Self {
            steps: vec![Work::Stream(s.boxed())],
        }
    }
    /// Runs the supplied tasks concurrently.
    pub fn batch(tasks: impl IntoIterator<Item = Self>) -> Self {
        Self {
            steps: tasks.into_iter().flat_map(|t| t.steps).collect(),
        }
    }
    /// Runs this task to completion before starting the next task.
    pub fn chain(self, then: Self) -> Self {
        Self {
            steps: vec![Work::Sequence(self, then)],
        }
    }
    /// Waits before producing a message on the host executor.
    pub fn sleep(duration: std::time::Duration, f: impl Fn() -> M + Send + 'static) -> Self {
        Self {
            steps: vec![Work::Sleep(
                duration
                    .as_millis()
                    .try_into()
                    .expect("duration exceeds u64 milliseconds"),
                Box::new(f),
            )],
        }
    }
    /// Reads clipboard text with a callback safe to move to the executor.
    ///
    pub fn read_clipboard(f: impl Fn(Option<String>) -> M + Send + 'static) -> Self {
        Self {
            steps: vec![Work::Clipboard(Box::new(f))],
        }
    }
    pub fn write_clipboard(text: impl Into<String>) -> Self {
        Self::effect(wire_task::Action::WriteClipboard(text.into().into()))
    }
    pub fn load_font(bytes: Vec<u8>) -> Self {
        Self::effect(wire_task::Action::LoadFont(bytes.into_iter().collect()))
    }
    pub fn exit() -> Self {
        Self::effect(wire_task::Action::Exit)
    }
    fn effect(a: wire_task::Action) -> Self {
        Self {
            steps: vec![Work::Effect(a)],
        }
    }
    /// Transforms every message, including delayed and streamed outputs.
    pub fn map<B: Send + 'static>(self, f: impl Fn(M) -> B + Send + Sync + 'static) -> Task<B> {
        self.map_shared(std::sync::Arc::new(f))
    }
    fn map_shared<B: Send + 'static>(
        self,
        f: std::sync::Arc<dyn Fn(M) -> B + Send + Sync>,
    ) -> Task<B> {
        Task {
            steps: self
                .steps
                .into_iter()
                .map(|step| match step {
                    Work::Output(v) => Work::Output(f(v)),
                    Work::Future(v) => {
                        let f = f.clone();
                        Work::Future(async move { f(v.await) }.boxed())
                    }
                    Work::Stream(v) => {
                        let f = f.clone();
                        Work::Stream(v.map(move |v| f(v)).boxed())
                    }
                    Work::Effect(a) => Work::Effect(a),
                    Work::Clipboard(g) => {
                        let f = f.clone();
                        Work::Clipboard(Box::new(move |v| f(g(v))))
                    }
                    Work::Sleep(ms, g) => {
                        let f = f.clone();
                        Work::Sleep(ms, Box::new(move || f(g())))
                    }
                    Work::Sequence(a, b) => {
                        Work::Sequence(a.map_shared(f.clone()), b.map_shared(f.clone()))
                    }
                })
                .collect(),
        }
    }
    pub(crate) fn erase(self) -> wire_task::Task {
        use crate::messages::Message;
        wire_task::Task::batch(self.steps.into_iter().map(|step| {
            match step {
                Work::Output(v) => wire_task::Task::done(Message::new(v)),
                Work::Future(v) => wire_task::Task::future(async move { Message::new(v.await) }),
                Work::Stream(v) => wire_task::Task::stream(v.map(Message::new)),
                Work::Effect(a) => wire_task::Task::effect(a),
                Work::Clipboard(g) => wire_task::Task::effect(wire_task::Action::ReadClipboard(
                    stabby::boxed::Box::new(
                        move |v: stabby::option::Option<stabby::string::String>| {
                            Message::new(g(
                                Option::<stabby::string::String>::from(v).map(String::from)
                            ))
                        },
                    )
                    .into(),
                )),
                Work::Sleep(ms, g) => wire_task::Task::sleep(
                    ms,
                    stabby::boxed::Box::new(move || Message::new(g())).into(),
                ),
                Work::Sequence(a, b) => a.erase().chain(b.erase()),
            }
        }))
    }
}
impl<M: Send + 'static> Default for Task<M> {
    fn default() -> Self {
        Self::none()
    }
}
impl<M> std::fmt::Debug for Task<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Task")
            .field("steps", &self.steps.len())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mapping_covers_immediate_async_and_sequence_outputs() {
        let task = Task::batch([
            Task::done(1),
            Task::future(async { 2 }),
            Task::stream(futures::stream::iter([3, 4])),
            Task::done(5).chain(Task::done(6)),
        ])
        .map(|v| format!("value:{v}"));
        let mut values = Vec::new();
        fn collect(task: Task<String>, values: &mut Vec<String>) {
            for work in task.steps {
                match work {
                    Work::Output(value) => values.push(value),
                    Work::Future(f) => values.push(futures::executor::block_on(f)),
                    Work::Stream(s) => {
                        values.extend(futures::executor::block_on(s.collect::<Vec<_>>()));
                    }
                    Work::Sequence(a, b) => {
                        collect(a, values);
                        collect(b, values);
                    }
                    _ => panic!("unexpected work"),
                }
            }
        }
        collect(task, &mut values);
        assert_eq!(
            values,
            [
                "value:1", "value:2", "value:3", "value:4", "value:5", "value:6"
            ]
        );
    }
    #[test]
    fn mapping_composes_clipboard_and_sleep_callbacks() {
        let task = Task::batch([
            Task::read_clipboard(Option::unwrap_or_default),
            Task::sleep(std::time::Duration::from_millis(10), || "tick".to_owned()),
        ])
        .map(|v| v.len())
        .map(|v| v + 1);
        let mut steps = task.steps.into_iter();
        let Work::Clipboard(callback) = steps.next().unwrap() else {
            panic!("expected clipboard");
        };
        assert_eq!(callback(Some("hello".to_owned())), 6);
        let Work::Sleep(ms, callback) = steps.next().unwrap() else {
            panic!("expected sleep");
        };
        assert_eq!(ms, 10);
        assert_eq!(callback(), 5);
    }
    #[test]
    fn dropping_an_unpolled_task_drops_its_captures() {
        struct Guard(std::sync::Arc<std::sync::atomic::AtomicUsize>);
        impl Drop for Guard {
            fn drop(&mut self) {
                self.0.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
        }
        let count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let guard = Guard(count.clone());
        let task = Task::future(async move {
            drop(guard);
            1
        });
        drop(task.erase());
        assert_eq!(count.load(std::sync::atomic::Ordering::Relaxed), 1);
    }
}
