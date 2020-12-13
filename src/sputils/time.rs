// https://www.iausofa.org/2020_0721_C/sofa/sofa_ts_c.pdf
use sofa_sys::*;
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy)]
pub enum TimeError {
    UnacceptableDate,
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

    #[allow(non_snake_case)]
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

    pub fn from_system_time(time: std::time::SystemTime) -> Option<Self> {
        let t = if let Ok(t) = time.duration_since(std::time::SystemTime::UNIX_EPOCH) {
            t.as_secs_f64()
        } else if let Ok(t) = std::time::SystemTime::UNIX_EPOCH.duration_since(time) {
            -t.as_secs_f64()
        } else {
            return None; // Unreachable, as far as I know
        } / DAYSEC;

        Some(UTC(240000.5, t + 40587.0))
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

impl UT1 {
    pub fn from_tai(tai: TAI, dta: f64) -> Self {
        unsafe {
            let mut a: f64 = 0.0;
            let mut b: f64 = 0.0;
            iauTaiut1(tai.0, tai.1, dta, &mut a, &mut b);
            Self(a, b)
        }
    }
}

impl TAI {
    #[inline(always)]
    pub fn into_ut1(self, dta: f64) -> UT1 {
        UT1::from_tai(self, dta)
    }
}

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

impl TT {
    pub fn from_tdb(tdb: TDB, dtr: f64) -> Self {
        unsafe {
            let mut a: f64 = 0.0;
            let mut b: f64 = 0.0;
            iauTdbtt(tdb.0, tdb.1, dtr, &mut a, &mut b);
            Self(a, b)
        }
    }
}

impl TDB {
    #[inline(always)]
    pub fn into_tt(self, dtr: f64) -> TT {
        TT::from_tdb(self, dtr)
    }
}

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

impl TDB {
    pub fn from_tt(tt: TT, dtr: f64) -> Self {
        unsafe {
            let mut a: f64 = 0.0;
            let mut b: f64 = 0.0;
            iauTttdb(tt.0, tt.1, dtr, &mut a, &mut b);
            Self(a, b)
        }
    }
}

impl TT {
    #[inline(always)]
    pub fn into_tdb(self, dtr: f64) -> TDB {
        TDB::from_tt(self, dtr)
    }
}

impl UT1 {
    pub fn from_tt(tt: TT, dt: f64) -> Self {
        unsafe {
            let mut a: f64 = 0.0;
            let mut b: f64 = 0.0;
            iauTtut1(tt.0, tt.1, dt, &mut a, &mut b);
            Self(a, b)
        }
    }
}

impl TT {
    #[inline(always)]
    pub fn into_ut1(self, dt: f64) -> UT1 {
        UT1::from_tt(self, dt)
    }
}

impl TAI {
    pub fn from_ut1(ut1: UT1, dta: f64) -> Self {
        unsafe {
            let mut a: f64 = 0.0;
            let mut b: f64 = 0.0;
            iauUt1tai(ut1.0, ut1.1, dta, &mut a, &mut b);
            Self(a, b)
        }
    }
}

impl UT1 {
    #[inline(always)]
    pub fn into_tai(self, dta: f64) -> TAI {
        TAI::from_ut1(self, dta)
    }
}

impl TT {
    pub fn from_ut1(ut1: UT1, dt: f64) -> Self {
        unsafe {
            let mut a: f64 = 0.0;
            let mut b: f64 = 0.0;
            iauUt1tt(ut1.0, ut1.1, dt, &mut a, &mut b);
            Self(a, b)
        }
    }
}

impl UT1 {
    #[inline(always)]
    pub fn into_tt(self, dt: f64) -> TT {
        TT::from_ut1(self, dt)
    }
}

impl UTC {
    pub fn try_from_ut1(ut1: UT1, dut1: f64) -> Result<Self, TimeError> {
        unsafe {
            let mut a: f64 = 0.0;
            let mut b: f64 = 0.0;
            if iauUt1utc(ut1.0, ut1.1, dut1, &mut a, &mut b) < 0 {
                Err(TimeError::UnacceptableDate)
            } else {
                Ok(Self(a, b))
            }
        }
    }
}

impl UT1 {
    #[inline(always)]
    pub fn try_into_utc(self, dut1: f64) -> Result<UTC, TimeError> {
        UTC::try_from_ut1(self, dut1)
    }
}

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

impl UT1 {
    pub fn try_from_utc(utc: UTC, dut1: f64) -> Result<Self, TimeError> {
        unsafe {
            let mut a: f64 = 0.0;
            let mut b: f64 = 0.0;
            if iauUtcut1(utc.0, utc.1, dut1, &mut a, &mut b) < 0 {
                Err(TimeError::UnacceptableDate)
            } else {
                Ok(Self(a, b))
            }
        }
    }
}

impl UTC {
    #[inline(always)]
    pub fn try_into_ut1(self, dut1: f64) -> Result<UT1, TimeError> {
        UT1::try_from_utc(self, dut1)
    }
}
