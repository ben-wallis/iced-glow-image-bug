use crate::image::Handle;
use iced::widget::column;
use iced::widget::text;
use iced::widget::{container, image};
use iced::{executor, ContentFit, Length};
use iced::{Application, Command, Element, Settings, Theme};
pub const TEST_IMAGE: &[u8] = include_bytes!("test_image.png");

pub fn main() -> iced::Result {
    GlowImageBugRepro::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

struct GlowImageBugRepro {}

#[derive(Debug, Clone, Copy)]
enum Message {}

impl Application for GlowImageBugRepro {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (GlowImageBugRepro {}, Command::none())
    }

    fn title(&self) -> String {
        String::from("ContentFit::Cover bug reproduction")
    }

    fn update(&mut self, _: Message) -> Command<Message> {
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let add_image = |fit: ContentFit| {
            column![
                text(format!("{:?}", fit)),
                container(image(Handle::from_memory(TEST_IMAGE.to_vec())).content_fit(fit))
                    .width(Length::Units(200))
                    .height(Length::Units(125))
                    .style(iced::theme::Container::Box)
            ]
        };

        let column = column![
            add_image(ContentFit::Cover),
            add_image(ContentFit::Contain),
            add_image(ContentFit::None),
            add_image(ContentFit::Fill),
            add_image(ContentFit::ScaleDown)
        ]
        .spacing(10);

        container(column).into()
    }
}
