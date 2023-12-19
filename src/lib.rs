use enum_iterator::Sequence;

#[derive(Debug, PartialEq, Sequence, Clone, Copy)]
pub enum Month {
    JAN,
    FEB,
    MAR,
    APR,
    MAY,
    JUN,
    JUL,
    AUG,
    SEP,
    OCT,
    NOV,
    DEC,
}

impl Month {
    fn as_num(month: Month) -> u8 {
        match month {
            Month::JAN => 1,
            Month::FEB => 2,
            Month::MAR => 3,
            Month::APR => 4,
            Month::MAY => 5,
            Month::JUN => 6,
            Month::JUL => 7,
            Month::AUG => 8,
            Month::SEP => 9,
            Month::OCT => 10,
            Month::NOV => 11,
            Month::DEC => 12,
        }
    }
}

#[derive(Debug, PartialEq, Sequence, Clone, Copy)]
pub struct Date {
    pub day: u8,
    pub month: Month,
    pub year: u16,
}

impl Date {
    pub fn get_date_string(self) -> String {
        format!(
            "{} / {} / {}",
            self.day,
            Month::as_num(self.month),
            self.year
        )
    }

    pub fn is_leap_year(self) -> bool {
        if self.year % 400 == 0 {
            true
        } else if self.year % 4 == 0 && self.year % 100 != 0 {
            true
        } else {
            false
        }
    }
}

pub fn is_leap_year(y: u16) -> bool {
    if y % 400 == 0 {
        true
    } else if y % 4 == 0 && y % 100 != 0 {
        true
    } else {
        false
    }
}

pub fn get_max_day(m: &Month, y: u16) -> u8 {
    match m {
        Month::FEB => {
            if is_leap_year(y) {
                29
            } else {
                28
            }
        }
        Month::JAN
        | Month::MAR
        | Month::MAY
        | Month::JUL
        | Month::AUG
        | Month::OCT
        | Month::DEC => 31,
        _ => 30,
    }
}

pub fn fill_date_vec(vec: &mut Vec<Date>, y: u16) {
    for m in enum_iterator::all::<Month>().collect::<Vec<_>>() {
        for d in 1..=get_max_day(&m, y) {
            vec.push(Date {
                day: d,
                month: m,
                year: y,
            })
        }
    }
}
