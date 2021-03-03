use std::mem;

use druid::im::{vector, Vector};
use druid::widget::{Button, Flex, Label, List, Painter, Scroll, SizedBox, TextBox};
use druid::{AppLauncher, Color, Data, Lens, RenderContext, Widget, WidgetExt, WindowDesc};

mod zdb;

#[derive(Clone, Data, Lens)]
struct State {
    to_send: String,
    messages: Vector<String>,
}

fn main() {
    // let main_window = WindowDesc::new(build_root_widget)
    //     .title("Hello World!")
    //     .window_size((800.0, 400.0));

    // let state = State {
    //     to_send: String::new(),
    //     messages: vector![],
    // };

    // AppLauncher::with_window(main_window)
    //     .launch(state)
    //     .expect("Failed to launch");
    println!("START");
    let s = zdb::state();
    dbg!(s);
    println!("END");
}

fn build_root_widget() -> impl Widget<State> {
    Flex::row()
        .with_child(SizedBox::new(rectangle(Color::RED)).width(50.0))
        .with_child(SizedBox::new(rectangle(Color::BLUE)).width(200.0))
        .with_flex_child(
            Flex::column()
                .with_flex_child(
                    Scroll::new(List::new(|| {
                        Label::new(|a: &String, _: &_| a.to_owned()).expand_width()
                    }))
                    .vertical()
                    .lens(State::messages)
                    .expand_height(),
                    1.0,
                )
                .with_child(
                    SizedBox::new(TextBox::multiline().lens(State::to_send))
                        .height(100.0)
                        .expand_width(),
                )
                .with_child(Button::new("Send").on_click(|_, data: &mut State, _| {
                    let msg = mem::take(&mut data.to_send);
                    data.messages.push_back(msg);
                })),
            1.0,
        )
        .with_child(SizedBox::new(rectangle(Color::GREEN)).width(200.0))
}

fn rectangle<T: Data>(c: Color) -> impl Widget<T> {
    Painter::new(move |rc, _, _| {
        let rect = rc.size().to_rect();
        rc.fill(rect, &c)
    })
}
