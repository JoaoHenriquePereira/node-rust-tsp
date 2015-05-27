extern crate rand;

mod city;
mod graph;
mod population;
mod tour;
mod tsp;

use graph::Graph;
use graph::GraphBuilder;
use population::Population;
use population::PopulationBuilder;
use tsp::TSP;

static ELITISM: bool = true;
static GRAPH_SIZE: usize = 10;
static MUTATION_RATE: f64 = 0.015;
static POPULATION_SIZE: usize = 30;
static RUN_SIZE: usize = 100;
static TOURNAMENT_SIZE: usize = 5;

fn main() {
	
	let cities: Graph = GraphBuilder::new()
								.generate_random_graph(GRAPH_SIZE)
								.finalize();

	let init_routes: Population = PopulationBuilder::new()
								.generate_random_population(cities.clone(), POPULATION_SIZE)
								.finalize();
	
	let mut tsp = TSP::new(init_routes, cities, TOURNAMENT_SIZE, MUTATION_RATE, ELITISM);

	for _ in 0..RUN_SIZE {
		tsp.compute();
	}

}
