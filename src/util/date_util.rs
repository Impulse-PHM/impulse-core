//! Utility functions when dealing with dates

use time::{Date, OffsetDateTime};


/// Calculate and return the age for a user based on UTC rather than a specific time zone
/// 
/// For those born on February 29th, this function considers March 1st to be their 
/// birthday on a regular/non-leap year. Otherwise, the birthday is considered as February 29th 
/// on a leap year.
/// 
/// Parameters:
/// `date_of_birth`: a date of birth
/// 
pub fn get_age(date_of_birth: &Date) -> i32 {
    let today = OffsetDateTime::now_utc();
    let mut age = today.year() - date_of_birth.year();

    // Subtract 1 if the user's birthday hasn't yet occurred
    if (today.month(), today.day()) < (date_of_birth.month(), date_of_birth.day()) {
        age -= 1;
    }

    age
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::date;

    
    #[test]
    fn get_age() {
        let tony_stark_birth_date = date!(1970-05-29);

        let now = OffsetDateTime::now_utc();
        let mut expected_age = now.year() - tony_stark_birth_date.year();

        if (now.month(), now.day()) < (tony_stark_birth_date.month(), tony_stark_birth_date.day()) {
            // The user's birthday has not occurred yet, so subtract 1
            expected_age -= 1;
        }

        assert_eq!(super::get_age(&tony_stark_birth_date), expected_age);
    }
}
