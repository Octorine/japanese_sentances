use std::collections::HashMap;

use iced::{
    button::State, Application, Button, Color, Column, Container, Element, Error, Length, Padding,
    Row, Rule, Sandbox, Settings, Text,
};
use kanji::{level_table, Kanji, Level};

use crate::sentances::{self, Sentance};

pub struct JapaneseSentanceApp {
    pub level_table: HashMap<Kanji, Level>,
    pub sentances: Vec<crate::sentances::Sentance>,
    pub filtered_sentances: Vec<crate::sentances::Sentance>,
    pub current: usize,
    pub button_state: State,
    pub revealed: bool,
    pub level: kanji::Level,
}
impl JapaneseSentanceApp {
    pub fn apply_filter(&mut self) {
        let table = &self.level_table;
        self.filtered_sentances = self
            .sentances
            .iter()
            .filter(|s| {
                s.jp.chars()
                    .filter_map(Kanji::new)
                    .filter_map(|k| table.get(&k))
                    .all(|l| l <= &self.level)
            })
            .cloned()
            .collect::<Vec<Sentance>>()
    }
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
        let mut app = JapaneseSentanceApp {
            level_table: level_table(),
            sentances,
            filtered_sentances: vec![],
            current: 0,
            button_state: State::new(),
            revealed: false,
            level: kanji::Level::One,
        };
        app.apply_filter();

        let count = app.filtered_sentances.len();
        app.current = rand::random::<usize>() % count;
        app
    }

    fn title(&self) -> String {
        "Example Japanese Sentances".to_owned()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            JapaneseSentanceAppMessage::RevealOrNext => {
                if self.revealed {
                    self.revealed = !self.revealed;
                    self.current = rand::random::<usize>() % self.filtered_sentances.len();
                } else {
                    self.revealed = !self.revealed
                }
            }
            JapaneseSentanceAppMessage::Quit => todo!(),
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let current_sentance = &self.filtered_sentances[self.current];
        let display = Column::new().push(Text::new(&current_sentance.jp)).push(
            Text::new(&current_sentance.en).color(if self.revealed {
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
