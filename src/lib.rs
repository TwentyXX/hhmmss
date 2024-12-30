use fracts::FractPartOfDuration;

pub mod fracts;
#[cfg(test)] mod tests;
pub trait Hhmmss {
	const NANOSECONDS_IN_A_MICROSECOND: u64 = 1_000;
	const MICROSECONDS_IN_A_MILLISECOND: u64 = 1_000;
	const MILLISECONDS_IN_A_SECOND: u64 = 1_000;
	const SECONDS_IN_A_MINUTE: u64 = 60;
	const MINUTES_IN_AN_HOUR: u64 = 60;

	const SECONDS_IN_A_HOUR: u64 = Self::SECONDS_IN_A_MINUTE * Self::MINUTES_IN_AN_HOUR;
	const NANOSECONDS_IN_A_SECOND: u64 = Self::NANOSECONDS_IN_A_MICROSECOND
		* Self::MICROSECONDS_IN_A_MILLISECOND
		* Self::MILLISECONDS_IN_A_SECOND;

	/// Returns the sign of the duration as a string.
	/// "-" for negative durations, and an empty string for non-negative
	/// durations. The output is either "-" or "".
	fn get_sign(&self) -> String {
		if self.is_negative() {
			"-".to_owned()
		} else {
			"".to_owned()
		}
	}

	/// Checks if the duration is negative.
	/// Returns `true` if negative, otherwise `false`.
	fn is_negative(&self) -> bool;
	/// Returns the absolute value of the hours part of the duration.
	/// The output is in the range [0, ∞).
	fn part_of_hours_abs(&self) -> u64;
	/// Returns the absolute value of the minutes part of the duration.
	/// The output is in the range [0, 59].
	fn part_of_minutes_abs(&self) -> u64;
	/// Returns the absolute value of the seconds part of the duration.
	/// The output is in the range [0, 59].
	fn part_of_seconds_abs(&self) -> u64;
	/// Returns the absolute value of the milliseconds part of the duration.
	/// The output is in the range [0, 999].
	fn part_of_milliseconds_abs(&self) -> u64;
	/// Returns the absolute value of the microseconds part of the duration.
	/// The output is in the range [0, 999].
	fn part_of_microseconds_abs(&self) -> u64;
	/// Returns the absolute value of the nanoseconds part of the duration.
	/// The output is in the range [0, 999].
	fn part_of_nanoseconds_abs(&self) -> u64;
	/// Returns the hours part of the duration.
	/// The output can be negative or positive, depending on the duration.
	fn part_of_hours(&self) -> i64;
	/// Returns the minutes part of the duration.
	/// The output is in the range [-59, 59].
	fn part_of_minutes(&self) -> i64;
	/// Returns the seconds part of the duration.
	/// The output is in the range [-59, 59].
	fn part_of_seconds(&self) -> i64;
	/// Returns the milliseconds part of the duration.
	/// The output is in the range [-999, 999].
	fn part_of_milliseconds(&self) -> i64;
	/// Returns the microseconds part of the duration.
	/// The output is in the range [-999, 999].
	fn part_of_microseconds(&self) -> i64;
	/// Returns the nanoseconds part of the duration.
	/// The output is in the range [-999, 999].
	fn part_of_nanoseconds(&self) -> i64;

	/// Formats the absolute value of the hours part as a two-digit string.
	/// The output is in the format "HH".
	fn unsigned_hh(&self) -> String { format!("{:02}", self.part_of_hours_abs()) }
	/// Formats the absolute value of the minutes part as a two-digit string.
	/// The output is in the format "MM".
	fn unsigned_mm(&self) -> String { format!("{:02}", self.part_of_minutes_abs()) }
	/// Formats the absolute value of the seconds part as a two-digit string.
	/// The output is in the format "SS".
	fn unsigned_ss(&self) -> String { format!("{:02}", self.part_of_seconds_abs()) }
	/// Formats the absolute value of the milliseconds part as a three-digit
	/// string. The output is in the format "xxx".
	fn unsigned_xxx(&self) -> String { format!("{:03}", self.part_of_milliseconds_abs()) }

	/// Formats the hours part with a sign.
	/// The output is in the format "-HH" or "HH".
	fn fmt_hh(&self) -> String { self.get_sign() + &self.unsigned_hh() }
	/// Formats the minutes part with a sign.
	/// The output is in the format "-MM" or "MM".
	fn fmt_mm(&self) -> String { self.get_sign() + &self.unsigned_mm() }
	/// Formats the seconds part with a sign.
	/// The output is in the format "-SS" or "SS".
	fn fmt_ss(&self) -> String { self.get_sign() + &self.unsigned_ss() }
	/// Formats the milliseconds part with a sign.
	/// The output is in the format "-xxx" or "xxx".
	fn fmt_xxx(&self) -> String { self.get_sign() + &self.unsigned_xxx() }

	/// Formats the absolute value of the duration as "MM:SS".
	/// The output is in the format "MM:SS".
	fn unsigned_mmss(&self) -> String { format!("{}:{}", self.unsigned_mm(), self.unsigned_ss()) }
	/// Formats the absolute value of the duration as "MM:SS.xxx".
	/// The output is in the format "MM:SS.xxx".
	fn unsigned_mmssxxx(&self) -> String {
		format!(
			"{}:{}.{}",
			self.unsigned_mm(),
			self.unsigned_ss(),
			self.unsigned_xxx()
		)
	}

	/// Formats the absolute value of the duration as "MM:SS" with fractional seconds.
	/// The output is in the format "MM:SS.xxx", "MM:SS.xxxxxx", or "MM:SS.xxxxxxxxx"
	/// depending on the specified fractional part.
	fn unsigned_mmss_and_fract(&self, included: FractPartOfDuration) -> String {
		format!(
			"{}:{}.{}",
			self.unsigned_mm(),
			self.unsigned_ss(),
			self.fmt_fract(included)
		)
	}
	/// Formats the absolute value of the duration as "M:SS".
	/// The output is in the format "M:SS".
	fn unsigned_mss(&self) -> String {
		format!("{}:{}", self.part_of_minutes_abs(), self.unsigned_ss())
	}
	/// Formats the absolute value of the duration as "M:SS.xxx".
	/// The output is in the format "M:SS.xxx".
	fn unsigned_mssxxx(&self) -> String {
		format!(
			"{}:{}.{}",
			self.part_of_minutes_abs(),
			self.unsigned_ss(),
			self.unsigned_xxx(),
		)
	}

	/// Formats the absolute value of the duration as "M:SS" with fractional seconds.
	/// The output is in the format "M:SS.xxx", "M:SS.xxxxxx", or "M:SS.xxxxxxxxx"
	/// depending on the specified fractional part.
	fn unsigned_mss_and_fract(&self, included: FractPartOfDuration) -> String {
		format!(
			"{}:{}.{}",
			self.part_of_minutes_abs(),
			self.unsigned_ss(),
			self.fmt_fract(included)
		)
	}
	/// Formats the absolute value of the duration as "HH:MM:SS".
	/// The output is in the format "HH:MM:SS".
	fn unsigned_hhmmss(&self) -> String {
		format!(
			"{}:{}:{}",
			self.unsigned_hh(),
			self.unsigned_mm(),
			self.unsigned_ss(),
		)
	}

	/// Formats the absolute value of the duration as "HH:MM:SS.xxx".
	/// The output is in the format "HH:MM:SS.xxx".
	fn unsigned_hhmmssxxx(&self) -> String {
		format!(
			"{}:{}:{}.{}",
			self.unsigned_hh(),
			self.unsigned_mm(),
			self.unsigned_ss(),
			self.unsigned_xxx()
		)
	}

	/// Formats the absolute value of the duration as "HH:MM:SS" with fractional seconds.
	/// The output is in the format "HH:MM:SS.xxx", "HH:MM:SS.xxxxxx", or "HH:MM:SS.xxxxxxxxx"
	/// depending on the specified fractional part.
	fn unsigned_hhmmss_and_fract(&self, included: FractPartOfDuration) -> String {
		format!(
			"{}:{}:{}.{}",
			self.unsigned_hh(),
			self.unsigned_mm(),
			self.unsigned_ss(),
			self.fmt_fract(included)
		)
	}
	/// Formats the absolute value of the duration as "H:MM:SS".
	/// The output is in the format "H:MM:SS".
	fn unsigned_hmmss(&self) -> String {
		format!(
			"{}:{}:{}",
			self.part_of_hours_abs(),
			self.unsigned_mm(),
			self.unsigned_ss()
		)
	}
	/// Formats the absolute value of the duration as "H:MM:SS.xxx".
	/// The output is in the format "H:MM:SS.xxx".
	fn unsigned_hmmssxxx(&self) -> String {
		format!(
			"{}:{}:{}.{}",
			self.part_of_hours_abs(),
			self.unsigned_mm(),
			self.unsigned_ss(),
			self.unsigned_xxx()
		)
	}

	/// Formats the absolute value of the duration as "H:MM:SS" with fractional seconds.
	/// The output is in the format "H:MM:SS.xxx", "H:MM:SS.xxxxxx", or "H:MM:SS.xxxxxxxxx"
	/// depending on the specified fractional part.
	fn unsigned_hmmss_and_fract(&self, included: FractPartOfDuration) -> String {
		format!(
			"{}:{}:{}.{}",
			self.part_of_hours_abs(),
			self.unsigned_mm(),
			self.unsigned_ss(),
			self.fmt_fract(included)
		)
	}
	/// Formats the duration as "MM:SS" with a sign.
	/// The output is in the format "-MM:SS" or "MM:SS".
	fn mmss(&self) -> String { self.get_sign() + &self.unsigned_mmss() }
	/// Formats the duration as "MM:SS.xxx" with a sign.
	/// The output is in the format "-MM:SS.xxx" or "MM:SS.xxx".
	fn mmssxxx(&self) -> String { self.get_sign() + &self.unsigned_mmssxxx() }
	/// Formats the duration as "MM:SS" with fractional seconds and a sign.
	/// The output is in the format "-MM:SS.xxx", "-MM:SS.xxxxxx", or "-MM:SS.xxxxxxxxx"
	/// (or without "-" for positive durations) depending on the specified fractional part.
	fn mmss_and_fract(&self, included: FractPartOfDuration) -> String {
		self.get_sign() + &self.unsigned_mmss_and_fract(included)
	}
	/// Formats the duration as "M:SS" with a sign.
	/// The output is in the format "-M:SS" or "M:SS".
	fn mss(&self) -> String { self.get_sign() + &self.unsigned_mss() }
	/// Formats the duration as "M:SS.xxx" with a sign.
	/// The output is in the format "-M:SS.xxx" or "M:SS.xxx".
	fn mssxxx(&self) -> String { self.get_sign() + &self.unsigned_mssxxx() }
	/// Formats the duration as "M:SS" with fractional seconds and a sign.
	/// The output is in the format "-M:SS.xxx", "-M:SS.xxxxxx", or "-M:SS.xxxxxxxxx"
	/// (or without "-" for positive durations) depending on the specified fractional part.
	fn mss_and_fract(&self, included: FractPartOfDuration) -> String {
		self.get_sign() + &self.unsigned_mss_and_fract(included)
	}
	/// Formats the duration as "HH:MM:SS" with a sign.
	/// The output is in the format "-HH:MM:SS" or "HH:MM:SS".
	fn hhmmss(&self) -> String { self.get_sign() + &self.unsigned_hhmmss() }
	/// Formats the duration as "HH:MM:SS.xxx" with a sign.
	/// The output is in the format "-HH:MM:SS.xxx" or "HH:MM:SS.xxx".
	fn hhmmssxxx(&self) -> String { self.get_sign() + &self.unsigned_hhmmssxxx() }
	/// Formats the duration as "HH:MM:SS" with fractional seconds and a sign.
	/// The output is in the format "-HH:MM:SS.xxx", "-HH:MM:SS.xxxxxx", or "-HH:MM:SS.xxxxxxxxx"
	/// (or without "-" for positive durations) depending on the specified fractional part.
	fn hhmmss_and_fract(&self, included: FractPartOfDuration) -> String {
		self.get_sign() + &self.unsigned_hhmmss_and_fract(included)
	}
	/// Formats the duration as "H:MM:SS" with a sign.
	/// The output is in the format "-H:MM:SS" or "H:MM:SS".
	fn hmmss(&self) -> String { self.get_sign() + &self.unsigned_hmmss() }
	/// Formats the duration as "H:MM:SS.xxx" with a sign.
	/// The output is in the format "-H:MM:SS.xxx" or "H:MM:SS.xxx".
	fn hmmssxxx(&self) -> String { self.get_sign() + &self.unsigned_hmmssxxx() }
	/// Formats the duration as "H:MM:SS" with fractional seconds and a sign.
	/// The output is in the format "-H:MM:SS.xxx", "-H:MM:SS.xxxxxx", or "-H:MM:SS.xxxxxxxxx"
	/// (or without "-" for positive durations) depending on the specified fractional part.
	fn hmmss_and_fract(&self, included: FractPartOfDuration) -> String {
		self.get_sign() + &self.unsigned_hmmss_and_fract(included)
	}
	
	#[deprecated(since = "0.1.4", note = "Please use `fmt_smart` instead")]
	fn smart_hhmmss(&self) -> String {
		self.fmt_smart()
	}

	/// Formats the duration in a smart way, choosing the most appropriate format based on the duration value.
	/// 
	/// The output format is determined by the following rules:
	/// - For zero duration: "0"
	/// - For seconds only: "Xs" (where X is the number of seconds)
	/// - For minutes and seconds: "M:SS" format
	/// - For hours, minutes and seconds: "H:MM:SS" format
	/// 
	/// If milliseconds are present:
	/// - For seconds only: "X.xxxs" format
	/// - For minutes: "M:SS.xxx" format
	/// - For hours: "H:MM:SS.xxx" format
	/// 
	/// If the fractional part is not exact in milliseconds, the output is prefixed with "about ".
	/// 
	/// # Examples
	/// - "0" for zero duration
	/// - "5s" for 5 seconds
	/// - "1:23" for 1 minute 23 seconds
	/// - "2:34:56" for 2 hours 34 minutes 56 seconds
	/// - "1.234s" for 1.234 seconds
	/// - "about 1.2345s" for 1.2345 seconds
	fn fmt_smart(&self) -> String {
		let mut value = if self.part_of_milliseconds() == 0 {
			if self.part_of_hours() == 0 {
				if self.part_of_minutes() == 0 {
					if self.part_of_seconds() == 0 {
						"0".to_owned()
					} else {
						format!("{}s", self.part_of_seconds())
					}
				} else {
					self.mss()
				}
			} else {
				self.hmmss()
			}
		} else {
			if self.part_of_hours() == 0 {
				if self.part_of_minutes() == 0 {
					format!(
						"{}.{}s",
						self.part_of_seconds() as f64,
						self.fmt_fract(FractPartOfDuration::Milliseconds)
					)
				} else {
					self.mss_and_fract(FractPartOfDuration::Milliseconds)
				}
			} else {
				self.hmmss_and_fract(FractPartOfDuration::Milliseconds)
			}
		};

		value.insert_str(
			0,
			if self.part_of_microseconds() == 0 && self.part_of_nanoseconds() == 0 {
				""
			} else {
				"about "
			},
		);

		value
	}
	fn fract_of_secs_abs(&self) -> f64 {
		((self.part_of_nanoseconds_abs() as f64 / Self::NANOSECONDS_IN_A_MICROSECOND as f64
			+ self.part_of_microseconds_abs() as f64)
			/ Self::MICROSECONDS_IN_A_MILLISECOND as f64
			+ self.part_of_milliseconds_abs() as f64)
			/ Self::MILLISECONDS_IN_A_SECOND as f64
	}
	fn fract_of_secs(&self) -> f64 {
		self.fract_of_secs_abs()
			* if self.is_negative() {
				-1.0
			} else {
				1.0
			}
	}
	fn fmt_fract(&self, included: FractPartOfDuration) -> String {
		((self.fract_of_secs_abs() * included.units_per_sec() as f64).floor()
			/ included.units_per_sec() as f64)
			.fract()
			.to_string()[2..]
			.to_string()
	}
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
	fn part_of_hours(&self) -> i64 { (self.as_secs() / Self::SECONDS_IN_A_HOUR) as i64 }

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

	fn part_of_hours_abs(&self) -> u64 { self.as_secs() / Self::SECONDS_IN_A_HOUR }

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
