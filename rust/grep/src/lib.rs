use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Error;

/// While using `&[&str]` to handle flags is convenient for exercise purposes,
/// and resembles the output of [`std::env::args`], in real-world projects it is
/// both more convenient and more idiomatic to contain runtime configuration in
/// a dedicated struct. Therefore, we suggest that you do so in this exercise.
///
/// In the real world, it's common to use crates such as [`clap`] or
/// [`structopt`] to handle argument parsing, and of course doing so is
/// permitted in this exercise as well, though it may be somewhat overkill.
///
/// [`clap`]: https://crates.io/crates/clap
/// [`std::env::args`]: https://doc.rust-lang.org/std/env/fn.args.html
/// [`structopt`]: https://crates.io/crates/structopt
#[derive(Debug)]
pub struct Flags<'a>(Vec<&'a str>);
// i think it should be fine to go with static

impl<'a> Flags<'a> {
    pub fn new(flags: &[&'a str]) -> Self {
        Self(flags.iter().map(|&flag| flag).collect())
    }
}

// if single file result
// multifile file:result
// -n Print the line numbers of each matching line.
// add
// -l Print only the names of files that contain at least one matching line.
// change output, this is exclusif i guess
// -i Match line using a case-insensitive comparison.
// mode, match modifier
// -v Invert the program -- collect all lines that fail to match the pattern.
// mode
// -x Only match entire lines, instead of lines that contain a match.
// mode

pub fn grep<'a>(pattern: &str, flags: &'a Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut r: Vec<String> = Vec::new();

    let is_multifile = files.len() > 1;
    let is_n_flag = flags.0.contains(&"-n");
    for path in files.into_iter() {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        for (n, line) in reader.lines().enumerate() {
            let line = line?;

            // matching
            let mut matched = match (flags.0.contains(&"-i"), flags.0.contains(&"-x")) {
                (true, true) => line.to_lowercase() == pattern.to_lowercase(),
                (false, true) => line == pattern,
                (false, false) => line.contains(pattern),
                (true, false) => line.to_lowercase().contains(&pattern.to_lowercase()),
                _ => line.contains(pattern),
            };

            if flags.0.contains(&"-v") {
                matched = !matched
            }
            // with -l flag i guess we should print the file only once
            // dbg!(matched, path, pattern, &line);
            // println!("\n\n");

            // printing
            // file.txt:1:line
            if matched {
                let mut line_r = String::new();
                // not too sure about this one
                match (is_multifile, flags.0.contains(&"-l")) {
                    (_, true) => {
                        if !r.contains(&path.to_string()) {
                            line_r.push_str(path);
                        }
                    }
                    (true, false) => {
                        line_r.push_str(path);
                        line_r.push(':');
                    }
                    _ => {}
                }
                if !flags.0.contains(&"-l") {
                    if is_n_flag {
                        line_r.push_str(&(n + 1).to_string());
                        line_r.push(':');
                    }
                    line_r.push_str(&line.to_string());
                }
                if !line_r.is_empty() {
                    r.push(line_r);
                }
            }
        }
    }
    Ok(r)
}
