extern crate rustc_serialize;

use rustc_serialize::json::{self, Json, ToJson};
use std::fmt;
use std::collections::BTreeMap;

#[derive(Clone, Copy)]
pub struct City{
	pub name: &'static str,
	pub x: f64,
	pub y: f64,
}

impl PartialEq for City {
	fn eq(&self, other: &City) -> bool {
		self.x == other.x && self.y == other.y
	}

	fn ne(&self, other: &City) -> bool {
		self.x != other.x || self.y != other.y
	}
}

impl fmt::Display for City {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "Name:{},X:{},Y:{}", self.name, self.x, self.y)
	}
}


impl ToJson for City {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("name".to_string(), self.name.to_json());
        d.insert("x".to_string(), self.x.to_json());
        d.insert("y".to_string(), self.y.to_json());
        Json::Object(d)
    }
}