extern crate rand;

use std::mem::swap;

use graph::Graph;
use population::Population;
use population::PopulationBuilder;
use rand::Rng;
use tour::Tour;
use tour::TourBuilder;

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
    _elitism: bool,
}

impl TSP {

    /// New travelling salesman constructor
    pub fn new(routes: Population, cities: Graph, tournament_size: usize, mutation_rate: f64, elitism: bool) -> TSP {
        TSP {
            routes: routes,
            cities: cities,
            tournament_size: tournament_size,
            mutation_rate: mutation_rate,
            _elitism: elitism,
        }
    }

    pub fn compute(&mut self) {
        
        let population_size = self.routes.get_population_size();
        let mut evolved_routes: Population = PopulationBuilder::new()
                                .generate_empty_with_size(population_size)
                                .finalize();

        // lets save the fittest in case
        let mut elite_offset = 0;
        if self._elitism {
            evolved_routes.save_tour(self.routes.get_fittest());
            elite_offset = 1;
        }

        // Time to evolve the current population set
        while elite_offset < population_size {

            // Select
            let parent_1: Tour = self.tournament_selection();
            let parent_2: Tour = self.tournament_selection();

            // Crossover
            let mut child: Tour = self.crossover(parent_1, parent_2);

            // Mutate
            child = self.mutate(child);

            // Save the result
            evolved_routes.save_tour(child);

            elite_offset += 1;
        }

        self.routes = evolved_routes;
    }

    pub fn get_fittest_result(&mut self) -> Tour{
        self.routes.get_fittest()
    }

    /// Generates a rate between 0 and 1
    fn generate_random_rate(&self) -> f64 {
        (rand::thread_rng().gen_range(1, 101) / 100) as f64 
    }
}

impl GA for TSP {

    /// Select the fittest tours to be parents for crossover
    fn tournament_selection(&mut self) -> Tour {

        let population_size: usize = self.routes.get_population_size();

        let mut tournament: Population = PopulationBuilder::new()
                                                            .generate_empty_with_size(self.tournament_size)
                                                            .finalize();

        for _ in 0..self.tournament_size {
            let random_selection = rand::thread_rng().gen_range(0, population_size);
            tournament.save_tour(self.routes.get_tour(random_selection));
        }

        tournament.get_fittest()
    }

    /// TODO Crossover two parents 
    fn crossover(&self, parent_1: Tour, parent_2: Tour) -> Tour {

        let graph_size: usize = self.cities.get_graph_size();
        // Easier way is to clone a parent and change the tours that will be crossed
        let mut child: Tour = TourBuilder::new()
                                .generate_empty_with_size(graph_size)
                                .finalize();
        
        let mut start_city_index = rand::thread_rng().gen_range(0, graph_size);
        let mut last_city_index = rand::thread_rng().gen_range(0, graph_size);

        if last_city_index > start_city_index {
            swap(&mut start_city_index, &mut last_city_index);
        }

        let mut city_found: bool = false;
        // Comment-start: Ugly but does the trick, TODO should apply rust native methods and optimize performance
        // Get parent 2 heritage, i.e. the cities that are not part of parent 1 heritage (crossover point)
        for it in 0..graph_size {
            // Search parent_2 for the cities that are not part of parent_1 heritage (between start_city_index and last_city_index)
            for it2 in start_city_index..last_city_index {
                if parent_2.get_city(it) == parent_1.get_city(it2) {
                    city_found = true;
                    break;
                }
            }
            if !city_found {
                child.save_city(parent_2.get_city(it));
            } 
            city_found = false;
        }

        //Now that we have parent_2 heritage we just need to insert in parent_1's by inserting and pushing the remaining to the right
        for it in start_city_index..last_city_index {
            child.insert_city_at_index(it, parent_1.get_city(it));
        }
        // Comment-end

        assert_eq!(true, child.get_tour_size() == graph_size);

        child
    }

    /// Mutate the candidate tour by swapping a random city in the tour
    fn mutate(&self, mut tour: Tour) -> Tour {

        let graph_size: usize = self.cities.get_graph_size();
        
        for it in 0..graph_size {
            if self.generate_random_rate() < self.mutation_rate {
                // Changing a random tour
                let random_tour_index = rand::thread_rng().gen_range(0, graph_size);

                // Swap the random_tour_index with the current loop index
                tour.alter_swap(random_tour_index, it);
            }
        }

        tour
    }
}
