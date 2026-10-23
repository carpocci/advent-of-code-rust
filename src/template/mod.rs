use std::{env, fs};

pub mod aoc_cli;
pub mod commands;
pub mod runner;

pub use day::*;

mod day;
mod readme_benchmarks;
mod run_multi;
mod timings;

pub const ANSI_ITALIC: &str = "\x1b[3m";
pub const ANSI_BOLD: &str = "\x1b[1m";
pub const ANSI_RESET: &str = "\x1b[0m";

/// Helper function that reads a text file to a string.
#[must_use]
pub fn read_file(folder: &str, day: Day) -> String {
    match read_file_safe(folder, day) {
        Ok(content) => content,
        Err(e) => unreachable!("error reading file: {}", e),
    }
}

fn read_file_safe(folder: &str, day: Day) -> Result<String, std::io::Error> {
    let cwd = env::current_dir()?;
    let filepath = cwd.join("data").join(folder).join(format!("{day}.txt"));
    fs::read_to_string(filepath)
}

/// Helper function that reads a text file to string, appending a part suffix. E.g. like `01-2.txt`.
#[must_use]
pub fn read_file_part(folder: &str, day: Day, part: u8) -> String {
    match read_file_part_safe(folder, day, part) {
        Ok(content) => content,
        Err(e) => unreachable!("error reading file part: {}", e),
    }
}

fn read_file_part_safe(folder: &str, day: Day, part: u8) -> Result<String, std::io::Error> {
    let cwd = env::current_dir()?;
    let filepath = cwd
        .join("data")
        .join(folder)
        .join(format!("{day}-{part}.txt"));
    fs::read_to_string(filepath)
}

/// Creates the constant `DAY` and sets up the input and runner for each part.
///
/// The optional, second parameter (1 or 2) allows you to only run a single part of the solution.
#[macro_export]
macro_rules! solution {
    ($day:expr) => {
        $crate::solution!(@impl $day, [part_one, 1] [part_two, 2]);
    };
    ($day:expr, 1) => {
        $crate::solution!(@impl $day, [part_one, 1]);
    };
    ($day:expr, 2) => {
        $crate::solution!(@impl $day, [part_two, 2]);
    };

    (@impl $day:expr, $( [$func:expr, $part:expr] )*) => {
        /// The current day.
        const DAY: $crate::template::Day = $crate::day!($day);

        #[cfg(feature = "dhat-heap")]
        #[global_allocator]
        static ALLOC: dhat::Alloc = dhat::Alloc;

        fn main() -> Result<(), std::io::Error> {
            use $crate::template::runner::*;
            let input = $crate::template::read_file("inputs", DAY);
            $( run_part($func, &input, DAY, $part); )*
            Ok(())
        }
    };
}
