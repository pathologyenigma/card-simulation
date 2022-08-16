use iced::{
    scrollable, Application, Column, Command, Container, Font, HorizontalAlignment, Scrollable,
    Text, VerticalAlignment,
};
#[derive(Debug, Clone, Copy, Default)]
pub struct App {
    scrollable: scrollable::State,
    font: Font,
}

impl iced::Application for App {
    type Executor = iced::executor::Default;

    type Message = ();

    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let font = Font::External {
            name: "ArianaVioleta-dz2K",
            bytes: include_bytes!("../fonts/ArianaVioleta-dz2K.ttf"),
        };
        (
            App {
                font,
                ..Default::default()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "card-client-with-gui".to_owned()
    }

    fn update(
        &mut self,
        message: Self::Message,
        clipboard: &mut iced::Clipboard,
    ) -> Command<Self::Message> {
        Command::none()
    }
    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let header = Text::new("Card Client with GUI")
            .size(64)
            .width(iced::Length::Fill)
            .font(self.font)
            .horizontal_alignment(HorizontalAlignment::Center);
        let body = Text::new("This is a card client with GUI")
            .size(32)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .font(self.font)
            .horizontal_alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Center);
        let content = Column::new()
            .push(header)
            .push(
                Scrollable::new(&mut self.scrollable)
                    .push(Container::new(body).width(iced::Length::Fill).center_y())
                    .height(iced::Length::Fill),
            )
            .width(iced::Length::Fill)
            .height(iced::Length::Fill);
        Container::new(content)
            .width(iced::Length::Fill)
            .center_x()
            .into()
    }
}

fn main() -> iced::Result {
    App::run(Default::default())
}
