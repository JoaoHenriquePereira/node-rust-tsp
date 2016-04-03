//! Unit tests module
//! Ensure the critical operations pass testing
//! Status: OnGoing
//! Guidelines from: https://doc.rust-lang.org/book/testing.html

extern crate rusttsp;

use rusttsp::libtsp::graph::Graph;
use rusttsp::libtsp::graph::GraphBuilder;

use rusttsp::libtsp::population::Population;
use rusttsp::libtsp::population::PopulationBuilder;

use rusttsp::libtsp::tour::Tour;
use rusttsp::libtsp::tour::IsValidTSPTour;

static GRAPH_SIZE: usize = 100;
static MAX_X: f64 = 100.0;
static MAX_Y: f64 = 100.0;
static POPULATION_SIZE: usize = 30;

///Graph
#[test]
fn random_graph_generation() {

    let mut graph: Graph = GraphBuilder::new()
									.generate_random_graph(GRAPH_SIZE, MAX_X, MAX_Y)
									.finalize();
	
	// Generated a Vec<City> with the desired size
	assert_eq!(true, graph.get_graph_size() == GRAPH_SIZE);
	
	// The content is a collection of random tuples
	let graph_test = graph.get_map();
	
	for it in 0..graph_test.len() {
		// x
		assert_eq!(true, graph_test[it].coordinates.0 > 0.0);
		assert_eq!(true, graph_test[it].coordinates.1 > 0.0);

		// y
		assert_eq!(true, graph_test[it].coordinates.0 < MAX_X);
		assert_eq!(true, graph_test[it].coordinates.1 < MAX_Y);
	}
}


///Population
#[test]
fn random_graph_population() {

    let graph: Graph = GraphBuilder::new()
									.generate_random_graph(GRAPH_SIZE, MAX_X, MAX_Y)
									.finalize();
	
	let mut population: Population = PopulationBuilder::new()
								.generate_random_population(graph.clone(), POPULATION_SIZE)
								.finalize();

	// Generated a Vec<Tour> with the desired size
	assert_eq!(true, population.get_population_size() == POPULATION_SIZE);

	// Check if cities in the tour exist in graph and unique
	for it in 0..population.get_population_size() {
		let candidate_tour: Tour = population.get_tour(it);
		assert_eq!(true, Tour::is_valid_tsp_tour(candidate_tour, graph.clone()));
	}
}
