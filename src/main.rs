use std::fmt::Display;

use clap::Parser;
use nom::bytes::complete::{tag, take_till1, take_while1};
use nom::character::complete::digit1;
use nom::combinator::{all_consuming, map_res};
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
    all_consuming(separated_list1(tag(","), parse_usize))(i)
}

#[derive(clap::Parser)]
#[command(author, version, about)]
struct Options {
    /// In the format of comma delimited positive integers. At least one field must be specified.
    /// The field number starts from 0.
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
        print_row(field_ids.iter().filter_map(|&i| fields.get(i)));
    }
}
