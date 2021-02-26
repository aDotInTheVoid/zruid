use druid::widget::{Flex, Painter, Scroll, SizedBox, TextBox};
use druid::{AppLauncher, Color, Data, Lens, RenderContext, Widget, WidgetExt, WindowDesc};

#[derive(Clone, Data, Lens)]
struct State {
    to_send: String
}



fn main() {
    let main_window = WindowDesc::new(build_root_widget)
        .title("Hello World!")
        .window_size((800.0, 400.0));

    let state = State {
        to_send: String::new()
    };


    AppLauncher::with_window(main_window)
        .launch(state)
        .expect("Failed to launch");

}

fn build_root_widget() -> impl Widget<State> {
    Flex::row()
        .with_child(SizedBox::new(rectangle(Color::RED)).width(50.0))
        .with_child(SizedBox::new(rectangle(Color::BLUE)).width(200.0))
        .with_flex_child(
            Flex::column()
                .with_flex_child(Scroll::new(messages()).vertical(), 1.0)
                .with_child(SizedBox::new(TextBox::multiline().lens(State::to_send)).height(100.0).expand_width()),
            1.0,
        )
        .with_child(SizedBox::new(rectangle(Color::GREEN)).width(200.0))
}

fn messages<T: Data>() -> impl Widget<T> {
    let mut result = Flex::column();
    for _ in 0..10 {
        result.add_child(SizedBox::new(rectangle(Color::SILVER)).height(30.0));
        result.add_child(SizedBox::new(rectangle(Color::GRAY)).height(30.0));
    }

    result
}

fn rectangle<T: Data>(c: Color) -> impl Widget<T> {
    Painter::new(move |rc, _, _| {
        let rect = rc.size().to_rect();
        rc.fill(rect, &c)
    })
}
