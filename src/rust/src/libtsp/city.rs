extern crate rustc_serialize;

use rustc_serialize::json::{Json, ToJson};
use std::fmt;
use std::collections::BTreeMap;

#[derive(Clone, RustcEncodable, RustcDecodable)]
pub struct City{
	pub name: String,
	pub coordinates: (f64, f64),
}

impl PartialEq for City {
	fn eq(&self, other: &City) -> bool {
		self.coordinates.0 == other.coordinates.0 && self.coordinates.1 == other.coordinates.1
	}

	fn ne(&self, other: &City) -> bool {
		self.coordinates.0 != other.coordinates.0 || self.coordinates.1 != other.coordinates.1
	}
}

impl fmt::Display for City {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "Name:{},X:{},Y:{}", self.name, self.coordinates.0, self.coordinates.1)
	}
}


impl ToJson for City {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("name".to_string(), self.name.to_json());
        d.insert("coordinates".to_string(), self.coordinates.to_json());
        Json::Object(d)
    }
}