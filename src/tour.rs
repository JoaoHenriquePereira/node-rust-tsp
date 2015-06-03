extern crate rand;

use city::City;
use graph::Graph;
use rand::{thread_rng, Rng};

pub trait HasFitness {
    fn calc_fitness(&mut self) -> f64;
}

pub trait IsValidTSPTour {
	fn is_valid_tsp_tour(tour: Tour, graph: Graph) -> bool;
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

	pub fn insert_city_at_index(&mut self, index: usize, city: City) {
		self.tour.insert(index, city);
	}

	pub fn get_city(&self, index: usize) -> City {
		self.tour[index].clone()
	}

	pub fn get_tour_size(&self) -> usize {
		self.tour.len()
	}

	pub fn sub_tour_between_index(&mut self, start_index: usize, end_index: usize) -> Vec<City> {
		(start_index..end_index).map(|i| {println!("{}",i);self.tour[i]}).collect::<Vec<City>>()
	}

	/// Warning: Method is tightly coupled with the interface but remains cohesive
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

impl IsValidTSPTour for Tour {
	/// True if tour contains all cities in the graph and no repeated nodes
	fn is_valid_tsp_tour(tour: Tour, mut graph: Graph) -> bool {

		let tour_size: usize = tour.get_tour_size();
		let graph_size: usize = graph.get_graph_size();

		assert_eq!(true, tour_size == graph_size);

		let map: Vec<City> = graph.get_map();

		let mut counter: u8 = 0;

		//Shameful O(n^2)
		for it in 0..graph_size {
			for it2 in 0..tour_size {
				if map[it] == tour.get_city(it2){
					counter += 1; 
				}
			}
			assert_eq!(1, counter);
			counter = 0;
		}

		true
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

	pub fn generate_random_tour(&mut self, mut graph: Graph) -> &mut TourBuilder {
		self.tour = graph.get_map();
		thread_rng().shuffle(&mut self.tour);
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