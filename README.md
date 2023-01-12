# excavate

A command line tool to excavate columns from multi-column texts.

`excavate` is a simple program that does one specific job, getting and printing the specified column, line-by-line reading from standard input. This is a job that can be done by some more sophisticated programs, e.g. `awk` or `sed`, but having the memory capacity of a goldfish, I never could recall their syntax when I need to. Personally, I frequently need the fetching of a column, and infrequently need anything that requires the rich feature-set offered by those tools. That's my rationale for creating this highly specialized, simplistic, one-trick-pony program.

## Usage:

```bash
excavate <COLUMNS>
```

`<COLUMNS>` is supplied in the format of comma delimited list. Each item can be a positive integer, or a range in the format of "a-b". At least one column must be specified. The column ID starts from 0 (first column is 0, the second 1, etc.). The range "a-b" includes both end, with "a" being the lower end, and "b" the higher end. If "a" is greater than "b", the range is empty (selects no column).
    
For example:

| fields | Description                                         |
| :----- | :-------------------------------------------------- |
| 0      | Select the first column                             |
| 0-2    | Select the first three columns                      |
| 0,3,5  | Select the first, the fourth, and the sixth column  |
| 0-3,5  | Select the first four columns, and the sixth column |

For example:

Printing the all the process IDs matching "bash":

```bash
$ ps aux | grep bash 
henry     7308  0.0  0.0   8988  5652 pts/3    S+   12:03   0:00 bash
henry     7482  0.0  0.0   8988  5500 pts/4    S+   12:03   0:00 bash
henry     7656  0.0  0.0   8988  5540 pts/5    S+   12:03   0:00 bash
henry     8776  0.0  0.0   6564  2176 pts/1    S+   12:08   0:00 grep --color=auto bash

$ ps aux | grep bash | excavate 1
7308
7482
7656
8826
```

The columns, regardless of your input order, will always be printed from low to high, i.e. from left to right in the table, and duplicates are ignored. `excavate 1,2` is equivalent to `excavate 2,1,1`.

If a line has not enough columns, `excavate` ignores any field beyond the limit. For instance, running `excavate 2,10` against the input `bob alice eve` gets you `eve`. Running it against `bob alice` gets you a blank line. The output always has the same number of lines as the input, even if some of them are empty.

## Installation

At the moment, the program can be installed via `cargo`:

```bash
cargo install excavate
```

## Build

`excavate` has only Rust dependencies, `clap` and `nom`. There's no external dependencies. Clone this repository, and run

```bash
cargo build
```
