extern crate rand;

use rand::Rng;

use population::Population;
use population::PopulationBuilder;
use graph::Graph;
use tour::Tour;

/// Genetic algorithm interface definition
trait GA {
    fn tournament_selection(&mut self) -> Tour;
    fn crossover(&self, parent_1: Tour, parent_2: Tour) -> Tour;
    fn mutate(&self, tour: Tour) -> Tour;
}

/// Travelling salesman problem structure definition
pub struct TSP {
    routes: Population,
    cities: Graph,
    tournament_size: usize,
    mutation_rate: f64,
    elitism: bool,
}

impl TSP {

    /// New travelling salesman constructor
    pub fn new(routes: Population, cities: Graph, tournament_size: usize, mutation_rate: f64, elitism: bool) -> TSP {
        TSP {
            routes: routes,
            cities: cities,
            tournament_size: tournament_size,
            mutation_rate: mutation_rate,
            elitism: elitism,
        }
    }

    /// Generates a rate between 0 and 1
    fn generate_random_rate(&self) -> f64 {
        (rand::thread_rng().gen_range(1, 101) / 100) as f64 
    }

    pub fn compute(&mut self) {

    }

}

impl GA for TSP {

    /// Select the fittest tours to be parents for crossover
    fn tournament_selection(&mut self) -> Tour {

        let population_size: usize = self.routes.get_population_size();

        let mut tournament: Population = PopulationBuilder::new()
                                                            .generate_empty_with_size(self.tournament_size)
                                                            .finalize();

        for it in 0..self.tournament_size {
            let random_selection = rand::thread_rng().gen_range(0, population_size - 1);
            tournament.save_tour(self.routes.get_tour(random_selection));
        }

        tournament.get_fittest()

    }

    /// Crossover two parents 
    fn crossover(&self, parent_1: Tour, parent_2: Tour) -> Tour {

        let graph_size: usize = self.cities.get_graph_size();
        let tour: Tour = Tour::new(graph_size);
        
        let starting_city_index = rand::thread_rng().gen_range(0, graph_size - 1);
        let last_city_index = rand::thread_rng().gen_range(0, graph_size - 1);

        for it in 0..graph_size - 1 {

        }

        tour
    }

    /// Mutate the candidate tour by swapping a random city in the tour
    fn mutate(&self, tour: Tour) -> Tour {

        let graph_size: usize = self.cities.get_graph_size();
        let maybe_mutated_tour: Tour = Tour::new(graph_size);

        for it in 0..graph_size - 1 {
            if self.generate_random_rate() < self.mutation_rate {
                //Changing a random tour
                let random_tour_index = rand::thread_rng().gen_range(0, graph_size - 1);

                //Get the random_tour_city 

                //Swap the random_tour_index with the current loop index

            }
        }

        maybe_mutated_tour

    }

}
