use tour::Tour;
use tour::HasFitness;

#[derive(Clone)]
pub struct Population {
    tours: Vec<Tour>,
}

pub struct PopulationBuilder {
	tours: Vec<Tour>,
}

impl Population {

	/// Constructor for an empty population with allocated capacity
	pub fn new(population_size: usize) -> Population {
		Population {
			tours: Vec::with_capacity(population_size),
		}
	}

	/// Go through all the tours and return the one with best fitness
	pub fn get_fittest(&self) -> Tour {

		let mut fittest: Tour = self.tours[0].clone();

		for it in 0..self.tours.len() {
			if fittest.get_fitness() < self.tours[it].get_fitness() {
				fittest = self.tours[it].clone();
			}
		}		

		fittest
	}

	pub fn get_population_size(&self) -> usize {
		self.tours.len()
	}

	pub fn save_tour(&mut self, tour: Tour) {
		let new_tour = tour;
		self.tours.push(new_tour);
	}

	pub fn get_tour(&mut self, index: usize) -> Tour {
		self.tours[index].clone()
	}

}

impl PopulationBuilder {
	pub fn new() -> PopulationBuilder {
		PopulationBuilder {
			tours: Vec::new(),
		}
	}

	pub fn generate_random_population(&mut self, population_size: usize) -> &mut PopulationBuilder {

    	self
    }

    pub fn generate_empty_with_size(&mut self, population_size: usize) -> &mut PopulationBuilder {
    	self.tours = Vec::with_capacity(population_size);
    	self
    }

    pub fn finalize(&self) -> Population {
        Population { 
        	tours: self.tours.clone(),
        }
    }
}


