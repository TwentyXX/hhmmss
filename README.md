# hhmmss

Format time and duration in chrono, std::time and time as `HH:MM:SS` or `HH:MM:SS.xxx`.

## Why?

I just wanted to format `chrono::Duration` as `HH:MM:SS` in some of my crates. However there isn't a ready-to-use method for this. Format is only supported for instances. Neither `time` nor `std::time`'s `Duration` support this. So I made this crate, mainly for my own convinience.

# Usage

Add `hhmmss` to `Cargo.toml`:

```toml
[dependencies]
hhmmss = "0.1"
```

Then:

```rust
use hhmmss::Hhmmss as _;

fn main() {
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
		let time_duration = time::Duration::try_from(std_duration).unwrap();
		assert_eq!(&time_duration.hhmmss(), "01:23:45");
		assert_eq!(&time_duration.hhmmssxxx(), "01:23:45.678");
		assert_eq!(&time_duration.mmss(), "23:45");
		assert_eq!(&time_duration.mmssxxx(), "23:45.678");
}
```

## All Features
```rust
use hhmmss::{Hhmmss, FractPartOfDuration} as _;

#[test]
fn test_all_features() {
	let minus_one_hour = chrono::Duration::seconds(-((1 * 60 + 23) * 60 + 45))
		+ chrono::Duration::nanoseconds(-678_901_234);
	test(minus_one_hour);
	let minus_one_hour = time::Duration::new(-((1 * 60 + 23) * 60 + 45), -678_901_234);
	test(minus_one_hour);

	fn test<T: Hhmmss>(d: T) {
		assert_eq!(&d.fmt_smart(), "about -1:23:45.678");
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
```

## `fmt_smart` function
```rust

#[test]
fn test_fmt_smart() {
	// Zero duration
	let d = chrono::Duration::zero();
	assert_eq!(d.fmt_smart(), "0");

	// Seconds only (integer)
	let d = chrono::Duration::seconds(5);
	assert_eq!(d.fmt_smart(), "5s");

	// Seconds with milliseconds (exact)
	let d = chrono::Duration::milliseconds(1234);
	assert_eq!(d.fmt_smart(), "1.234s");

	// Seconds with fractional part (not exact milliseconds)
	let d = chrono::Duration::nanoseconds(1234567890);
	assert_eq!(d.fmt_smart(), "about 1.234s");

	// Minutes and seconds
	let d = chrono::Duration::seconds(83); // 1:23
	assert_eq!(d.fmt_smart(), "1:23");

	// Minutes and seconds with milliseconds
	let d = chrono::Duration::seconds(83) + chrono::Duration::milliseconds(456);
	assert_eq!(d.fmt_smart(), "1:23.456");

	// Hours, minutes and seconds
	let d = chrono::Duration::seconds(7296); // 2:01:36
	assert_eq!(d.fmt_smart(), "2:01:36");

	// Hours, minutes, seconds with milliseconds
	let d = chrono::Duration::seconds(7296) + chrono::Duration::milliseconds(789);
	assert_eq!(d.fmt_smart(), "2:01:36.789");

	// Negative durations
	let d = chrono::Duration::seconds(-5);
	assert_eq!(d.fmt_smart(), "-5s");

	let d = chrono::Duration::milliseconds(-1234);
	assert_eq!(d.fmt_smart(), "-1.234s");

	let d = chrono::Duration::seconds(-83);
	assert_eq!(d.fmt_smart(), "-1:23");

	let d = chrono::Duration::seconds(-7296) + chrono::Duration::milliseconds(-789);
	assert_eq!(d.fmt_smart(), "-2:01:36.789");

	// The following behavior was due to lax handling of floating point numbers and
	// has been corrected since version 0.1.5.
	let d = chrono::Duration::milliseconds(2405);
	// As of 0.1.4, this test passed.
	// assert_eq!(d.fmt_smart(), "2.4050000000000002s");
	// 0.1.5 or later:
	assert_eq!(d.fmt_smart(), "2.405s");
	
	let d = std::time::Duration::from_secs_f64(2.405000002); // The last 2 are included in nanoseconds.
	assert_eq!(d.fmt_smart(), "about 2.405s");

	let d = std::time::Duration::from_secs_f64(2.4050000002); // The last 2 are not included in nanoseconds.
	assert_eq!(d.fmt_smart(), "2.405s");

}
```

# Acknowledgements

Special thanks to Tianyi Shi.