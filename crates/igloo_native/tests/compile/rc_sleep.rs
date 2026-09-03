pub fn example() {
    let value=std::rc::Rc::new(1_u32);
    let _=igloo_native::Task::sleep(std::time::Duration::from_millis(1),move ||*value);
}
