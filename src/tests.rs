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
	
	#[test]
	fn test_smart_hhmmss() {
		let test_cases = vec![
			(std::time::Duration::new(0, 0), "0"),
			(std::time::Duration::new(1, 0), "1s"),
			(std::time::Duration::new(60, 0), "1:00"),
			(std::time::Duration::new(3600, 0), "1:00:00"),
			(std::time::Duration::new(3661, 0), "1:01:01"),
			(std::time::Duration::new(0, 1_000_000), "about 0"),
			(std::time::Duration::new(0, 1_000_001), "about 0"),
			(std::time::Duration::new(0, 1_000_000_000), "1s"),
			(std::time::Duration::new(0, 1_500_000_000), "about 1.50s"),
			(std::time::Duration::new(0, 1_678_901_234), "about 1.67s"),
		];

		for (duration, expected) in test_cases {
			assert_eq!(duration.smart_hhmmss(), expected);
		}

		let negative_test_cases = vec![
			(chrono::Duration::seconds(-1), "-1s"),
			(chrono::Duration::minutes(-1), "-1:00"),
			(chrono::Duration::hours(-1), "-1:00:00"),
			(chrono::Duration::seconds(-3661), "-1:01:01"),
			(chrono::Duration::nanoseconds(-1_000_000), "about 0"),
			(chrono::Duration::nanoseconds(-1_000_001), "about 0"),
			(chrono::Duration::nanoseconds(-1_000_000_000), "-1s"),
			(chrono::Duration::nanoseconds(-1_500_000_000), "about -1.50s"),
			(chrono::Duration::nanoseconds(-1_678_901_234), "about -1.67s"),
		];

		for (duration, expected) in negative_test_cases {
			assert_eq!(duration.smart_hhmmss(), expected);
		}
	}
