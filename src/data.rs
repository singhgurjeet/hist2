use std::fmt::Error;
use std::fs::File;
use std::io::{self, BufRead};
use itertools::Itertools;
use crate::InputSource;
use std::collections::HashMap;

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

fn histogram_from_numbers(vals: &Vec<String>, num_bars: &usize) -> (Vec<(String, usize)>, Option<f32>, Option<f32>, Option<f32>) {
    let sorted_nums: Vec<f32> = vals.iter()
        .filter(|x| x.len() > 0)
        .map(|x| x.parse::<f32>())
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .sorted_by(|x, y| compare_f32(x,y))
        .collect::<Vec<f32>>();
    let min = *sorted_nums.first().unwrap();
    let max = *sorted_nums.last().unwrap();
    let delta = (max - min) / (*num_bars as f32);
    let len_25 = ((sorted_nums.len() as f32) * 0.25) as usize;
    let existing_counts: HashMap<usize, usize> = sorted_nums.iter().map(|x| ((*x - min) / delta) as usize)
        .group_by(|e| *e)
        .into_iter()
        .map(|(k, group_k)| (k, group_k.count()))
        .collect();
    (
        (0..*num_bars).map(|i| (i as usize, existing_counts.get(&(i as usize)).unwrap_or(&(0 as usize))))
        .map(|(i, val)| (format!("{}", min + (i as f32) * delta + 0.5 as f32), *val))
        .collect::<Vec<(String, usize)>>(),
     Some((sorted_nums[len_25] - min)/ (max-min)),
     Some((sorted_nums[len_25 * 2] - min)/ (max-min)),
     Some((sorted_nums[len_25 * 3] - min)/ (max-min))
    )
}

pub async fn compute_histogram(num_bins: usize, input: InputSource) -> Result<(Vec<(String, usize)>, Option<f32>, Option<f32>, Option<f32>), Error> {
    let max_num_lines = 10_000_000;

    let vals = match input {
        InputSource::Stdin => read_from_stdin(max_num_lines),
        InputSource::FileName(file_name) => read_from_file(&file_name, max_num_lines)
    };

    let mostly_string = is_mostly_strings(&vals);

    let num_uniques = vals.iter().unique().count();

    if mostly_string || num_uniques < num_bins {
        Ok((histogram_from_categories(&vals), None, None, None))
    } else {
        Ok(histogram_from_numbers(&vals, &num_bins))
    }
}