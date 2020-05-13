use iced::{canvas, executor, Application, Canvas, Color, Command, Container, Element, Length, Point,
           Settings, Size, Column, Text, Row, HorizontalAlignment, VerticalAlignment};
use std::fmt::Error;
mod data;

pub fn main() {
    App::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

struct App {
    data: Hist,
    bars: canvas::layer::Cache<Hist>,
}

#[derive(Debug, Clone)]
enum Message {
    Loaded(Result<Vec<(String, usize)>, Error>)
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (App {
            data: Hist {
                labels_and_counts: vec! {
                    ("a".to_owned(), 10)}
            },
            bars: Default::default(),
        },
         Command::perform(data::compute_histogram(), Message::Loaded),
        )
    }

    fn title(&self) -> String {
        String::from("Histogram")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Loaded(Ok(data)) => {
                *self = App {
                    data: Hist{labels_and_counts: data},
                    bars: Default::default(),
                };
                Command::none()
            }
            _ => { Command::none() }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let canvas = Canvas::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(self.bars.with(&self.data));

        let labels: Element<_> = self.data.labels_and_counts
            .iter()
            .map(|(c, _)| {
                Text::new(format!("{}", *c))
                    .size(15)
                    .width(Length::Fill)
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .vertical_alignment(VerticalAlignment::Center)
            })
            .fold(Row::new()
                      .width(Length::Fill),
                  |row, label| row.push(label))
            .into();
        let counts: Element<_> = self.data.labels_and_counts
            .iter()
            .map(|(_, c)| {
                Text::new(format!("{}", *c))
                    .size(15)
                    .width(Length::Fill)
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .vertical_alignment(VerticalAlignment::Center)
            })
            .fold(Row::new()
                      .width(Length::Fill),
                  |row, label| row.push(label))
            .into();

        Column::new()
            .padding(1)
            .push(Container::new(canvas)
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(0)
            )
            .push(labels)
            .push(counts)
            .into()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Hist {
    labels_and_counts: Vec<(String, usize)>
}

impl canvas::Drawable for Hist {
    fn draw(&self, frame: &mut canvas::Frame) {
        use canvas::{Fill, Path, Stroke};
        let width = frame.width();
        let height = frame.height();
        let num_bins = self.labels_and_counts.len() as f32;
        let bar_width = width / num_bins;
        let max_count = *self.labels_and_counts.iter()
            .map(|(_, i)| i)
            .max_by(|x, y| x.cmp(y)).unwrap_or(&(0 as usize));
        let height_per_count = height / (max_count as f32);
        self.labels_and_counts.iter().enumerate().for_each(|(i, (_, c))| {
            let r = Path::rectangle(
                Point::new((i as f32) * bar_width, height - (*c as f32) * height_per_count),
                Size::new(bar_width, (*c as f32) * height_per_count));
            frame.fill(&r, Fill::Color(Color::BLACK));
            frame.stroke(&r, Stroke {
                width: 0.5,
                color: Color::from_rgba8(128, 128, 128, 1.0),
                ..Stroke::default()
            })
        });
    }
}