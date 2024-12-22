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
	/// Pretty-print chrono::Duration in the format
	/// `H:MM:SS` if the hour value is one digit,
	/// otherwise in the format `HH:MM:SS`.
	fn hmmss(&self) -> String {
		let (s, _ms) = self.s_ms();
		s2hhmmss(s)
	}
	/// Pretty-print chrono::Duration in the format
	/// `H:MM:SS.xxx` if the hour value is one digit,
	/// otherwise in the format `HH:MM:SS.xxx`.
	fn hmmssxxx(&self) -> String {
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
	/// Pretty-print chrono::Duration in the format
	/// `M:SS` if the minute value is one digit,
	/// otherwise in the format `MM:SS`.
	fn mss(&self) -> String {
		let (s, _ms) = self.s_ms();
		s2mmss(s)
	}
	/// Pretty-print chrono::Duration in the format
	/// `M:SS.xxx` if the minute value is one digit,
	/// otherwise in the format `MM:SS.xxx`.
	fn mssxxx(&self) -> String {
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
	let (neg, _h, m, s, ms) = s2tuple(s, ms);
	format!("{}{:02}:{:02}.{:03}", if neg { "-" } else { "" }, m, s, ms)
}

fn s2tuple(s: i64, ms: i64) -> (bool, i64, i64, i64, i64) {
	let mut neg = false;
	let (mut s, mut ms) = (s, ms);
	if s < 0 {
		neg = true;
		s = -s;
		ms = -ms;
	}
	let (h, s) = (s / 3600, s % 3600);
	let (m, s) = (s / 60, s % 60);
	(neg, h, m, s, ms)
}
#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn test_all() {
		let std_duration = std::time::Duration::new((1 * 60 + 23) * 60 + 45, 678_901_234);
		assert_eq!(&std_duration.hhmmss(), "01:23:45");
		assert_eq!(&std_duration.hhmmssxxx(), "01:23:45.678");
		assert_eq!(&std_duration.mmss(), "23:45");
		assert_eq!(&std_duration.mmssxxx(), "23:45.678");
		let chrono_duration = chrono::Duration::from_std(std_duration).unwrap();
		assert_eq!(&chrono_duration.hhmmss(), "01:23:45");
		assert_eq!(&chrono_duration.hhmmssxxx(), "01:23:45.678");
		assert_eq!(&chrono_duration.mmss(), "23:45");
		assert_eq!(&chrono_duration.mmssxxx(), "23:45.678");
		let time_duration = time::Duration::from_std(std_duration).unwrap();
		assert_eq!(&time_duration.hhmmss(), "01:23:45");
		assert_eq!(&time_duration.hhmmssxxx(), "01:23:45.678");
		assert_eq!(&time_duration.mmss(), "23:45");
		assert_eq!(&time_duration.mmssxxx(), "23:45.678");
	}
}
