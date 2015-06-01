//! A Travelling salesman problem solved with genetic algorithm written in Rust.
//!
//! This project aims to solve the travelling salesman problem (best routing solution between nodes) 
//! by applying a genetic algorithm to solve it. I did it for myself as practice and to learn a bit 
//! more about Rust. I also write about it in my blog http://codeandunicorns.com, feel free to contact
//! me directly.
//!
//! # Problem
//!
//! Given a 
//! 
//!
//! Lets say we have a 2D graph of cities and we want to know what's the best way to visit each one.
//! This is the called travelling salesman problem and a NP-HARD problem. As the graph grows the harder
//! it is to compute the best route possible. Here I solve it (in a small scale) by applying a genetic algorithm.
//!
//! GA mimics the behavior of chromosomes in which they evolve and cross traits between them. In our case:
//!
//! Chromosome - A possible route solution or a tour along all nodes.
//! Population - Our "population" of solutions
//! 
//! Tour - A tour has a fitness value given by the total magnitude distance between cities:
//! Pseudo-code
//! ```
//! for city_iterator in 0..TOUR_SIZE {//Or GRAPH_SIZE
//!     x = x_city_to - x_city_from 
//!		y = y_city_to - y_city_from 
//!		distance = sqrt(x^2 + y^2)
//!		final_distance += distance
//! }
//! ```
//!

extern crate rand;

mod city;
mod graph;
mod population;
mod tests;
mod tour;
mod tsp;

use graph::Graph;
use graph::GraphBuilder;
use population::Population;
use population::PopulationBuilder;
use tsp::TSP;

static ELITISM: bool = true;
static GRAPH_SIZE: usize = 10;
static MAX_X: f64 = 100.0;
static MAX_Y: f64 = 100.0;
static MUTATION_RATE: f64 = 0.015;
static POPULATION_SIZE: usize = 30;
static RUN_SIZE: usize = 100;
static TOURNAMENT_SIZE: usize = 5;

fn main() {
	
	let cities: Graph = GraphBuilder::new()
								.generate_random_graph(GRAPH_SIZE, MAX_X, MAX_Y)
								.finalize();

	let init_routes: Population = PopulationBuilder::new()
								.generate_random_population(cities.clone(), POPULATION_SIZE)
								.finalize();
	

	let mut tsp = TSP::new(init_routes, cities, TOURNAMENT_SIZE, MUTATION_RATE, ELITISM);

	for _ in 0..RUN_SIZE {
		tsp.compute();
	}

}