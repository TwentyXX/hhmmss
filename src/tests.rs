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

#[test]
fn test_smart_hhmmss() {
	assert_eq!(std::time::Duration::new(0, 0).smart_hhmmss(), "0");
	assert_eq!(std::time::Duration::new(1, 0).smart_hhmmss(), "1s");
	assert_eq!(std::time::Duration::new(60, 0).smart_hhmmss(), "1:00");
	assert_eq!(std::time::Duration::new(3600, 0).smart_hhmmss(), "1:00:00");
	assert_eq!(std::time::Duration::new(3661, 0).smart_hhmmss(), "1:01:01");
	assert_eq!(
		std::time::Duration::new(0, 100_000_000).smart_hhmmss(),
		"0.1s"
	);
	assert_eq!(
		std::time::Duration::new(0, 1_000_001).smart_hhmmss(),
		"about 0.001s"
	);
	assert_eq!(
		std::time::Duration::new(0, 1_000_000_000).smart_hhmmss(),
		"1s"
	);
	assert_eq!(
		std::time::Duration::new(0, 1_500_000_000).smart_hhmmss(),
		"1.5s"
	);
	assert_eq!(
		std::time::Duration::new(0, 1_678_901_234).smart_hhmmss(),
		"about 1.678s"
	);
}

#[test]
fn test_fraction_of_seconds() {
	assert_eq!(
		std::time::Duration::from_secs_f64(0.0).fract_of_secs_abs(),
		0.0
	);
	assert_eq!(
		std::time::Duration::from_secs_f64(0.1).fract_of_secs_abs(),
		0.1
	);
	assert_eq!(
		std::time::Duration::from_secs_f64(0.123456789).fract_of_secs_abs(),
		0.123456789
	);
}

#[test]
fn test_unsigned_mmss_and_fract() {
	use FractPartOfDuration;
	let duration = chrono::Duration::seconds((1 * 60 + 23) * 60 + 45)
		+ chrono::Duration::nanoseconds(678_901_234);
	test(duration);
	let duration = time::Duration::new((1 * 60 + 23) * 60 + 45, 678_901_234);
	test(duration);
	let duration = std::time::Duration::new((1 * 60 + 23) * 60 + 45, 678_901_234);
	test(duration);

	fn test<T: Hhmmss>(duration: T) {
		assert_eq!(
			duration.unsigned_mmss_and_fract(FractPartOfDuration::Milliseconds),
			"23:45.678"
		);
		assert_eq!(
			duration.unsigned_mmss_and_fract(FractPartOfDuration::Microseconds),
			"23:45.678901"
		);
		assert_eq!(
			duration.unsigned_mmss_and_fract(FractPartOfDuration::Nanoseconds),
			"23:45.678901234"
		);
	}

	// Test negative duration
	let duration = chrono::Duration::seconds(-((1 * 60 + 23) * 60 + 45))
		+ chrono::Duration::nanoseconds(-678_901_234);
	test2(duration);
	let duration = time::Duration::new(-((1 * 60 + 23) * 60 + 45), -678_901_234);
	test2(duration);

	fn test2<T: Hhmmss>(duration: T) {
		assert_eq!(
			duration.unsigned_mmss_and_fract(FractPartOfDuration::Milliseconds),
			"23:45.678"
		);
		assert_eq!(
			duration.unsigned_mmss_and_fract(FractPartOfDuration::Microseconds),
			"23:45.678901"
		);
		assert_eq!(
			duration.unsigned_mmss_and_fract(FractPartOfDuration::Nanoseconds),
			"23:45.678901234"
		);
	}
}
