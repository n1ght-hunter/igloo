pub fn example() {
    let value=std::rc::Rc::new(1_u32);
    let _=igloo_native::Subscription::run(1,move ||futures::stream::iter([*value]));
}
