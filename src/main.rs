use std::fmt::Display;

use clap::Parser;
use nom::bytes::complete::{tag, take_till1, take_while1};
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::multi::{separated_list0, separated_list1};
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

fn parse_fields(i: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(tag(","), parse_usize)(i)
}

#[derive(clap::Parser)]
struct Options {
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

fn main() {
    let opts = Options::parse();
    let (_, field_ids) = parse_fields(&opts.fields).expect("Failed to parse fields.");

    for line in std::io::stdin().lines() {
        let line = line.expect("Failed to read from standard input");
        let (_, fields) = parse_line(&line).expect(&format!("Failed to parse input: {}", line));
        print_row(field_ids.iter().map(|&i| fields[i]));
    }
}
