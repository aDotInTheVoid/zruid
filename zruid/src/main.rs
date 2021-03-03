use std::mem;

use druid::im::Vector;
use druid::widget::{
    Button, Flex, Label, List, Painter, Scroll, SizedBox, TabInfo, Tabs, TabsPolicy, TextBox,
};
use druid::{AppLauncher, Color, Data, Lens, RenderContext, Widget, WidgetExt, WindowDesc};

mod zdb;

#[derive(Clone, Data, Lens)]
struct State {
    to_send: String,
    messages: Vector<String>,
}

fn message_widget() -> impl Widget<zdb::Message> {
    Label::new(|data: &zdb::Message, _: &_| format!("From {}: {}", data.from, data.content))
}

fn topic_widget() -> impl Widget<zdb::Topic> {
    List::new(message_widget).lens(zdb::Topic::messages)
}
#[derive(Clone, Data)]
struct TopicsTab;

impl TabsPolicy for TopicsTab {
    type Key = String;
    type Input = zdb::Stream;

    type BodyWidget = Box<dyn Widget<Self::Input>>;

    type LabelWidget = Label<Self::Input>;

    type Build = ();

    fn tabs_changed(&self, old_data: &Self::Input, data: &Self::Input) -> bool {
        old_data == data
    }

    fn tabs(&self, data: &Self::Input) -> Vec<Self::Key> {
        data.topics.iter().map(|x| x.name.clone()).collect()
    }

    fn tab_info(&self, key: Self::Key, data: &Self::Input) -> druid::widget::TabInfo<Self::Input> {
        TabInfo::new(format!("Tab {:?}", key), false)
    }

    fn tab_body(&self, key: Self::Key, data: &Self::Input) -> Self::BodyWidget {
        Box::new(Label::new(format!("Dynamic tab body {:?}", key)))
    }

    fn tab_label(
        &self,
        key: Self::Key,
        info: druid::widget::TabInfo<Self::Input>,
        data: &Self::Input,
    ) -> Self::LabelWidget {
        Self::default_make_label(info)
    }
}

fn stream_widget() -> impl Widget<zdb::Stream> {
    Tabs::for_policy(TopicsTab)
}

fn main() {
    let main_window = WindowDesc::new(stream_widget)
        .title("Hello World!")
        .window_size((800.0, 400.0));

    let state = zdb::state().streams[1].clone();

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
