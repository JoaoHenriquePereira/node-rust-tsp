extern crate rand;

use rustc_serialize::json::{Json, ToJson};
use libtsp::city::City;
use rand::Rng;
use std::collections::BTreeMap;

#[derive(Clone, RustcEncodable, RustcDecodable)]
pub struct Graph {
    nodes: Vec<City>,
}

impl Graph {

    pub fn get_graph_size(&self) -> usize {
    	self.nodes.len()
    }

    pub fn get_map(&mut self) -> Vec<City> {
    	self.nodes.clone()
    }
    
}

pub struct GraphBuilder {
    nodes: Vec<City>,
}

impl ToJson for Graph {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("nodes".to_string(), self.nodes.to_json());
        Json::Object(d)
    }
}

impl GraphBuilder {

    /// Constructor for an empty population
	pub fn new() -> GraphBuilder {
		GraphBuilder {
			nodes: Vec::new(),
		}
	}

    //Generate a random graph, assuming we always have positive values for x and y on a map
	pub fn generate_random_graph(&mut self, graph_size: usize, max_x: f64, max_y: f64) -> &mut GraphBuilder {
        (0..graph_size).map(|_| {
            let city = City {
                            name: "Node".to_string(), 
                            coordinates: (rand::thread_rng().gen_range(0.0, max_x), rand::thread_rng().gen_range(0.0, max_y)),
                        };
            self.nodes.push(city);
        });
        self
    }

    /// Terminates construction and returns instance
    pub fn finalize(&self) -> Graph {
        Graph { 
        	nodes: self.nodes.clone(),
        }
    }

}


