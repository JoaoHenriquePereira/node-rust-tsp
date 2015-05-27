extern crate rand;

use std::mem::swap;

use city::City;
use rand::Rng;

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

    fn add_node(&mut self, node: City) {
        self.cities.push(node);
    }

}

pub struct GraphBuilder {
    cities: Vec<City>,
}

impl GraphBuilder {

	pub fn new() -> GraphBuilder {
		GraphBuilder {
			cities: Vec::new(),
		}
	}

	pub fn generate_random_graph(&mut self, graph_size: usize) -> &mut GraphBuilder {
        for _ in 0..graph_size {
            let city = City(rand::thread_rng().gen_range(0.0, 100.0), rand::thread_rng().gen_range(0.0, 100.0));
            self.cities.push(city);
        }
    	self
    }

    pub fn finalize(&self) -> Graph {
        Graph { 
        	cities: self.cities.clone(),
        }
    }

}
