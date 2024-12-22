pub trait Hhmmss {
	const NANOSECONDS_IN_A_MICROSECOND: u64 = 1_000;
	const MICROSECONDS_IN_A_MILLISECOND: u64 = 1_000;
	const MILLISECONDS_IN_A_SECOND: u64 = 1_000;
	const SECONDS_IN_A_MINUTE: u64 = 60;
	const MINUTES_IN_AN_HOUR: u64 = 60;

	const HOUR_IN_SECONDS: u64 = Self::SECONDS_IN_A_MINUTE * Self::MINUTES_IN_AN_HOUR;
	const NANOSECONDS_IN_A_SECOND: u64 = Self::NANOSECONDS_IN_A_MICROSECOND
		* Self::MICROSECONDS_IN_A_MILLISECOND
		* Self::MILLISECONDS_IN_A_SECOND;

	fn get_sign(&self) -> String {
		if self.is_negative() {
			"-".to_owned()
		} else {
			"".to_owned()
		}
	}
	fn is_negative(&self) -> bool;
	fn part_of_hours_abs(&self) -> u64;
	fn part_of_minutes_abs(&self) -> u64;
	fn part_of_seconds_abs(&self) -> u64;
	fn part_of_milliseconds_abs(&self) -> u64;
	fn part_of_microseconds_abs(&self) -> u64;
	fn part_of_nanoseconds_abs(&self) -> u64;
	fn part_of_hours(&self) -> i64;
	fn part_of_minutes(&self) -> i64;
	fn part_of_seconds(&self) -> i64;
	fn part_of_milliseconds(&self) -> i64;
	fn part_of_microseconds(&self) -> i64;
	fn part_of_nanoseconds(&self) -> i64;

	fn unsigned_hh(&self) -> String { format!("{:02}", self.part_of_hours_abs()) }
	fn unsigned_mm(&self) -> String { format!("{:02}", self.part_of_minutes_abs()) }
	fn unsigned_ss(&self) -> String { format!("{:02}", self.part_of_seconds_abs()) }
	fn unsigned_xxx(&self) -> String { format!("{:03}", self.part_of_milliseconds_abs()) }

	fn fmt_hh(&self) -> String { self.get_sign() + &self.unsigned_hh() }
	fn fmt_mm(&self) -> String { self.get_sign() + &self.unsigned_mm() }
	fn fmt_ss(&self) -> String { self.get_sign() + &self.unsigned_ss() }
	fn fmt_xxx(&self) -> String { self.get_sign() + &self.unsigned_xxx() }

	/// Pretty-prints a chrono::Duration in the form `MM:SS`
	fn unsigned_mmss(&self) -> String { format!("{}:{}", self.unsigned_mm(), self.unsigned_ss()) }
	/// Pretty-prints a chrono::Duration in the form `MM:SS.xxx`
	fn unsigned_mmssxxx(&self) -> String {
		format!(
			"{}:{}.{}",
			self.unsigned_mm(),
			self.unsigned_ss(),
			self.unsigned_xxx()
		)
	}
	/// Pretty-print chrono::Duration in the format
	/// `M:SS` if the minute value is one digit,
	/// otherwise in the format `MM:SS`.
	fn unsigned_mss(&self) -> String {
		format!("{}:{}", self.part_of_minutes_abs(), self.unsigned_ss())
	}
	/// Pretty-print chrono::Duration in the format
	/// `M:SS.xxx` if the minute value is one digit,
	/// otherwise in the format `MM:SS.xxx`.
	fn unsigned_mssxxx(&self) -> String {
		format!(
			"{}:{}.{}",
			self.part_of_minutes_abs(),
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
			self.part_of_hours_abs(),
			self.unsigned_mm(),
			self.unsigned_ss()
		)
	}
	fn unsigned_hmmssxxx(&self) -> String {
		format!(
			"{}:{}:{}.{}",
			self.part_of_hours_abs(),
			self.unsigned_mm(),
			self.unsigned_ss(),
			self.unsigned_xxx()
		)
	}
	fn mmss(&self) -> String { self.get_sign() + &self.unsigned_mmss() }
	fn mmssxxx(&self) -> String { self.get_sign() + &self.unsigned_mmssxxx() }
	fn mss(&self) -> String { self.get_sign() + &self.unsigned_mss() }
	fn mssxxx(&self) -> String { self.get_sign() + &self.unsigned_mssxxx() }
	fn hhmmss(&self) -> String { self.get_sign() + &self.unsigned_hhmmss() }
	fn hhmmssxxx(&self) -> String { self.get_sign() + &self.unsigned_hhmmssxxx() }
	fn hmmss(&self) -> String { self.get_sign() + &self.unsigned_hmmss() }
	fn hmmssxxx(&self) -> String { self.get_sign() + &self.unsigned_hmmssxxx() }
}

impl Hhmmss for chrono::Duration {
	fn part_of_hours(&self) -> i64 { self.num_hours() }

	fn part_of_minutes(&self) -> i64 { self.num_minutes() % Self::MINUTES_IN_AN_HOUR as i64 }

	fn part_of_seconds(&self) -> i64 { self.num_seconds() % Self::SECONDS_IN_A_MINUTE as i64 }

	fn part_of_milliseconds(&self) -> i64 {
		self.num_milliseconds() % Self::MILLISECONDS_IN_A_SECOND as i64
	}

	fn part_of_microseconds(&self) -> i64 {
		self.num_microseconds().unwrap_or(0) % Self::MICROSECONDS_IN_A_MILLISECOND as i64
	}

	fn part_of_nanoseconds(&self) -> i64 {
		self.num_nanoseconds().unwrap_or(0) % Self::NANOSECONDS_IN_A_MICROSECOND as i64
	}

	fn part_of_hours_abs(&self) -> u64 { self.num_hours().unsigned_abs() }

	fn part_of_minutes_abs(&self) -> u64 {
		self.num_minutes().unsigned_abs() % Self::MINUTES_IN_AN_HOUR
	}

	fn part_of_seconds_abs(&self) -> u64 {
		self.num_seconds().unsigned_abs() % Self::SECONDS_IN_A_MINUTE
	}

	fn part_of_milliseconds_abs(&self) -> u64 {
		self.num_milliseconds().unsigned_abs() % Self::MILLISECONDS_IN_A_SECOND
	}

	fn part_of_microseconds_abs(&self) -> u64 {
		self.num_microseconds().unwrap_or(0).unsigned_abs() % Self::MICROSECONDS_IN_A_MILLISECOND
	}

	fn part_of_nanoseconds_abs(&self) -> u64 {
		self.num_nanoseconds().unwrap_or(0).unsigned_abs() % Self::NANOSECONDS_IN_A_MICROSECOND
	}

	fn is_negative(&self) -> bool { self.num_seconds() < 0 }
}

impl Hhmmss for std::time::Duration {
	fn part_of_hours(&self) -> i64 { (self.as_secs() / Self::HOUR_IN_SECONDS) as i64 }

	fn part_of_minutes(&self) -> i64 {
		((self.as_secs() / Self::SECONDS_IN_A_MINUTE) % Self::MINUTES_IN_AN_HOUR) as i64
	}

	fn part_of_seconds(&self) -> i64 { (self.as_secs() % Self::SECONDS_IN_A_MINUTE) as i64 }

	fn part_of_milliseconds(&self) -> i64 { self.subsec_millis() as i64 }

	fn part_of_microseconds(&self) -> i64 {
		(self.subsec_micros() % Self::MICROSECONDS_IN_A_MILLISECOND as u32) as i64
	}

	fn part_of_nanoseconds(&self) -> i64 {
		(self.subsec_nanos() % Self::NANOSECONDS_IN_A_MICROSECOND as u32) as i64
	}

	fn part_of_hours_abs(&self) -> u64 { self.as_secs() / Self::HOUR_IN_SECONDS }

	fn part_of_minutes_abs(&self) -> u64 {
		(self.as_secs() / Self::SECONDS_IN_A_MINUTE) % Self::MINUTES_IN_AN_HOUR
	}

	fn part_of_seconds_abs(&self) -> u64 { self.as_secs() % Self::SECONDS_IN_A_MINUTE }

	fn part_of_milliseconds_abs(&self) -> u64 { self.subsec_millis() as u64 }

	fn part_of_microseconds_abs(&self) -> u64 {
		(self.subsec_micros() % Self::MICROSECONDS_IN_A_MILLISECOND as u32) as u64
	}

	fn part_of_nanoseconds_abs(&self) -> u64 {
		(self.subsec_nanos() % Self::NANOSECONDS_IN_A_MICROSECOND as u32) as u64
	}

	fn is_negative(&self) -> bool { false }

	fn get_sign(&self) -> String { "".to_owned() }
}

impl Hhmmss for time::Duration {
	fn part_of_hours(&self) -> i64 { self.whole_hours() }

	fn part_of_minutes(&self) -> i64 { self.whole_minutes() % Self::MINUTES_IN_AN_HOUR as i64 }

	fn part_of_seconds(&self) -> i64 { self.whole_seconds() % Self::SECONDS_IN_A_MINUTE as i64 }

	fn part_of_milliseconds(&self) -> i64 { self.subsec_milliseconds() as i64 }

	fn part_of_microseconds(&self) -> i64 {
		(self.subsec_microseconds() % Self::MICROSECONDS_IN_A_MILLISECOND as i32) as i64
	}

	fn part_of_nanoseconds(&self) -> i64 {
		(self.subsec_nanoseconds() % Self::NANOSECONDS_IN_A_MICROSECOND as i32) as i64
	}

	fn part_of_hours_abs(&self) -> u64 { self.whole_hours().unsigned_abs() }

	fn part_of_minutes_abs(&self) -> u64 {
		self.whole_minutes().unsigned_abs() % Self::MINUTES_IN_AN_HOUR
	}

	fn part_of_seconds_abs(&self) -> u64 {
		self.whole_seconds().unsigned_abs() % Self::SECONDS_IN_A_MINUTE
	}

	fn part_of_milliseconds_abs(&self) -> u64 {
		self.subsec_milliseconds().unsigned_abs() as u64 % Self::MILLISECONDS_IN_A_SECOND
	}

	fn part_of_microseconds_abs(&self) -> u64 {
		self.subsec_microseconds().unsigned_abs() as u64 % Self::MICROSECONDS_IN_A_MILLISECOND
	}

	fn part_of_nanoseconds_abs(&self) -> u64 {
		self.subsec_nanoseconds().unsigned_abs() as u64 % Self::NANOSECONDS_IN_A_MICROSECOND
	}

	fn is_negative(&self) -> bool { self.whole_seconds() < 0 }
}

#[cfg(test)]
mod tests {

	use crate::Hhmmss;

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
		let time_duration = time::Duration::try_from(std_duration).unwrap();
		assert_eq!(&time_duration.unsigned_hhmmssxxx(), "01:23:45.678");
	}

	#[test]
	fn test_all_features() {
		let minus_one_hour = chrono::Duration::seconds(-((1 * 60 + 23) * 60 + 45))
			+ chrono::Duration::nanoseconds(-678_901_234);
		test(minus_one_hour);
		let minus_one_hour = time::Duration::new(-((1 * 60 + 23) * 60 + 45), -678_901_234);
		test(minus_one_hour);

		fn test<T: Hhmmss>(minus_one_hour: T) {
			assert_eq!(minus_one_hour.part_of_hours(), -1);
			assert_eq!(minus_one_hour.part_of_minutes(), -23);
			assert_eq!(minus_one_hour.part_of_seconds(), -45);
			assert_eq!(minus_one_hour.part_of_milliseconds(), -678);
			assert_eq!(minus_one_hour.part_of_microseconds(), -901);
			assert_eq!(minus_one_hour.part_of_nanoseconds(), -234);
			assert_eq!(minus_one_hour.part_of_hours_abs(), 1);
			assert_eq!(minus_one_hour.part_of_minutes_abs(), 23);
			assert_eq!(minus_one_hour.part_of_seconds_abs(), 45);
			assert_eq!(minus_one_hour.part_of_milliseconds_abs(), 678);
			assert_eq!(minus_one_hour.part_of_microseconds_abs(), 901);
			assert_eq!(minus_one_hour.part_of_nanoseconds_abs(), 234);
			assert_eq!(minus_one_hour.is_negative(), true);

			assert_eq!(&minus_one_hour.fmt_hh(), "-01");
			assert_eq!(&minus_one_hour.fmt_mm(), "-23");
			assert_eq!(&minus_one_hour.fmt_ss(), "-45");
			assert_eq!(&minus_one_hour.mss(), "-23:45");
			assert_eq!(&minus_one_hour.mmss(), "-23:45");
			assert_eq!(&minus_one_hour.hmmss(), "-1:23:45");
			assert_eq!(&minus_one_hour.hhmmss(), "-01:23:45");
			assert_eq!(&minus_one_hour.mssxxx(), "-23:45.678");
			assert_eq!(&minus_one_hour.mmssxxx(), "-23:45.678");
			assert_eq!(&minus_one_hour.hmmssxxx(), "-1:23:45.678");
			assert_eq!(&minus_one_hour.hhmmssxxx(), "-01:23:45.678");
			assert_eq!(&minus_one_hour.unsigned_hh(), "01");
			assert_eq!(&minus_one_hour.unsigned_mm(), "23");
			assert_eq!(&minus_one_hour.unsigned_ss(), "45");
			assert_eq!(&minus_one_hour.unsigned_mss(), "23:45");
			assert_eq!(&minus_one_hour.unsigned_mmss(), "23:45");
			assert_eq!(&minus_one_hour.unsigned_hmmss(), "1:23:45");
			assert_eq!(&minus_one_hour.unsigned_hhmmss(), "01:23:45");
			assert_eq!(&minus_one_hour.unsigned_mssxxx(), "23:45.678");
			assert_eq!(&minus_one_hour.unsigned_mmssxxx(), "23:45.678");
			assert_eq!(&minus_one_hour.unsigned_hmmssxxx(), "1:23:45.678");
			assert_eq!(&minus_one_hour.unsigned_hhmmssxxx(), "01:23:45.678");

			assert_eq!(&minus_one_hour.get_sign(), "-");
		}
	}
	#[test]
	fn test_all_funcs_without_negative() {
		let std_duration = std::time::Duration::new((1 * 60 + 23) * 60 + 45, 678_901_234);
		test(std_duration);
		let sample_val = time::Duration::try_from(std_duration).unwrap();
		test(sample_val);
		let sample_val = chrono::Duration::from_std(std_duration).unwrap();
		test(sample_val);

		fn test<T: Hhmmss>(sample_val: T) {
			assert_eq!(sample_val.part_of_hours(), 1);
			assert_eq!(sample_val.part_of_minutes(), 23);
			assert_eq!(sample_val.part_of_seconds(), 45);
			assert_eq!(sample_val.part_of_milliseconds(), 678);
			assert_eq!(sample_val.part_of_microseconds(), 901);
			assert_eq!(sample_val.part_of_nanoseconds(), 234);
			assert_eq!(sample_val.part_of_hours_abs(), 1);
			assert_eq!(sample_val.part_of_minutes_abs(), 23);
			assert_eq!(sample_val.part_of_seconds_abs(), 45);
			assert_eq!(sample_val.part_of_milliseconds_abs(), 678);
			assert_eq!(sample_val.part_of_microseconds_abs(), 901);
			assert_eq!(sample_val.part_of_nanoseconds_abs(), 234);

			assert_eq!(&sample_val.fmt_hh(), "01");
			assert_eq!(&sample_val.fmt_mm(), "23");
			assert_eq!(&sample_val.fmt_ss(), "45");
			assert_eq!(&sample_val.mss(), "23:45");
			assert_eq!(&sample_val.mmss(), "23:45");
			assert_eq!(&sample_val.hmmss(), "1:23:45");
			assert_eq!(&sample_val.hhmmss(), "01:23:45");
			assert_eq!(&sample_val.mssxxx(), "23:45.678");
			assert_eq!(&sample_val.mmssxxx(), "23:45.678");
			assert_eq!(&sample_val.hmmssxxx(), "1:23:45.678");
			assert_eq!(&sample_val.hhmmssxxx(), "01:23:45.678");
			assert_eq!(&sample_val.unsigned_hh(), "01");
			assert_eq!(&sample_val.unsigned_mm(), "23");
			assert_eq!(&sample_val.unsigned_ss(), "45");
			assert_eq!(&sample_val.unsigned_mss(), "23:45");
			assert_eq!(&sample_val.unsigned_mmss(), "23:45");
			assert_eq!(&sample_val.unsigned_hmmss(), "1:23:45");
			assert_eq!(&sample_val.unsigned_hhmmss(), "01:23:45");
			assert_eq!(&sample_val.unsigned_mssxxx(), "23:45.678");
			assert_eq!(&sample_val.unsigned_mmssxxx(), "23:45.678");
			assert_eq!(&sample_val.unsigned_hmmssxxx(), "1:23:45.678");
			assert_eq!(&sample_val.unsigned_hhmmssxxx(), "01:23:45.678");

			assert_eq!(&sample_val.get_sign(), "");
		}
	}
}
