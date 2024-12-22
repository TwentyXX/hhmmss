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
		let time_duration = time::Duration::from_std(std_duration).unwrap();
		assert_eq!(&time_duration.hhmmss(), "01:23:45");
		assert_eq!(&time_duration.hhmmssxxx(), "01:23:45.678");
		assert_eq!(&time_duration.mmss(), "23:45");
		assert_eq!(&time_duration.mmssxxx(), "23:45.678");
}
```

## All Features
```rust
use hhmmss::Hhmmss as _;

fn main() {
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
}
```

# Acknowledgements

Special thanks to Tianyi Shi.