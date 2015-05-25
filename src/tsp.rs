extern crate rand;

use rand::Rng;

use population::Population;
use population::PopulationBuilder;
use graph::Graph;
use tour::Tour;


trait GA {
    fn tournament_selection(&mut self) -> Tour;
    fn crossover(&self, parent_1: Tour, parent_2: Tour) -> Tour;
    fn mutate(&self, tour: Tour) -> Tour;
}

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

    fn crossover(&self, parent_1: Tour, parent_2: Tour) -> Tour {

        let graph_size: usize = self.cities.get_graph_size();
        let tour: Tour = Tour::new(graph_size);
        
        let starting_city_index = rand::thread_rng().gen_range(0, graph_size - 1);
        let last_city_index = rand::thread_rng().gen_range(0, graph_size - 1);

        for it in 0..graph_size - 1 {

        }

        tour
    }

    fn mutate(&self, tour: Tour) -> Tour {

        let graph_size: usize = self.cities.get_graph_size();
        let tour: Tour = Tour::new(graph_size);

        /*
        // Loop through tour cities
        for(int tourPos1=0; tourPos1 < tour.tourSize(); tourPos1++){
            // Apply mutation rate
            if(Math.random() < mutationRate){
                // Get a second random position in the tour
                int tourPos2 = (int) (tour.tourSize() * Math.random());

                // Get the cities at target position in tour
                City city1 = tour.getCity(tourPos1);
                City city2 = tour.getCity(tourPos2);

                // Swap them around
                tour.setCity(tourPos2, city1);
                tour.setCity(tourPos1, city2);
            }
        }
        */
        for it in 0..graph_size - 1 {
            if self.generate_random_rate() < self.mutation_rate {

            }
        }

        tour

    }

}
