pub trait Hhmmss {
    fn s_ms(&self) -> (i64, i64);
    /// Pretty-prints a chrono::Duration in the form `HH:MM:SS.xxx`
    fn hhmmss(&self) -> String {
        let (s, _ms) = self.s_ms();
        s2hhmmss(s)
    }
    /// Pretty-prints a chrono::Duration in the form `HH:MM:SS.xxx`
    fn hhmmssxxx(&self) -> String {
        let (s, ms) = self.s_ms();
        sms2hhmmsxxx(s, ms)
    }
    /// Pretty-prints a chrono::Duration in the form `MM:SS`
    fn mmss(&self) -> String {
        let (s, _ms) = self.s_ms();
        s2mmss(s)
    }
    /// Pretty-prints a chrono::Duration in the form `MM:SS.xxx`
    fn mmssxxx(&self) -> String {
        let (s, ms) = self.s_ms();
        sms2mmsxxx(s, ms)
    }
}

impl Hhmmss for chrono::Duration {
    fn s_ms(&self) -> (i64, i64) {
        let s = self.num_seconds();
        let ms = self.num_milliseconds() - 1000 * s;
        (s, ms)
    }
}

impl Hhmmss for std::time::Duration {
    fn s_ms(&self) -> (i64, i64) {
        let s = self.as_secs();
        let ms = self.subsec_millis();
        (s as i64, ms as i64)
    }
}

impl Hhmmss for time::Duration {
    fn s_ms(&self) -> (i64, i64) {
        let s = self.whole_seconds();
        let ms = self.whole_milliseconds() as i64 - 1000 * s;
        (s, ms)
    }
}

fn s2hhmmss(s: i64) -> String {
    let mut neg = false;
    let mut s = s;
    if s < 0 {
        neg = true;
        s = -s;
    }
    let (h, s) = (s / 3600, s % 3600);
    let (m, s) = (s / 60, s % 60);
    format!("{}{:02}:{:02}:{:02}", if neg { "-" } else { "" }, h, m, s)
}

fn sms2hhmmsxxx(s: i64, ms: i64) -> String {
    let mut neg = false;
    let (mut s, mut ms) = (s, ms);
    if s < 0 {
        neg = true;
        s = -s;
        ms = -ms;
    }
    let (h, s) = (s / 3600, s % 3600);
    let (m, s) = (s / 60, s % 60);
    format!(
        "{}{:02}:{:02}:{:02}.{:03}",
        if neg { "-" } else { "" },
        h,
        m,
        s,
        ms
    )
}

fn s2mmss(s: i64) -> String {
    let mut neg = false;
    let mut s = s;
    if s < 0 {
        neg = true;
        s = -s;
    }
    let (_h, s) = (s / 3600, s % 3600);
    let (m, s) = (s / 60, s % 60);
    format!("{}{:02}:{:02}", if neg { "-" } else { "" }, m, s)
}

fn sms2mmsxxx(s: i64, ms: i64) -> String {
    let mut neg = false;
    let (mut s, mut ms) = (s, ms);
    if s < 0 {
        neg = true;
        s = -s;
        ms = -ms;
    }
    let (_h, s) = (s / 3600, s % 3600);
    let (m, s) = (s / 60, s % 60);
    format!(
        "{}{:02}:{:02}.{:03}",
        if neg { "-" } else { "" },
        m,
        s,
        ms
    )
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all() {
        let std_duration = std::time::Duration::new(3661, 534_000_000);
				assert_eq!(&std_duration.hhmmss(), "01:01:01");
				assert_eq!(&std_duration.hhmmssxxx(), "01:01:01.534");
				assert_eq!(&std_duration.mmss(), "01:01");
				assert_eq!(&std_duration.mmssxxx(), "01:01.534");
        let chrono_duration = chrono::Duration::from_std(std_duration).unwrap();
				assert_eq!(&chrono_duration.hhmmss(), "01:01:01");
				assert_eq!(&chrono_duration.hhmmssxxx(), "01:01:01.534");
				assert_eq!(&chrono_duration.mmss(), "01:01");
				assert_eq!(&chrono_duration.mmssxxx(), "01:01.534");
        let time_duration = time::Duration::from_std(std_duration).unwrap();
				assert_eq!(&time_duration.hhmmss(), "01:01:01");
				assert_eq!(&time_duration.hhmmssxxx(), "01:01:01.534");
				assert_eq!(&time_duration.mmss(), "01:01");
				assert_eq!(&time_duration.mmssxxx(), "01:01.534");
    }
}
