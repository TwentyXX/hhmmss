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
}
