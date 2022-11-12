use iced::{
    button::State, Application, Button, Color, Column, Container, Element, Error, Length, Padding,
    Row, Rule, Sandbox, Settings, Text,
};

use crate::sentances;

pub struct JapaneseSentanceApp {
    pub sentances: Vec<crate::sentances::Sentance>,
    pub current: usize,
    pub button_state: State,
    pub revealed: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum JapaneseSentanceAppMessage {
    RevealOrNext,
    Quit,
}

impl Default for JapaneseSentanceApp {
    fn default() -> Self {
        <JapaneseSentanceApp as Sandbox>::new()
    }
}

impl Sandbox for JapaneseSentanceApp {
    type Message = JapaneseSentanceAppMessage;

    fn new() -> Self {
        let sentances = sentances::load_sentances();
        let count = sentances.len();
        JapaneseSentanceApp {
            sentances,
            current: rand::random::<usize>() % count,
            button_state: State::new(),
            revealed: false,
        }
    }

    fn title(&self) -> String {
        "Example Japanese Sentances".to_owned()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            JapaneseSentanceAppMessage::RevealOrNext => {
                if self.revealed {
                    self.revealed = !self.revealed;
                    self.current = rand::random::<usize>() % self.sentances.len();
                } else {
                    self.revealed = !self.revealed
                }
            }
            JapaneseSentanceAppMessage::Quit => todo!(),
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let display = Column::new()
            .push(Text::new(&self.sentances[self.current].jp))
            .push(
                Text::new(&self.sentances[self.current].en).color(if self.revealed {
                    Color::BLACK
                } else {
                    Color::TRANSPARENT
                }),
            );
        let controls = Column::new()
            .push(Text::new("Right side").width(Length::Fill))
            .push(
                Button::new(
                    &mut self.button_state,
                    Text::new(if self.revealed { "Next" } else { "Reveal" }),
                )
                .on_press(JapaneseSentanceAppMessage::RevealOrNext),
            );
        Container::new(
            Row::new()
                .push(display.width(Length::Fill))
                .push(Rule::vertical(4))
                .push(controls.width(Length::Fill))
                .width(Length::Fill)
                .height(Length::Fill)
                .align_items(iced::Alignment::Center),
        )
        .into()
    }

    fn background_color(&self) -> Color {
        Color::from_rgb(0.5, 0.5, 0.5)
    }

    fn scale_factor(&self) -> f64 {
        1.0
    }

    fn should_exit(&self) -> bool {
        false
    }

    fn run(settings: Settings<()>) -> Result<(), Error>
    where
        Self: 'static + Sized,
    {
        <Self as Application>::run(settings)
    }
}
