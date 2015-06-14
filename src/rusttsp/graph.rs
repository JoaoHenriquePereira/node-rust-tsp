extern crate rand;

use rustc_serialize::json::{self, Json, ToJson};
use rusttsp::city::City;
use rand::Rng;
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct Graph {
    cities: Vec<City>,
}

impl Graph {

    pub fn get_graph_size(&self) -> usize {
    	self.cities.len()
    }

    pub fn get_map(&mut self) -> Vec<City> {
    	self.cities.clone()
    }

    fn get_node_at_index(&self, it: usize) -> City {
        self.cities[it]
    }

}

pub struct GraphBuilder {
    cities: Vec<City>,
}

impl ToJson for Graph {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("cities".to_string(), self.cities.to_json());
        Json::Object(d)
    }
}

impl GraphBuilder {

    /// Constructor for an empty population
	pub fn new() -> GraphBuilder {
		GraphBuilder {
			cities: Vec::new(),
		}
	}

    //Generate a random graph, assuming we always have positive values for x and y on a map
	pub fn generate_random_graph(&mut self, graph_size: usize, max_x: f64, max_y: f64) -> &mut GraphBuilder {
        (0..graph_size).map(|i| {
            let city = City {
                            name: "Node", 
                            x: rand::thread_rng().gen_range(0.0, max_x), 
                            y: rand::thread_rng().gen_range(0.0, max_y),
                        };
            self.cities.push(city);
        });
        self
    }

    /// Terminates construction and returns instance
    pub fn finalize(&self) -> Graph {
        Graph { 
        	cities: self.cities.clone(),
        }
    }

}


