use libtsp::graph::Graph;
use libtsp::tour::HasFitness;
use libtsp::tour::Tour;
use libtsp::tour::TourBuilder;

#[derive(Clone)]
pub struct Population {
    tours: Vec<Tour>,
}

impl Population {

	/// Go through all the tours and return the one with best fitness
	pub fn get_fittest(&mut self) -> Tour {

		let mut fittest: Tour = self.tours[0].clone();

		for it in 0..self.tours.len() {
			if fittest.calc_fitness() < self.tours[it].calc_fitness() {
				fittest = self.tours[it].clone();
			}
		}		

		fittest
	}

	pub fn get_population_size(&self) -> usize {
		self.tours.len()
	}

	pub fn get_tour(&mut self, index: usize) -> Tour {
		self.tours[index].clone()
	}

	pub fn save_tour(&mut self, tour: Tour) {
		self.tours.push(tour);
	}

}

pub struct PopulationBuilder {
	tours: Vec<Tour>,
}

impl PopulationBuilder {

	/// Constructor for an empty population
	pub fn new() -> PopulationBuilder {
		PopulationBuilder {
			tours: Vec::new(),
		}
	}

    /// Constructor for an empty population with allocated capacity
    pub fn generate_empty_with_size(&mut self, population_size: usize) -> &mut PopulationBuilder {
    	self.tours = Vec::with_capacity(population_size);
    	self
    }

    /// Constructor for generating a random population
	pub fn generate_random_population(&mut self, cities: Graph, population_size: usize) -> &mut PopulationBuilder {
		for _ in 0..population_size {
			self.tours.push(
					TourBuilder::new()
								.generate_random_tour(cities.clone())
								.finalize()
				);
		}
    	self
    }

    /// Terminates construction and returns instance
    pub fn finalize(&self) -> Population {
        Population { 
        	tours: self.tours.clone(),
        }
    }
}


