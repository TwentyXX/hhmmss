use super::{Hhmmss as _, HasSubseconds as _};
#[test]
pub(crate) fn test_nanos() {
	    let d = chrono::Duration::nanoseconds(123_456_789);
	    assert_eq!(d.subsecs(), 123_456_789);
	    assert_eq!(d.unsigned_subsecs(), 123_456_789);

	    let d = std::time::Duration::new(0, 123_456_789);
	    assert_eq!(d.subsecs(), 123_456_789);
	    assert_eq!(d.unsigned_subsecs(), 123_456_789);

	    let d = time::Duration::new(0, 123_456_789);
	    assert_eq!(d.subsecs(), 123_456_789);
	    assert_eq!(d.unsigned_subsecs(), 123_456_789);

	    // より複雑なテスト

	    let d = chrono::Duration::nanoseconds(1_234_567_890);
	    assert_eq!(d.subsecs(), 234_567_890);
	    assert_eq!(d.unsigned_subsecs(), 234_567_890);

	    let d = std::time::Duration::new(1, 234_567_890);
	    assert_eq!(d.subsecs(), 234_567_890);
	    assert_eq!(d.unsigned_subsecs(), 234_567_890);

	    let d = time::Duration::new(1, 234_567_890);
	    assert_eq!(d.subsecs(), 234_567_890);
	    assert_eq!(d.unsigned_subsecs(), 234_567_890);

	    // 負の値

	    let d = chrono::Duration::nanoseconds(-123_456_789);
	    assert_eq!(d.subsecs(), -123_456_789);
	    assert_eq!(d.unsigned_subsecs(), 123_456_789);

	    let d = std::time::Duration::new(0, 1_000_000_000 - 123_456_789);
	    assert_eq!(d.subsecs(), 1_000_000_000 - 123_456_789);
	    assert_eq!(d.unsigned_subsecs(), 1_000_000_000 - 123_456_789);

	    let d = time::Duration::new(0, 1_000_000_000 - 123_456_789);
	    assert_eq!(d.subsecs(), 1_000_000_000 - 123_456_789);
	    assert_eq!(d.unsigned_subsecs(), 1_000_000_000 - 123_456_789);
}

#[test]
pub(crate) fn test_fmt_fract_all() {
	    let d = chrono::Duration::milliseconds(1234);
	    assert_eq!(d.fmt_fract_all(), "234000000");

	    let d = chrono::Duration::nanoseconds(123_456_789);
	    assert_eq!(d.fmt_fract_all(), "123456789");

	    let d = std::time::Duration::new(0, 123_456_789);
	    assert_eq!(d.fmt_fract_all(), "123456789");

	    let d = time::Duration::new(0, 123_456_789);
	    assert_eq!(d.fmt_fract_all(), "123456789");

	    // より複雑なテスト

	    let d = chrono::Duration::nanoseconds(1_234_567_890);
	    assert_eq!(d.fmt_fract_all(), "234567890");

	    let d = std::time::Duration::new(1, 234_567_890);
	    assert_eq!(d.fmt_fract_all(), "234567890");

	    let d = time::Duration::new(1, 234_567_890);
	    assert_eq!(d.fmt_fract_all(), "234567890");

	    // 負の値

	    let d = chrono::Duration::nanoseconds(-123_456_789);
	    assert_eq!(d.fmt_fract_all(), "123456789");

	    let d = std::time::Duration::new(0, 1_000_000_000 - 123_456_789);
	    assert_eq!(d.fmt_fract_all(), "876543211");

	    let d = time::Duration::new(0, 1_000_000_000 - 123_456_789);
	    assert_eq!(d.fmt_fract_all(), "876543211");
}
