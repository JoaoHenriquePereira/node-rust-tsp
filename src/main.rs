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

static MUTATION_RATE: f64 = 0.015;
static TOURNAMENT_SIZE: u64 = 5;
static ELITISM: bool = true;
static POPULATION_SIZE: u64 = 30;
static GRAPH_SIZE: u64 = 8;

fn main() {
	
	let cities: Graph = GraphBuilder::new()
								.generate_random_graph(GRAPH_SIZE)
								.finalize();
	let init_routes: Population = Population::generate_random_population(POPULATION_SIZE);
	
	let tsp = TSP::new(init_routes, cities, TOURNAMENT_SIZE, MUTATION_RATE, ELITISM);

}
