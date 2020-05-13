use iced::{canvas, executor, Application, Canvas, Color, Command, Container, Element, Length, Point,
           Settings, Size, Column, Text, Row, HorizontalAlignment, VerticalAlignment};
use std::env;
use std::fmt::Error;

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
    Loaded(Result<Hist, Error>)
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
         Command::perform(compute_histogram(), Message::Loaded),
        )
    }

    fn title(&self) -> String {
        String::from("Histogram")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Loaded(Ok(hist)) => {
                *self = App {
                    data: hist,
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
        use canvas::{Fill, Path};
        let width = frame.width();
        let height = frame.height();
        let num_bins = self.labels_and_counts.len() as f32;
        let bar_width = width / num_bins;
        let max_count = *self.labels_and_counts.iter()
            .map(|(_, i)| i)
            .max_by(|x, y| x.cmp(y)).unwrap_or(&(0 as usize));
        let height_per_count = height / (max_count as f32);
        self.labels_and_counts.iter().enumerate().for_each(|(i, (_, c))| {
            frame.fill(&Path::rectangle(
                Point::new((i as f32) * bar_width, height - (*c as f32) * height_per_count),
                Size::new(bar_width, (*c as f32) * height_per_count)),
                       Fill::Color(Color::BLACK));
        });
    }
}

async fn compute_histogram() -> Result<Hist, Error> {
    use atty::Stream;
    use std::fs::File;
    use std::io::{self, BufRead};
    use itertools::Itertools;

    let max_num_lines = 10_000_000;
    let num_bars = 20 as f32;

    let mut vals: Vec<String> = Vec::new();
    if !atty::is(Stream::Stdin) {
        for line in std::io::stdin().lock().lines().take(max_num_lines) {
            vals.push(line.unwrap().trim().to_owned());
        }
    } else {
        // nothing is being piped, let's look at args
        let args: Vec<String> = env::args().collect();
        if args.len() < 2 {
            panic!("Usage: hist2 <filename>");
        }
        let file = File::open(&args[1]).unwrap();
        for line in io::BufReader::new(file).lines().take(max_num_lines) {
            vals.push(line.unwrap().trim().to_owned());
        }
    }

    let mostly_string = vals.iter()
        .map(|x| x.parse::<f32>())
        .filter(|x| x.is_err())
        .count() > (vals.len() / 2);

    let num_uniques = vals.iter().unique().count() as f32;

    let ret: Vec<(String, usize)> = if mostly_string || num_uniques < num_bars {
        vals.iter()
            .sorted()
            .group_by(|e| (**e).to_owned())
            .into_iter()
            .map(|(k, group_k)| (k, group_k.count()))
            .sorted_by(|(_, i), (_, j)| i.cmp(j))
            .collect()
    } else {
        let nums: Vec<f32> = vals.iter()
            .map(|x| x.parse::<f32>())
            .filter(|x| !x.is_err())
            .map(|x| x.unwrap())
            .collect::<Vec<f32>>();
        let min = *nums.iter().min_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal)).unwrap_or(&(0 as f32));
        let max = *nums.iter().max_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal)).unwrap_or(&(0 as f32));
        let delta = (max - min) / num_bars;
        nums.iter()
            .map(|x| ((x - min) / delta).round() as usize)
            .sorted()
            .group_by(|e| *e)
            .into_iter()
            .map(|(k, group_k)| (k, group_k.count()))
            .sorted_by(|(i, _), (j, _)| i.cmp(j))
            .map(|(i, val)| (format!("{}", min + (i as f32) * delta + 0.5 as f32), val))
            // .map(|(i,val)| (format!("{}", i), val))
            .collect()
    };

    Result::Ok(Hist { labels_and_counts: ret })
}