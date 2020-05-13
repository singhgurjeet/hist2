use std::fmt::Error;
use std::env;

pub async fn compute_histogram() -> Result<Vec<(String, usize)>, Error> {
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

    Result::Ok(ret)
}