use chrono::prelude::*;
use std::fmt;

const NUMBER_OF_DAYS_IN_A_WEEK: u32 = 7;
const NUMBER_OF_WEEKS_IN_A_QUARTER: u32 = 13;
const NUMBER_OF_QUARTERS_IN_A_YEAR: u32 = 4;
const NUMBER_OF_DAYS_IN_A_QUARTER: u32 = NUMBER_OF_DAYS_IN_A_WEEK * NUMBER_OF_WEEKS_IN_A_QUARTER;
const NUMBER_OF_DAYS_IN_A_YEAR: u32 = NUMBER_OF_DAYS_IN_A_QUARTER * NUMBER_OF_QUARTERS_IN_A_YEAR;

const ORDINAL_FOR_FIRST_DAY_OF_YEAR: u32 = 0;
const ORDINAL_FOR_LAST_DAY_OF_YEAR: u32 = ORDINAL_FOR_FIRST_DAY_OF_YEAR + NUMBER_OF_DAYS_IN_A_YEAR - 1;
const ORDINAL_FOR_NEW_YEARS_DAY: u32 = ORDINAL_FOR_LAST_DAY_OF_YEAR + 1;
const ORDINAL_FOR_LEAP_DAY: u32 = ORDINAL_FOR_NEW_YEARS_DAY + 1;

const NUMBER_OF_DAYS_FROM_NEW_YEARS_EVE_TO_SPRING_EQUINOX_IN_THE_GREGORIAN_CALENDAR: i32 = 78;

#[derive(Debug)]
pub struct DateClock<D>(D);

#[derive(Debug, PartialEq, Eq)]
pub enum Time {
    Regular {
        month: u32,
        week: u32,
        day: u32,
    },
    NewYears,
    LeapDay,
}

impl<D: Datelike> DateClock<D> {
    fn ordinal0(&self) -> u32 {
        // We define New Year's Day as the day before the average Spring Equinox (May 20th).
        // We define Leap Day as the day after New Year's day on leap years.
        // We implement this by treating a date_clock date as being a fixed offset from a Gregorian date.
        NaiveDate::from_num_days_from_ce(
            self.0.num_days_from_ce() - NUMBER_OF_DAYS_FROM_NEW_YEARS_EVE_TO_SPRING_EQUINOX_IN_THE_GREGORIAN_CALENDAR
        ).ordinal0()
    }

    pub fn time(&self) -> Time {
        let ordinal = self.ordinal0();

        match ordinal {
            ORDINAL_FOR_FIRST_DAY_OF_YEAR..=ORDINAL_FOR_LAST_DAY_OF_YEAR => Time::Regular {
                month: ordinal / NUMBER_OF_DAYS_IN_A_QUARTER,
                week: ordinal / NUMBER_OF_DAYS_IN_A_WEEK % NUMBER_OF_WEEKS_IN_A_QUARTER,
                day: ordinal % NUMBER_OF_DAYS_IN_A_WEEK,
            },
            ORDINAL_FOR_NEW_YEARS_DAY => Time::NewYears,
            ORDINAL_FOR_LEAP_DAY => Time::LeapDay,
            _ => unreachable!("out of range ordinal: `{ordinal}`"),
        }
    }
}

impl<D: Datelike> PartialEq<Time> for DateClock<D> {
    fn eq(&self, other: &Time) -> bool { self.time() == *other }
}

impl<D: Datelike> fmt::Display for DateClock<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.time() {
            Time::Regular { month, week, day } => write!(f, "{month}/{week}/{day}"),
            Time::NewYears => write!(f, "New Year's Day"),
            Time::LeapDay => write!(f, "Leap Day"),
        }
    }
}

impl DateClock<Date<Local>> {
    pub fn today() -> Self { Self(Local::today()) }
}

#[cfg(test)]
mod tests {
    use chrono::*;
    use super::*;

    // Leap days occur on dates in the year following a leap year because date clock puts the leap day at the end of
    // the year.
    // Furthermore, this implies that the start of the year is a day earlier on leap years.

    #[test]
    fn spring_equinox_is_the_first_regular_day_of_the_year() {
        assert_eq!(DateClock(NaiveDate::from_ymd(2022, 3, 20)), Time::Regular { month: 0, week: 0, day: 0 });
        assert_eq!(DateClock(NaiveDate::from_ymd(2023, 3, 20)), Time::Regular { month: 0, week: 0, day: 0 });
        assert_eq!(DateClock(NaiveDate::from_ymd(2024, 3, 19)), Time::Regular { month: 0, week: 0, day: 0 });
        assert_eq!(DateClock(NaiveDate::from_ymd(2025, 3, 20)), Time::Regular { month: 0, week: 0, day: 0 });
    }

    #[test]
    fn leap_day_is_the_day_after_new_years_day() {
        assert_eq!(DateClock(NaiveDate::from_ymd(2025, 3, 18)), Time::NewYears);
        assert_eq!(DateClock(NaiveDate::from_ymd(2025, 3, 19)).time(), Time::LeapDay);
    }
}