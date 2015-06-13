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


pub mod city;
pub mod graph;
pub mod population;
pub mod tests;
pub mod tour;
pub mod tsp;

/*
use getopts::{Options, Matches};
use graph::Graph;
use graph::GraphBuilder;
use population::Population;
use population::PopulationBuilder;
use rustc_serialize::{Decodable, Encodable, json};
use std::env;
use tour::HasFitness;
use tour::Tour;
use tour::TourBuilder;
use tsp::TSP;

static ELITISM: bool = true;
static GRAPH_SIZE: usize = 10;
static MAX_X: f64 = 100.0;
static MAX_Y: f64 = 100.0;
static MUTATION_RATE: f64 = 0.015;
static POPULATION_SIZE: usize = 30;
static RUN_SIZE: usize = 100;
static TOURNAMENT_SIZE: usize = 5;

fn do_work(inp: &str, out: Option<String>) {
    println!("{}", inp);
    match out {
        Some(x) => println!("{}", x),
        None => println!("No Output"),
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {

	/* Handle input */
	
	let cities: Graph = GraphBuilder::new()
								.generate_random_graph(GRAPH_SIZE, MAX_X, MAX_Y)
								.finalize();

	let mut init_tours: Population = PopulationBuilder::new()
								.generate_random_population(cities.clone(), POPULATION_SIZE)
								.finalize();

	let mut fittest_solution: Tour = init_tours.get_fittest();

	let model: MongoModel = MongoModel::new();

	/*let mut tsp = TSP::new(init_tours, cities, TOURNAMENT_SIZE, MUTATION_RATE, ELITISM);

	println!("Tour: {}", fittest_solution);
	println!("Fittest initial solution {}", fittest_solution.calc_fitness());

	for _ in 0..RUN_SIZE {
		tsp.compute();
	}

	fittest_solution = tsp.get_fittest_result();

	println!("Tour: {}", fittest_solution);
	println!("Fittest final solution {}", fittest_solution.calc_fitness());*/

}*/
