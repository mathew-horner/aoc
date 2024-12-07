/// Generate module definitions for each day's solution and define `run` to
/// dispatch to the right one.
///
/// Usage:
/// ```rust
/// days!(day1, day2, day3, ...);
/// ```
macro_rules! days {
    ($year:literal, $($day:ident), *) => {
        $( mod $day; )*

        pub fn run(day: &str, part: u8) {
            match day {
                $(
                    day @ stringify!($day) => {
                        // NOTE: u8 is fine since this is only every for the days until Christmas :^)
                        let day_number: u8 = day.strip_prefix("day")
                            .expect("identifier was not prefixed with \"day\"")
                            .parse()
                            .expect("identifier was not in the proper format \"day<x>\"");

                        // TODO: Allow user to input which year to run.
                        let input = crate::input::Input::fetch(crate::date::ChallengeDate { year: $year, day: day_number });
                        match part {
                            1 => println!("Part #1: {}", $day::part1(input)),
                            2 => println!("Part #2: {}", $day::part2(input)),
                            _ => panic!("Invalid part number"),
                        };
                    },
                )*
                other => panic!("{} has not been implemented", other),
            }
        }
    };
}

/// Generate module definitions for each year and define `run` to dispatch to
/// the right one.
///
/// Usage:
/// ```rust
/// years!(year_2022, year_2023, ...);
/// ```
macro_rules! years {
    ($($year:ident), *) => {
        $( mod $year; )*

        pub fn run(year: &str, day: &str, part: u8) {
            match year {
                $(
                    stringify!($year) => $year::run(day, part),
                )*
                other => panic!("{} has not been implemented", other),
            }
        }
    }
}

pub(crate) use {days, years};
