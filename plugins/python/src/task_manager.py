"""Example Python plugin for Igloo - Task Manager application."""

from dataclasses import dataclass
from typing import Literal

from igloo_py import (
    create_app,
    Text,
    Column,
    Row,
    Button,
    Container,
    TextInput,
    Checkbox,
    ProgressBar,
    Rule,
    Space,
    Scrollable,
    Length,
    Padding,
    MessageManager,
    ElementLike,
    Message as WitMessage,
)


# Task type
@dataclass
class Task:
    id: int
    text: str
    completed: bool


FilterType = Literal["all", "active", "completed"]


# App state
@dataclass
class State:
    tasks: list[Task]
    input_text: str
    next_id: int
    filter: FilterType


# Messages
@dataclass
class InputChanged:
    value: str


@dataclass
class AddTask:
    pass


@dataclass
class ToggleTask:
    id: int


@dataclass
class DeleteTask:
    id: int


@dataclass
class SetFilter:
    filter: FilterType


@dataclass
class ClearCompleted:
    pass


Msg = InputChanged | AddTask | ToggleTask | DeleteTask | SetFilter | ClearCompleted


def get_string(msg: WitMessage) -> str | None:
    """Extract string from message."""
    if isinstance(msg, dict) and msg.get("tag") == "string-type":
        return msg.get("val")
    return None


class TaskManagerApp:
    """Task Manager application following the Elm architecture."""

    def init(self) -> State:
        return State(
            tasks=[
                Task(1, "Learn igloo-py", True),
                Task(2, "Build a cool app", False),
                Task(3, "Share with others", False),
            ],
            input_text="",
            next_id=4,
            filter="all",
        )

    def update(self, state: State, msg: Msg) -> State:
        match msg:
            case InputChanged(value=value):
                return State(
                    tasks=state.tasks,
                    input_text=value,
                    next_id=state.next_id,
                    filter=state.filter,
                )

            case AddTask():
                if not state.input_text.strip():
                    return state
                new_task = Task(state.next_id, state.input_text.strip(), False)
                return State(
                    tasks=[*state.tasks, new_task],
                    input_text="",
                    next_id=state.next_id + 1,
                    filter=state.filter,
                )

            case ToggleTask(id=task_id):
                tasks = [
                    Task(t.id, t.text, not t.completed if t.id == task_id else t.completed)
                    for t in state.tasks
                ]
                return State(
                    tasks=tasks,
                    input_text=state.input_text,
                    next_id=state.next_id,
                    filter=state.filter,
                )

            case DeleteTask(id=task_id):
                return State(
                    tasks=[t for t in state.tasks if t.id != task_id],
                    input_text=state.input_text,
                    next_id=state.next_id,
                    filter=state.filter,
                )

            case SetFilter(filter=f):
                return State(
                    tasks=state.tasks,
                    input_text=state.input_text,
                    next_id=state.next_id,
                    filter=f,
                )

            case ClearCompleted():
                return State(
                    tasks=[t for t in state.tasks if not t.completed],
                    input_text=state.input_text,
                    next_id=state.next_id,
                    filter=state.filter,
                )

        return state

    def view(self, state: State, messages: MessageManager[Msg]) -> ElementLike:
        completed_count = sum(1 for t in state.tasks if t.completed)
        total_count = len(state.tasks)
        progress = completed_count / total_count if total_count > 0 else 0.0

        # Filter tasks
        if state.filter == "active":
            filtered_tasks = [t for t in state.tasks if not t.completed]
        elif state.filter == "completed":
            filtered_tasks = [t for t in state.tasks if t.completed]
        else:
            filtered_tasks = state.tasks

        # Build task list
        task_list = Column.new().spacing(8)
        for task in filtered_tasks:
            task_id = task.id  # Capture for closure
            task_list.push(
                Row.new()
                .spacing(10)
                .push(
                    Checkbox.new(task.completed)
                    .label(task.text)
                    .on_toggle(messages, lambda tid=task_id: ToggleTask(tid))
                )
                .push(Space.new().width(Length.fill()))
                .push(
                    Button.new(Text.new("×").size(16)).on_press(
                        messages, lambda tid=task_id: DeleteTask(tid)
                    )
                )
            )

        # Filter buttons helper
        def filter_button(label: str, f: FilterType) -> ElementLike:
            is_active = state.filter == f
            return Button.new(Text.new(label).size(14 if is_active else 12)).on_press(
                messages, lambda filter_val=f: SetFilter(filter_val)
            )

        # Empty state message
        if not filtered_tasks:
            if state.filter == "completed":
                empty_msg = "No completed tasks"
            elif state.filter == "active":
                empty_msg = "All tasks completed!"
            else:
                empty_msg = "No tasks yet. Add one above!"
            empty_widget: ElementLike = Text.new(empty_msg).size(14)
        else:
            empty_widget = Space.new()

        return (
            Container.new(
                Column.new()
                .spacing(16)
                .width(Length.fixed(400))
                .push(Text.new("Task Manager").size(28))
                .push(Rule.horizontal(1))
                # Progress section
                .push(
                    Column.new()
                    .spacing(4)
                    .push(
                        Text.new(f"Progress: {completed_count}/{total_count} tasks completed").size(
                            14
                        )
                    )
                    .push(ProgressBar.new(0, 1, progress).length(Length.fill()))
                )
                .push(Rule.horizontal(1))
                # Input section
                .push(
                    Row.new()
                    .spacing(10)
                    .push(
                        TextInput.new("Add a new task...", state.input_text)
                        .on_input(messages, lambda m: InputChanged(get_string(m) or ""))
                        .on_submit(messages, lambda: AddTask())
                        .width(Length.fill())
                        .padding(Padding.all(8))
                    )
                    .push(
                        Button.new(Text.new("Add"))
                        .on_press(messages, lambda: AddTask())
                        .padding(Padding.xy(16, 8))
                    )
                )
                # Filter buttons
                .push(
                    Row.new()
                    .spacing(8)
                    .push(filter_button("All", "all"))
                    .push(filter_button("Active", "active"))
                    .push(filter_button("Completed", "completed"))
                    .push(Space.new().width(Length.fill()))
                    .push(
                        Button.new(Text.new("Clear Completed")).on_press(
                            messages, lambda: ClearCompleted()
                        )
                    )
                )
                .push(Rule.horizontal(1))
                # Task list in scrollable container
                .push(Scrollable.new(task_list.padding(Padding.all(4))).height(Length.fixed(300)))
                # Empty state message
                .push(empty_widget)
            )
            .center(Length.fill())
            .padding(Padding.all(20))
        )


# Create the app instance
_app = create_app(TaskManagerApp())


# WitWorld class implementing the WIT world protocol
class WitWorld:
    """Implementation of the WIT world interface for the Task Manager app."""

    def update(self, message_id: int, message: WitMessage) -> None:
        """Handle an update message from the host."""
        _app.update(message_id, message)

    def view(self):
        """Render the current view."""
        return _app.view()


# Message export for cloning (required by WIT interface)
class Message:
    """Implementation of the WIT message export interface."""

    def clone_message(self, message: int) -> int:
        return message
