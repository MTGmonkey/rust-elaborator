# Rust Elaborator

This program serves to take a list of boardgames as a csv and return a csv with more data about them.

## Building

### Clone the git repo locally

`git clone https://git.mtgmonkey.net/Andromeda/rust-elaborator.git`
`cd rust-elaborator`

### Run the sample

`cat sample_in.csv | nix run`

the output `out.csv` should match the provided `sample_out.csv`

## Usage

The program reads a csv from stdin and outputs it to `out.csv`. The following command reads the contents of `in.csv` into the program and runs it.

`cat in.csv | rust_elaborator`

`in.csv` must be formatted as follows...

|title|
|-|
|Monopoly|
|Abomination|
|7 Wonders|
|Uno|

...in excel or as follows...

```csv
title,
Monopoly,
Abomination,
7 Wonders,
Uno,
```

...as plaintext

where `title` can be anything.
Capitalization does not matter.
Additional columns will not be present in `out.csv`.
