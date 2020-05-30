#[macro_use]
extern crate clap;

use iced::{canvas, executor, mouse, Application, Canvas, Command, Element, Length,
           Point, Settings, Size, Column, Text, Row, HorizontalAlignment,
           VerticalAlignment, Rectangle};
use std::fmt::Error;
use atty::Stream;
use iced::canvas::{Cache, Cursor, Geometry, Path, Event};

mod data;
mod styles;

pub fn main() {
    App::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

pub enum InputSource {
    FileName(String),
    Stdin
}

struct App {
    data: Hist,
    loaded: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Loaded(Result<(Vec<(String, usize)>, Option<f32>, Option<f32>, Option<f32>), Error>),
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let matches = clap_app!(myapp =>
            (version: "0.1")
            (about: "Plot the distribution of input. Data must either be piped in or given as an argument")
            (@arg BINS: -b --bins +takes_value "The number of bins in the histogram")
            (@arg INPUT: "Sets the input file to use")
        ).get_matches();
        let input = if !atty::is(Stream::Stdin) {
            InputSource::Stdin
        } else {
            InputSource::FileName(matches.value_of("INPUT").expect("No input").to_owned())
        };
        (App {
            data: Hist { labels_and_counts: vec!{("a".to_owned(), 10)},
                p_25: None, p_50: None, p_75: None, bars: Default::default(), highlight: None},
            loaded: false,
        },
         Command::perform(data::compute_histogram(
             matches.value_of("BINS").unwrap_or("20").parse::<usize>().unwrap(),
             input), Message::Loaded),
        )
    }

    fn title(&self) -> String {
        String::from("Histogram")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Loaded(Ok((labels_and_counts, p_25, p_50, p_75))) => {
                *self = App {
                    data: Hist {labels_and_counts, p_25, p_50, p_75, bars: Default::default(), highlight: None},
                    loaded: true,
                };
                Command::none()
            },
            _ => { Command::none() }
        }
    }

    fn view(&mut self) -> Element<Message> {
        if !self.loaded {
            Text::new("Loading...")
                .size(55)
                .width(Length::Fill)
                .height(Length::Fill)
                .horizontal_alignment(HorizontalAlignment::Center)
                .vertical_alignment(VerticalAlignment::Center)
                .into()
        } else {
            let labels: Element<_> = self.data.labels_and_counts
                .iter()
                .map(|(c, _)| {
                    Text::new(format!("{:.5}", *c))
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
                .push(self.data.view())
                .push(labels)
                .push(counts)
                .into()
        }
    }
}

#[derive(Default)]
struct Hist {
    labels_and_counts: Vec<(String, usize)>,
    p_25: Option<f32>,
    p_50: Option<f32>,
    p_75: Option<f32>,
    highlight: Option<usize>,
    bars: Cache
}

impl Hist {
    fn view(&mut self) -> Element<'_, Message> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl canvas::Program<Message> for Hist {
    fn update(
        &mut self,
        _event: Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Option<Message> {
        if let Some(cursor_position) = cursor.position_in(&bounds) {
            self.highlight = Some(((self.labels_and_counts.len() as f32) * cursor_position.x/bounds.width) as usize);
            self.bars.clear();
        } else {
            self.highlight = None;
            self.bars.clear();
        }
        None
    }

    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let bars = self.bars.draw(bounds.size(), |frame| {
            let width = frame.width();
            let height = frame.height();
            let num_bins = self.labels_and_counts.len() as f32;
            let bar_width = width / num_bins;
            let max_count = *self.labels_and_counts.iter()
                .map(|(_, i)| i)
                .max_by(|x, y| x.cmp(y)).unwrap_or(&(0 as usize));
            let height_per_count = height / (max_count as f32);
            frame.fill(&Path::rectangle(Point::new(0 as f32, 0 as f32), Size::new(width, height)),
                       styles::FRAME_BG_FILL);
            if let Some(p_25) = self.p_25 {
                frame.stroke(&Path::line(Point::new(p_25*width, 0.0), Point::new(p_25*width, height)), styles::PERCENTILE_STROKE);
            }
            if let Some(p_50) = self.p_50 {
                frame.stroke(&Path::line(Point::new(p_50*width, 0.0), Point::new(p_50*width, height)), styles::PERCENTILE_STROKE);
            }
            if let Some(p_75) = self.p_75 {
                frame.stroke(&Path::line(Point::new(p_75 * width, 0.0), Point::new(p_75 * width, height)), styles::PERCENTILE_STROKE);
            }
            self.labels_and_counts.iter().enumerate().for_each(|(i, (_, c))| {
                let r = Path::rectangle(
                    Point::new((i as f32) * bar_width, height - (*c as f32) * height_per_count),
                    Size::new(bar_width, (*c as f32) * height_per_count));
                if self.highlight == Some(i) {
                    frame.fill(&r, styles::H_BAR_FILL);
                } else {
                    frame.fill(&r, styles::BAR_FILL);
                }
                frame.stroke(&r, styles::BAR_STROKE)
            });
        });
        vec![bars]
    }

    fn mouse_interaction( &self, _bounds: Rectangle, _cursor: Cursor) -> mouse::Interaction {
        mouse::Interaction::Crosshair
    }
}