pub enum FractPartOfDuration {
	Milliseconds,
	Microseconds,
	Nanoseconds,
}

impl FractPartOfDuration {
	pub fn units_per_sec(&self) -> u32 {
		match self {
			FractPartOfDuration::Milliseconds => 1_000,
			FractPartOfDuration::Microseconds => 1_000_000,
			FractPartOfDuration::Nanoseconds => 1_000_000_000,
		}
	}
	pub fn decimal_places(&self) -> usize {
		match self {
			FractPartOfDuration::Milliseconds => 3,
			FractPartOfDuration::Microseconds => 6,
			FractPartOfDuration::Nanoseconds => 9,
		}
	}
}
