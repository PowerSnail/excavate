use std::fmt::Display;

use clap::Parser;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_till1, take_while1};
use nom::character::complete::digit1;
use nom::combinator::{all_consuming, map, map_res};
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::separated_pair;
use nom::IResult;

fn parse_line(i: &str) -> IResult<&str, Vec<&str>> {
    separated_list0(
        take_while1(|c: char| c.is_whitespace()),
        take_till1(|c: char| c.is_whitespace()),
    )(i)
}

fn parse_usize(i: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(i)
}

fn parse_range(i: &str) -> IResult<&str, (usize, usize)> {
    separated_pair(parse_usize, tag("-"), parse_usize)(i)
}

fn parse_fields(i: &str) -> IResult<&str, Vec<(usize, usize)>> {
    map(
        all_consuming(separated_list1(
            tag(","),
            alt((parse_range, map(parse_usize, |r| (r, r)))),
        )),
        sort_and_merge_ranges,
    )(i)
}

fn sort_and_merge_ranges(mut ranges: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    if ranges.is_empty() {
        return vec![];
    }
    ranges.sort();
    let mut queue = vec![ranges[0]];
    for (new_lo, new_hi) in ranges.into_iter().skip(1) {
        let (lo, hi) = queue.pop().unwrap();
        if new_lo <= hi + 1 {
            queue.push((lo, new_hi.max(hi)));
        } else {
            queue.push((lo, hi));
            queue.push((new_lo, new_hi));
        }
    }
    queue
}

#[derive(clap::Parser)]
#[command(author, version, about)]
struct Options {
    /// In the format of comma delimited list.
    /// Each item can be a positive integer, or a range in the format of "a-b".
    /// At least one field must be specified.
    /// The field id starts from 0.
    ///
    /// For example:
    ///
    /// 0           Select the first column  
    /// 0-2         Select the first three columns
    /// 0,3,5       Select the first, the fourth, and the sixth column  
    /// 0-3,5       Select the first four columns, and the sixth column  
    #[clap(verbatim_doc_comment)]
    fields: String,
}

fn print_row<T>(cells: impl Iterator<Item = T>)
where
    T: Display,
{
    let mut delim = "";
    for i in cells {
        print!("{}{}", delim, &i);
        delim = "\t";
    }
    println!("");
}

trait ErrorAndExit<R, E> {
    fn unwrap_or_exit(self, msg: &str) -> R
    where
        E: Display;
}

impl<R, E> ErrorAndExit<R, E> for Result<R, E> {
    fn unwrap_or_exit(self, msg: &str) -> R
    where
        E: Display,
    {
        match self {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}: {}", msg, &e);
                std::process::exit(1);
            }
        }
    }
}

fn main() {
    let opts = Options::parse();
    let field_ids = parse_fields(&opts.fields)
        .unwrap_or_exit("Faile to parse fields")
        .1;

    for line in std::io::stdin().lines() {
        let line = line.unwrap_or_exit("Failed to fetch line from stdin");
        let fields = parse_line(&line).unwrap_or_exit("Failed to parse input").1;
        print_row(
            field_ids
                .iter()
                .flat_map(|&(lo, hi)| lo..=hi)
                .map(|i| fields[i]),
        );
    }
}
