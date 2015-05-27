use city::City;
use graph::Graph;

pub trait HasFitness {
    fn calc_fitness(&mut self) -> f64;
}

#[derive(Clone)]
pub struct Tour {
    tour: Vec<City>,
    fitness: f64,
}

impl Tour {

	pub fn alter_swap(&mut self, from_swap_index: usize, to_swap_index: usize) {
		self.tour.swap(from_swap_index, to_swap_index);
	}

	pub fn save_city(&mut self, city: City) {
		self.tour.push(city);
	}

	pub fn get_city(&self, index: usize) -> City {
		self.tour[index].clone()
	}

	/// Warning: Method is tightly coupled with the interface
	fn get_distance(&mut self) -> f64 {

		let mut distance: f64 = 0.0;

		for it in 0..self.tour.len() {

			let from_city: City = self.tour[it].clone();

			let mut to_city;

			if (it + 1) < self.tour.len() {
				to_city = self.tour[it + 1].clone(); 
			} else {
				to_city = self.tour[0].clone();
			}

			let x_dist = (from_city.0 - to_city.0).abs();
			let y_dist = (from_city.1 - to_city.1).abs();
			distance += ( (x_dist * x_dist) + (y_dist * y_dist) ).sqrt();

		}

		distance
	}
}

impl HasFitness for Tour {
    
	fn calc_fitness(&mut self) -> f64 {
		if self.fitness == 0.0 {
			self.fitness = 1.0 / self.get_distance();
		}

		self.fitness
	}

}

pub struct TourBuilder {
	tour: Vec<City>,
    fitness: f64,
}

impl TourBuilder {
	pub fn new() -> TourBuilder {
		TourBuilder {
			tour: Vec::new(),
			fitness: 0.0,
		}
	}

	pub fn generate_random_tour(&mut self, graph: Graph) -> &mut TourBuilder {

    	self
    }

    pub fn generate_empty_with_size(&mut self, tour_size: usize) -> &mut TourBuilder {
    	self.tour = Vec::with_capacity(tour_size);
    	self
    }

    pub fn finalize(&self) -> Tour {
        Tour { 
        	tour: self.tour.clone(),
        	fitness: self.fitness,
        }
    }
}