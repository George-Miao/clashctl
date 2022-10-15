use std::fmt::Write;

pub trait HMS {
    fn as_second(&self) -> i64;
    fn hms(&self) -> String {
        let mut s = self.as_second();
        let mut neg = false;
        let mut written = false;
        if s < 0 {
            neg = true;
            s = -s;
        }
        let (h, s) = (s / 3600, s % 3600);
        let (m, s) = (s / 60, s % 60);
        let mut ret = String::with_capacity(10);
        if neg {
            written = true;
            ret.push('-')
        };
        if written || h > 0 {
            written = true;
            write!(ret, "{}h ", h).expect("Cannot write to buf")
        }
        if written || m > 0 {
            write!(ret, "{}m ", m).expect("Cannot write to buf")
        }
        write!(ret, "{}s", s).expect("Cannot write to buf");
        ret
    }
}

impl HMS for chrono::Duration {
    fn as_second(&self) -> i64 {
        self.num_seconds()
    }
}

impl HMS for std::time::Duration {
    fn as_second(&self) -> i64 {
        self.as_secs().try_into().expect("Seconds to big")
    }
}
