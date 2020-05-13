use std::fmt::Error;
use std::env;
use atty::Stream;
use std::fs::File;
use std::io::{self, BufRead};
use itertools::Itertools;

fn compare_f32(x: &f32, y: &f32) -> std::cmp::Ordering {
    x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal)
}

fn read_from_stdin(max_num_lines: usize) -> Vec<String> {
    let mut vals: Vec<String> = Vec::new();
    for line in std::io::stdin().lock().lines().take(max_num_lines) {
        vals.push(line.unwrap().trim().to_owned());
    }
    vals
}

fn read_from_file(file_name: &String, max_num_lines: usize) -> Vec<String> {
    let mut vals: Vec<String> = Vec::new();
    let file = File::open(file_name).unwrap();
    for line in io::BufReader::new(file).lines().take(max_num_lines) {
        vals.push(line.unwrap().trim().to_owned());
    }
    vals
}

fn is_mostly_strings(vals: &Vec<String>) -> bool {
    vals.iter()
        .filter(|x| x.len() > 0)
        .map(|x| x.parse::<f32>())
        .filter(|x| x.is_err())
        .count() > (vals.len() / 2)
}

fn histogram_from_categories(vals: &Vec<String>) -> Vec<(String, usize)> {
    vals.iter()
        .sorted()
        .group_by(|e| (**e).to_owned())
        .into_iter()
        .map(|(k, group_k)| (k, group_k.count()))
        .sorted_by(|(_, i), (_, j)| i.cmp(j))
        .collect()
}

fn histogram_from_numbers(vals: &Vec<String>, num_bars: &f32) -> Vec<(String, usize)> {
    let nums: Vec<f32> = vals.iter()
        .filter(|x| x.len() > 0)
        .map(|x| x.parse::<f32>())
        .filter(|x| !x.is_err())
        .map(|x| x.unwrap())
        .collect::<Vec<f32>>();
    let min = *nums.iter().min_by(|x, y| compare_f32(*x,*y)).unwrap_or(&(0 as f32));
    let max = *nums.iter().max_by(|x, y| compare_f32(*x,*y)).unwrap_or(&(0 as f32));
    let delta = (max - min) / num_bars;
    nums.iter()
        .map(|x| ((x - min) / delta).round() as usize)
        .sorted()
        .group_by(|e| *e)
        .into_iter()
        .map(|(k, group_k)| (k, group_k.count()))
        .sorted_by(|(i, _), (j, _)| i.cmp(j))
        .map(|(i, val)| (format!("{}", min + (i as f32) * delta + 0.5 as f32), val))
        .collect()
}

pub async fn compute_histogram() -> Result<Vec<(String, usize)>, Error> {
    let max_num_lines = 10_000_000;
    let num_bars = 20 as f32;

    let vals: Vec<String> = if !atty::is(Stream::Stdin) {
        read_from_stdin(max_num_lines)
    } else {
        let args: Vec<String> = env::args().collect();
        if args.len() < 2 {
            panic!("Usage: hist2 <filename>");
        }
        read_from_file(&args[1], max_num_lines)
    };

    let mostly_string = is_mostly_strings(&vals);

    let num_uniques = vals.iter().unique().count() as f32;

    if mostly_string || num_uniques < num_bars {
        Ok(histogram_from_categories(&vals))
    } else {
        Ok(histogram_from_numbers(&vals, &num_bars))
    }
}