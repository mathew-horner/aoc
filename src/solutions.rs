use crate::date::ChallengeDate;
use crate::input::Input;

/// Generate module definitions for each day's solution and define `run` to dispatch to the right one.
///
/// Usage:
/// ```rust
/// solutions!(day1, day2, day3, ...);
/// ```
macro_rules! solutions {
    ($($day:ident), *) => {
        $( mod $day; )*

        pub fn run(day: &str) {
            match day {
                $(
                    day @ stringify!($day) => {
                        // NOTE: u8 is fine since this is only every for the days until Christmas :^)
                        let day_number: u8 = day.strip_prefix("day")
                            .expect("identifier was not prefixed with \"day\"")
                            .parse()
                            .expect("identifier was not in the proper format \"day<x>\"");

                        // TODO: Allow user to input which year to run.
                        let input = Input::fetch(ChallengeDate { day: day_number, year: 2022 });
                        $day::solve(input);
                    },
                )*
                other => panic!("{} has not been implemented", other),
            }
        }
    };
}

solutions!(day1, day2, day3, day6);
