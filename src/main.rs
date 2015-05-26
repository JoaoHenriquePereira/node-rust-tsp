extern crate rand;

mod tsp;
mod graph;
mod city;
mod population;
mod tour;

use tsp::TSP;
use graph::Graph;
use graph::GraphBuilder;
use population::Population;
use population::PopulationBuilder;

static MUTATION_RATE: f64 = 0.015;
static TOURNAMENT_SIZE: usize = 5;
static ELITISM: bool = true;
static POPULATION_SIZE: usize = 30;
static GRAPH_SIZE: usize = 8;
static RUN_SIZE: usize = 100;

fn main() {
	
	let cities: Graph = GraphBuilder::new()
								.generate_random_graph(GRAPH_SIZE)
								.finalize();

	let init_routes: Population = PopulationBuilder::new()
								.generate_random_population(POPULATION_SIZE)
								.finalize();
	
	let mut tsp = TSP::new(init_routes, cities, TOURNAMENT_SIZE, MUTATION_RATE, ELITISM);

	for _ in 0..RUN_SIZE {
		tsp.compute();
	}

	

}
