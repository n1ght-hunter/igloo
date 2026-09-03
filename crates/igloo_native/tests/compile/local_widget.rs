pub fn example() {
    let value=std::rc::Rc::new(1_u32);
    let _:igloo_native::Element<u32>=igloo_native::widgets::button(igloo_native::widgets::text("local"))
        .on_press(move ||*value).into();
}
