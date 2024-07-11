pub fn date_to_day_of_the_week(day: i32, month: i32, year: i32) -> i32 {
    // Convert a date of the form dd/mm/yy to the corresponding day of
    // the week where 1 = Monday and 7 = Sunday. The algorithm for doing so
    // is based on Zeller's congruence for Gregorian calendars. More information
    // for this can be found on https://en.wikipedia.org/wiki/Zeller%27s_congruence

    // Make January and February values 13 and 14 instead of 1 and 2, note that they are
    // counted as the 13th and 14th month of the previous year!
    let mut new_month = month;
    let mut new_year = year;
    if month == 1 || month == 2 {
        new_month += 12;
        new_year -= 1;
    }

    let year_of_century = new_year % 100;
    let zero_based_century = new_year / 100;

    let mut day_of_the_week = (day
        + (13 * (new_month + 1) / 5)
        + year_of_century
        + year_of_century / 4
        + zero_based_century / 4
        - 2 * zero_based_century)
        % 7;

    // Shift so a value of 1 is Monday up to value of 7 for Sunday.
    // Zeller's congruence results 0 for Saturday up to 6 for Friday.
    day_of_the_week = ((day_of_the_week + 5) % 7) + 1;

    return day_of_the_week;
}

pub fn is_leap_year(year: i32) -> i32 {
    // Checks if given year is a leap year. A leap year is defined as a year
    // that is divisible by 4, but if the year is divisible by 100 it is not a
    // leap year unless it is divisible by 400. For reference see,
    // https://en.wikipedia.org/wiki/Leap_year

    if year % 4 == 0 {
        // If divisible by four and 100, additional check is needed.
        if year % 100 == 0 {
            // If the year is also divisble by 400 it is again a leap year.
            if year % 400 == 0 {
                return 1;
            }
            // However, if it is not divisible by 400, then divisibility by 100
            // overrules the divisble by 4 rule and the year is not a leap year.
            else {
                return 0;
            }
        }
        // If not divisible by 100, this is definitely a leap year.
        else {
            return 1;
        }
    }
    // If not divisible by 4, it is not a leap year.
    else {
        return 0;
    }
}
pub fn month_to_days_in_month(month: i32, year: i32) -> i32 {
    // Given a month, return the amount of days in that month.
    // We also require the year as the amount of days in February depends
    // on whether the year is a leap year or not.

    let leap_year = is_leap_year(year);

    // Check if the month is February
    if month == 2 {
        // 28 is no leap year and 29 if leap year
        return 28 + leap_year;
    } else {
        // (month - 1) % 7 % 2 return 0,1,0,1,... to modify the correct months,
        // see http://www.dispersiondesign.com/articles/time/number_of_days_in_a_month
        return 31 - ((month - 1) % 7) % 2;
    }
}
