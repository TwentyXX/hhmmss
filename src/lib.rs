pub trait Hhmmss {
	fn sign(&self) -> String {
		if self.is_negative() {
			"-".to_owned()
		} else {
			"".to_owned()
		}
	}
	fn is_negative(&self) -> bool;
	fn unsigned_hours(&self) -> u64;
	fn unsigned_minutes(&self) -> u64;
	fn unsigned_seconds(&self) -> u64;
	fn unsigned_milliseconds(&self) -> u64;
	fn unsigned_microseconds(&self) -> u64;
	fn unsigned_nanoseconds(&self) -> u64;
	fn get_hours(&self) -> i64;
	fn get_minutes(&self) -> i64;
	fn get_seconds(&self) -> i64;
	fn get_milliseconds(&self) -> i64;
	fn get_microseconds(&self) -> i64;
	fn get_nanoseconds(&self) -> i64;

	fn unsigned_hh(&self) -> String { format!("{:02}", self.unsigned_hours()) }
	fn unsigned_mm(&self) -> String { format!("{:02}", self.unsigned_minutes()) }
	fn unsigned_ss(&self) -> String { format!("{:02}", self.unsigned_seconds()) }
	fn unsigned_xxx(&self) -> String { format!("{:03}", self.unsigned_milliseconds()) }

	fn fmt_hh(&self) -> String { self.sign() + &self.unsigned_hh() }
	fn fmt_mm(&self) -> String { self.sign() + &self.unsigned_mm() }
	fn fmt_ss(&self) -> String { self.sign() + &self.unsigned_ss() }
	fn fmt_xxx(&self) -> String { self.sign() + &self.unsigned_xxx() }

	/// Pretty-prints a chrono::Duration in the form `MM:SS`
	fn unsigned_mmss(&self) -> String { format!("{}:{}", self.unsigned_mm(), self.unsigned_ss()) }
	/// Pretty-prints a chrono::Duration in the form `MM:SS.xxx`
	fn unsigned_mmssxxx(&self) -> String {
		format!(
			"{}:{}.{}",
			self.unsigned_hh(),
			self.unsigned_ss(),
			self.unsigned_xxx()
		)
	}
	/// Pretty-print chrono::Duration in the format
	/// `M:SS` if the minute value is one digit,
	/// otherwise in the format `MM:SS`.
	fn unsigned_mss(&self) -> String {
		format!("{}:{}", self.unsigned_minutes(), self.unsigned_ss())
	}
	/// Pretty-print chrono::Duration in the format
	/// `M:SS.xxx` if the minute value is one digit,
	/// otherwise in the format `MM:SS.xxx`.
	fn unsigned_mssxxx(&self) -> String {
		format!(
			"{}:{}.{}",
			self.unsigned_minutes(),
			self.unsigned_ss(),
			self.unsigned_xxx(),
		)
	}
	/// Pretty-prints a chrono::Duration in the form `HH:MM:SS.xxx`
	fn unsigned_hhmmss(&self) -> String {
		format!(
			"{}:{}:{}",
			self.unsigned_hh(),
			self.unsigned_mm(),
			self.unsigned_ss(),
		)
	}

	/// Pretty-prints a chrono::Duration in the form `HH:MM:SS.xxx`
	fn unsigned_hhmmssxxx(&self) -> String {
		format!(
			"{}:{}:{}.{}",
			self.unsigned_hh(),
			self.unsigned_mm(),
			self.unsigned_ss(),
			self.unsigned_xxx()
		)
	}
	fn unsigned_hmmss(&self) -> String {
		format!(
			"{}:{}:{}",
			self.unsigned_hours(),
			self.unsigned_mm(),
			self.unsigned_ss()
		)
	}
	fn unsigned_hmmssxxx(&self) -> String {
		format!(
			"{}:{}:{}.{}",
			self.unsigned_hours(),
			self.unsigned_mm(),
			self.unsigned_ss(),
			self.unsigned_xxx()
		)
	}
	fn mmss(&self) -> String { self.sign() + &self.unsigned_mmss() }
	fn mmssxxx(&self) -> String { self.sign() + &self.unsigned_mmssxxx() }
	fn mss(&self) -> String { self.sign() + &self.unsigned_mss() }
	fn mssxxx(&self) -> String { self.sign() + &self.unsigned_mssxxx() }
	fn hhmmss(&self) -> String { self.sign() + &self.unsigned_hhmmss() }
	fn hhmmssxxx(&self) -> String { self.sign() + &self.unsigned_hhmmssxxx() }
	fn hmmss(&self) -> String { self.sign() + &self.unsigned_hmmss() }
	fn hmmssxxx(&self) -> String { self.sign() + &self.unsigned_hmmssxxx() }
}

impl Hhmmss for chrono::Duration {
	fn get_hours(&self) -> i64 { self.num_hours() }

	fn get_minutes(&self) -> i64 { self.num_minutes() }

	fn get_seconds(&self) -> i64 { self.num_seconds() }

	fn get_milliseconds(&self) -> i64 { self.num_milliseconds() }

	fn get_microseconds(&self) -> i64 { self.num_microseconds().unwrap_or(0) }

	fn get_nanoseconds(&self) -> i64 { self.num_nanoseconds().unwrap_or(0) }

	fn unsigned_hours(&self) -> u64 { self.num_hours().unsigned_abs() }

	fn unsigned_minutes(&self) -> u64 { self.num_minutes().unsigned_abs() % 60 }

	fn unsigned_seconds(&self) -> u64 { self.num_seconds().unsigned_abs() % 60 }

	fn unsigned_milliseconds(&self) -> u64 { self.num_milliseconds().unsigned_abs() % 1000 }

	fn unsigned_microseconds(&self) -> u64 {
		self.num_microseconds().unwrap_or(0).unsigned_abs() % 1000
	}

	fn unsigned_nanoseconds(&self) -> u64 {
		self.num_nanoseconds().unwrap_or(0).unsigned_abs() % 1000
	}

	fn is_negative(&self) -> bool { self.num_seconds() < 0 }
}

impl Hhmmss for std::time::Duration {
	fn get_hours(&self) -> i64 { self.as_secs() as i64 / 3600 }

	fn get_minutes(&self) -> i64 { (self.as_secs() as i64 / 60) % 60 }

	fn get_seconds(&self) -> i64 { self.as_secs() as i64 % 60 }

	fn get_milliseconds(&self) -> i64 { self.subsec_millis() as i64 }

	fn get_microseconds(&self) -> i64 { self.subsec_micros() as i64 }

	fn get_nanoseconds(&self) -> i64 { self.subsec_nanos() as i64 }

	fn unsigned_hours(&self) -> u64 { self.as_secs() / 3600 }

	fn unsigned_minutes(&self) -> u64 { (self.as_secs() / 60) % 60 }

	fn unsigned_seconds(&self) -> u64 { self.as_secs() % 60 }

	fn unsigned_milliseconds(&self) -> u64 { self.subsec_millis() as u64 }

	fn unsigned_microseconds(&self) -> u64 { self.subsec_micros() as u64 }

	fn unsigned_nanoseconds(&self) -> u64 { self.subsec_nanos() as u64 }

	fn is_negative(&self) -> bool { false }

	fn sign(&self) -> String { "".to_owned() }
}

impl Hhmmss for time::Duration {
	fn get_hours(&self) -> i64 { self.whole_hours() }

	fn get_minutes(&self) -> i64 { self.whole_minutes() }

	fn get_seconds(&self) -> i64 { self.whole_seconds() }

	fn get_milliseconds(&self) -> i64 { self.subsec_milliseconds() as i64 }

	fn get_microseconds(&self) -> i64 { self.subsec_microseconds() as i64 }

	fn get_nanoseconds(&self) -> i64 { self.subsec_nanoseconds() as i64 }

	fn unsigned_hours(&self) -> u64 { (self.whole_hours().unsigned_abs()) }

	fn unsigned_minutes(&self) -> u64 { (self.whole_minutes().unsigned_abs() % 60) }

	fn unsigned_seconds(&self) -> u64 { (self.whole_seconds().unsigned_abs() % 60) }

	fn unsigned_milliseconds(&self) -> u64 {
		(self.subsec_milliseconds().unsigned_abs() as u64 % 1000)
	}

	fn unsigned_microseconds(&self) -> u64 {
		(self.subsec_microseconds().unsigned_abs() as u64 % 1000)
	}

	fn unsigned_nanoseconds(&self) -> u64 {
		(self.subsec_nanoseconds().unsigned_abs() as u64 % 1000)
	}

	fn is_negative(&self) -> bool { self.whole_seconds() < 0 }
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn test_all_crate_durations() {
		let std_duration = std::time::Duration::new((1 * 60 + 23) * 60 + 45, 678_901_234);
		assert_eq!(&std_duration.unsigned_hhmmss(), "01:23:45");
		assert_eq!(&std_duration.unsigned_hhmmssxxx(), "01:23:45.678");
		assert_eq!(&std_duration.unsigned_mmss(), "23:45");
		assert_eq!(&std_duration.unsigned_mmssxxx(), "23:45.678");
		assert_eq!(&std_duration.unsigned_hmmss(), "1:23:45");
		assert_eq!(&std_duration.unsigned_hmmssxxx(), "1:23:45.678");
		let chrono_duration = chrono::Duration::from_std(std_duration).unwrap();
		assert_eq!(&chrono_duration.unsigned_hhmmssxxx(), "01:23:45.678");
		let time_duration = time::Duration::from_std(std_duration).unwrap();
		assert_eq!(&time_duration.unsigned_hhmmssxxx(), "01:23:45.678");
	}

	#[test]
	fn test_negative_durations() {
		let minus_one_hour = chrono::Duration::hours(-1);
		dbg!(minus_one_hour);
		assert_eq!(&minus_one_hour.hmmss(), "-1:00:00");
		assert_eq!(&minus_one_hour.hhmmss(), "-01:00:00");
		assert_eq!(minus_one_hour.unsigned_hours(), 1);
		assert_eq!(minus_one_hour.get_hours(), -1);
		assert_eq!(&minus_one_hour.unsigned_hh(), "01");
		assert_eq!(&minus_one_hour.unsigned_hmmss(), "1:00:00");
	}

	#[test]
	fn debug_print() {
		let std_duration = std::time::Duration::new((1 * 60 + 23) * 60 + 45, 678_901_234);
		dbg!(&std_duration.unsigned_hhmmss());
		dbg!(&std_duration.unsigned_hhmmssxxx());
		dbg!(&std_duration.unsigned_mmss());
		dbg!(&std_duration.unsigned_mmssxxx());
		dbg!(&std_duration.unsigned_hmmss());
		dbg!(&std_duration.unsigned_hmmssxxx());
	}
}
