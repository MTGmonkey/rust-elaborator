# Rust Elaborator

This program serves to take a list of boardgames as a csv and return a csv with more data about them.

## Building

### Clone the git repo locally

`git clone https://git.mtgmonkey.net/Andromeda/rust-elaborator.git`
`cd rust-elaborator`

### Run the sample

`cp sample_in.csv in.csv`
`nix run`

the output `out.csv` should match the provided `sample_out.csv`

## Quick Windows user guide

- Make a new folder
- Put `rust_elaborator.exe` into that folder
- Copy your `in.csv`, a list of game names as specified in `Usage`, or `out.csv`, an output from when this program ran previously, into the new folder
- Double-click `rust_elaborator.exe` to run it
- Check that `out.csv` is satisfactory

## Usage

# WARNING: files may be overwritten

The files `in.csv`, `out.csv`, `test.csv`, `copy.csv`, and possibly others may be overwritten and data loss may occur. Run this program in an empty directory with a copy of `in.csv` or `out.csv` and nothing else present for safety.

The program reads a file `in.csv` or `out.csv` and outpus it to `out.csv`. The following command reads the contents of `in.csv` into the program and runs it.
note that on windows, `rust_elaborator` instead looks like `rust_elaborator.exe`

`rust_elaborator`

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

If `out.csv` is present, the program will take it as input and elaborate on it. This will fill in rows that have a) a blank second column or b) NOT_FOUND in the second column, if possible.
