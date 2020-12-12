// https://www.iausofa.org/2020_0721_C/sofa/sofa_ts_c.pdf
use sofa_sys::*;
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy)]
pub enum TimeError {
    UnacceptableDate
}

impl std::error::Error for TimeError {}
impl std::fmt::Display for TimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Unacceptable Date")
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UTC(f64, f64);

impl UTC {
    pub fn from_ymdf(y: i32, m: i32, d: i32, f: f64) -> Option<Self> {
        unsafe {
            let mut djmjd0: f64 = 0.0;
            let mut date: f64 = 0.0;

            if iauCal2jd(y, m, d, &mut djmjd0, &mut date) < 0 {
                None
            } else {
                Some(Self(djmjd0, date + f))
            }
        }
    }

    pub fn from_ymdhms(y: i32, m: i32, d: i32, H: i32, M: i32, S: f64) -> Option<Self> {
        Self::from_ymdf(y, m, d, unsafe {
            let mut d: f64 = 0.0;
            if iauTf2d(0, H, M, S, &mut d) != 0 {
                return None;
            } else {
                d
            }
        })
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

//impl From<TAI> for UT1 {
//    fn from(tai: TAI) -> Self {
//        unsafe {
//            let mut a: f64 = 0.0;
//            let mut b: f64 = 0.0;
//            iauTaiut1(tai.0, tai.1, dta, &mut a, &mut b);
//            Self(a, b)
//        }
//    }
//}

impl TryFrom<TAI> for UTC {
    type Error = TimeError;
    fn try_from(tai: TAI) -> Result<Self, Self::Error> {
        unsafe {
            let mut a: f64 = 0.0;
            let mut b: f64 = 0.0;
            if iauTaiutc(tai.0, tai.1, &mut a, &mut b) < 0 {
                Err(TimeError::UnacceptableDate)
            } else {
                Ok(Self(a, b))
            }
        }
    }
}

impl From<TCB> for TDB {
    fn from(tcb: TCB) -> Self {
        unsafe {
            let mut a: f64 = 0.0;
            let mut b: f64 = 0.0;
            iauTcbtdb(tcb.0, tcb.1, &mut a, &mut b);
            Self(a, b)
        }
    }
}

impl From<TCG> for TT {
    fn from(tcg: TCG) -> Self {
        unsafe {
            let mut a: f64 = 0.0;
            let mut b: f64 = 0.0;
            iauTcgtt(tcg.0, tcg.1, &mut a, &mut b);
            Self(a, b)
        }
    }
}

impl From<TDB> for TCB {
    fn from(tdb: TDB) -> Self {
        unsafe {
            let mut a: f64 = 0.0;
            let mut b: f64 = 0.0;
            iauTdbtcb(tdb.0, tdb.1, &mut a, &mut b);
            Self(a, b)
        }
    }
}

//impl From<TDB> for TT {
//    fn from(tdb: TDB) -> Self {
//        unsafe {
//            let mut a: f64 = 0.0;
//            let mut b: f64 = 0.0;
//            iauTdbtt(tdb.0, tdb.1, dtr, &mut a, &mut b);
//            Self(a, b)
//        }
//    }
//}

impl From<TT> for TAI {
    fn from(tt: TT) -> Self {
        unsafe {
            let mut a: f64 = 0.0;
            let mut b: f64 = 0.0;
            iauTttai(tt.0, tt.1, &mut a, &mut b);
            Self(a, b)
        }
    }
}

impl From<TT> for TCG {
    fn from(tt: TT) -> Self {
        unsafe {
            let mut a: f64 = 0.0;
            let mut b: f64 = 0.0;
            iauTttcg(tt.0, tt.1, &mut a, &mut b);
            Self(a, b)
        }
    }
}

//impl From<TT> for TDB {
//    fn from(tt: TT) -> Self {
//        unsafe {
//            let mut a: f64 = 0.0;
//            let mut b: f64 = 0.0;
//            iauTttdb(tt.0, tt.1, dtr, &mut a, &mut b);
//            Self(a, b)
//        }
//    }
//}

//impl From<TT> for UT1 {
//    fn from(tt: TT) -> Self {
//        unsafe {
//            let mut a: f64 = 0.0;
//            let mut b: f64 = 0.0;
//            iauTtut1(tt.0, tt.1, dt, &mut a, &mut b);
//            Self(a, b)
//        }
//    }
//}

//impl From<UT1> for TAI {
//    fn from(ut1: UT1) -> Self {
//        unsafe {
//            let mut a: f64 = 0.0;
//            let mut b: f64 = 0.0;
//            iauUt1tai(ut1.0, ut1.1, dta, &mut a, &mut b);
//            Self(a, b)
//        }
//    }
//}

//impl From<UT1> for TT {
//    fn from(ut1: UT1) -> Self {
//        unsafe {
//            let mut a: f64 = 0.0;
//            let mut b: f64 = 0.0;
//            iauUt1tt(ut1.0, ut1.1, dt, &mut a, &mut b);
//            Self(a, b)
//        }
//    }
//}

//impl From<UT1> for UTC {
//    fn from(ut1: UT1) -> Self {
//        unsafe {
//            let mut a: f64 = 0.0;
//            let mut b: f64 = 0.0;
//            if iauUt1utc(ut1.0, ut1.1, dut1, &mut a, &mut b) < 0 {
//                Self(0.0, 0.0) // Error has occured
//            } else {
//                Self(a, b)
//            }
//        }
//    }
//}

impl TryFrom<UTC> for TAI {
    type Error = TimeError;
    fn try_from(utc: UTC) -> Result<Self, Self::Error> {
        unsafe {
            let mut a: f64 = 0.0;
            let mut b: f64 = 0.0;
            if iauUtctai(utc.0, utc.1, &mut a, &mut b) < 0 {
                Err(TimeError::UnacceptableDate)
            } else {
                Ok(Self(a, b))
            }
        }
    }
}

//impl From<UTC> for UT1 {
//    fn from(utc: UTC) -> Self {
//        unsafe {
//            let mut a: f64 = 0.0;
//            let mut b: f64 = 0.0;
//            if iauUtcut1(utc.0, utc.1, dut1, &mut a, &mut b) < 0 {
//                Self(0.0, 0.0) // Error has occured
//            } else {
//                Self(a, b)
//            }
//        }
//    }
//}
