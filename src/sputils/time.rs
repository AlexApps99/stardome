// https://www.iausofa.org/2020_0721_C/sofa/sofa_ts_c.pdf
use sofa_sys::*;

#[derive(Debug, Clone, Copy)]
pub struct UTCDate {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub day_frac: f64,
}

impl UTCDate {
    pub fn from_ymdhms(year: i32, month: i32, day: i32, h: i32, m: i32, s: f64) -> Self {
        Self {
            year,
            month,
            day,
            day_frac: unsafe {
                let mut d: f64 = 0.0;
                // Range check ignored for convenience
                // It will calculate despite it
                iauTf2d(0, h, m, s, &mut d);
                d
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UTC(f64, f64);

impl UTC {
    pub fn from_date(d: &UTCDate) -> Option<Self> {
        unsafe {
            let mut djmjd0: f64 = 0.0;
            let mut date: f64 = 0.0;

            if iauCal2jd(d.year, d.month, d.day, &mut djmjd0, &mut date) < 0 {
                None
            } else {
                Some(Self(djmjd0, date + d.day_frac))
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TAI(f64, f64);

#[derive(Debug, Clone, Copy)]
pub struct TT(f64, f64);

#[derive(Debug, Clone, Copy)]
pub struct UT1(f64, f64);

#[derive(Debug, Clone, Copy)]
pub struct TCG(f64, f64);

#[derive(Debug, Clone, Copy)]
pub struct TCB(f64, f64);

#[derive(Debug, Clone, Copy)]
pub struct TDB(f64, f64);

// There are a total of 16 functions for conversions between time formats.
// If I have the time, I may implement them all later.

impl From<UTC> for TAI {
    fn from(utc: UTC) -> Self {
        unsafe {
            let mut a: f64 = 0.0;
            let mut b: f64 = 0.0;
            if iauUtctai(utc.0, utc.1, &mut a, &mut b) < 0 {
                Self(0.0, 0.0) // Error has occured
            } else {
                Self(a, b)
            }
        }
    }
}

impl From<TAI> for TT {
    fn from(tai: TAI) -> Self {
        unsafe {
            let mut a: f64 = 0.0;
            let mut b: f64 = 0.0;
            iauTaitt(tai.0, tai.1, &mut a, &mut b);
            Self(a, b)
        }
    }
}

// TODO hard-coded DUT1 is NOT smart
impl From<UTC> for UT1 {
    fn from(utc: UTC) -> Self {
        unsafe {
            let mut a: f64 = 0.0;
            let mut b: f64 = 0.0;
            if iauUtcut1(utc.0, utc.1, -0.2, &mut a, &mut b) < 0 {
                Self(0.0, 0.0) // Error has occured
            } else {
                Self(a, b)
            }
        }
    }
}
