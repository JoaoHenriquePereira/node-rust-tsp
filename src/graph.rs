use city::City;

pub struct Graph {
    cities: Vec<City>,
}

impl Graph {

    fn add_node(&mut self, node: City) {
    	self.cities.push(node);
    }

    pub fn get_graph_size(&self) -> usize {
    	self.cities.len()
    }

    pub fn get_map(&mut self) -> Vec<City> {
    	self.cities.clone()
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

    	self
    }

    pub fn finalize(&self) -> Graph {
        Graph { 
        	cities: self.cities.clone(),
        }
    }

}
