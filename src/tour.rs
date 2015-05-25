use city::City;

pub trait HasFitness {
    fn get_fitness(&self) -> f64;
}

#[derive(Clone)]
pub struct Tour {
    tour: Vec<City>,
    fitness: f64,
}

impl Tour {

	pub fn new (graph_size: usize) -> Tour {
		Tour {
			tour: Vec::with_capacity(graph_size),
			fitness: 0.0,
		}
	}


}

impl HasFitness for Tour {
    
	fn get_fitness(&self) -> f64 {
		0.0
	}

}