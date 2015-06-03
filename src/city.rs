use std::fmt;

#[derive(Clone, Copy)]
pub struct City(pub f64,pub f64);

impl PartialEq for City {
	fn eq(&self, other: &City) -> bool {
		self.0 == other.0 && self.1 == other.1
	}

	fn ne(&self, other: &City) -> bool {
		self.0 != other.0 || self.1 != other.1
	}
}

impl fmt::Display for City {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "X:{},Y:{}", self.0, self.1)
	}
}