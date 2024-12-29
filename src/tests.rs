use crate::{FractPartOfDuration, Hhmmss};

#[test]
fn test_all_crate_durations() {
	let std_duration = std::time::Duration::new((1 * 60 + 23) * 60 + 45, 678_901_234);
	assert_eq!(&std_duration.unsigned_hhmmss(), "01:23:45");
	assert_eq!(&std_duration.unsigned_hhmmssxxx(), "01:23:45.678");
	assert_eq!(&std_duration.unsigned_mmss(), "23:45");
	assert_eq!(&std_duration.unsigned_mmssxxx(), "23:45.678");
	assert_eq!(&std_duration.unsigned_mss(), "23:45");
	assert_eq!(&std_duration.unsigned_mssxxx(), "23:45.678");
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

	fn test<T: Hhmmss>(d: T) {
		assert_eq!(&d.smart_hhmmss(), "about -1:23:45.678");
		assert_eq!(d.get_sign(), "-");
		assert_eq!(d.is_negative(), true);
		assert_eq!(d.part_of_hours(), -1);
		assert_eq!(d.part_of_minutes(), -23);
		assert_eq!(d.part_of_seconds(), -45);
		assert_eq!(d.part_of_milliseconds(), -678);
		assert_eq!(d.part_of_microseconds(), -901);
		assert_eq!(d.part_of_nanoseconds(), -234);
		assert_eq!(d.part_of_hours_abs(), 1);
		assert_eq!(d.part_of_minutes_abs(), 23);
		assert_eq!(d.part_of_seconds_abs(), 45);
		assert_eq!(d.part_of_milliseconds_abs(), 678);
		assert_eq!(d.part_of_microseconds_abs(), 901);
		assert_eq!(d.part_of_nanoseconds_abs(), 234);
		assert_eq!(&d.fmt_hh(), "-01");
		assert_eq!(&d.fmt_mm(), "-23");
		assert_eq!(&d.fmt_ss(), "-45");
		assert_eq!(&d.fmt_xxx(), "-678");
		assert_eq!(&d.mss(), "-23:45");
		assert_eq!(&d.mmss(), "-23:45");
		assert_eq!(&d.hmmss(), "-1:23:45");
		assert_eq!(&d.hhmmss(), "-01:23:45");
		assert_eq!(&d.mssxxx(), "-23:45.678");
		assert_eq!(&d.mmssxxx(), "-23:45.678");
		assert_eq!(&d.hmmssxxx(), "-1:23:45.678");
		assert_eq!(&d.hhmmssxxx(), "-01:23:45.678");
		assert_eq!(&d.unsigned_hh(), "01");
		assert_eq!(&d.unsigned_mm(), "23");
		assert_eq!(&d.unsigned_ss(), "45");
		assert_eq!(&d.unsigned_xxx(), "678");
		assert_eq!(&d.unsigned_mss(), "23:45");
		assert_eq!(&d.unsigned_mmss(), "23:45");
		assert_eq!(&d.unsigned_hmmss(), "1:23:45");
		assert_eq!(&d.unsigned_hhmmss(), "01:23:45");
		assert_eq!(&d.unsigned_mssxxx(), "23:45.678");
		assert_eq!(&d.unsigned_mmssxxx(), "23:45.678");
		assert_eq!(&d.unsigned_hmmssxxx(), "1:23:45.678");
		assert_eq!(&d.unsigned_hhmmssxxx(), "01:23:45.678");
		assert_eq!(d.fract_of_secs(), -0.678901234);
		assert_eq!(d.fract_of_secs_abs(), 0.678901234);
		assert_eq!(&d.fmt_fract(FractPartOfDuration::Nanoseconds), "678901234");
		assert_eq!(
			&d.mss_and_fract(FractPartOfDuration::Milliseconds),
			"-23:45.678"
		);
		assert_eq!(
			&d.mmss_and_fract(FractPartOfDuration::Milliseconds),
			"-23:45.678"
		);
		assert_eq!(
			&d.hmmss_and_fract(FractPartOfDuration::Milliseconds),
			"-1:23:45.678"
		);
		assert_eq!(
			&d.hhmmss_and_fract(FractPartOfDuration::Milliseconds),
			"-01:23:45.678"
		);
		assert_eq!(
			&d.unsigned_mss_and_fract(FractPartOfDuration::Milliseconds),
			"23:45.678"
		);
		assert_eq!(
			&d.unsigned_mmss_and_fract(FractPartOfDuration::Milliseconds),
			"23:45.678"
		);
		assert_eq!(
			&d.unsigned_hmmss_and_fract(FractPartOfDuration::Milliseconds),
			"1:23:45.678"
		);
		assert_eq!(
			&d.unsigned_hhmmss_and_fract(FractPartOfDuration::Milliseconds),
			"01:23:45.678"
		);
	}
}