extern crate libc;
extern crate rand;
extern crate rustc_serialize;

pub mod libtsp;

use libc::c_char;
use rustc_serialize::json::{self, Json, ToJson};

use libtsp::graph::Graph;
use libtsp::population::Population;
use libtsp::population::PopulationBuilder;
use libtsp::tour::Tour;
use libtsp::tsp::TSP;

use std::collections::BTreeMap;
use std::ffi::{CString, CStr};
use std::str;

static RUN_SIZE: usize = 1000;

/// External adapter using ffi
#[no_mangle]
pub extern "C" fn compute_adapter(input: *const c_char) -> *const c_char {
    let c_input = unsafe { CStr::from_ptr(input).to_bytes() };
    match str::from_utf8(c_input) {
        Ok(input) => compute(input),
        Err(e) => CString::new(e.to_string()).unwrap().as_ptr(),
    }
}

// Actual handling
fn compute(input: &str) -> *const c_char {

    // Deserialize JSON data
    let decoded_input: Input = json::decode(input).unwrap();

    let init_tours: Population = PopulationBuilder::new()
                                .generate_random_population(decoded_input.graph.clone(), decoded_input.options.population_size)
                                .finalize();

    let mut tsp = TSP::new(init_tours, 
                        decoded_input.graph, 
                        decoded_input.options.tournament_size, 
                        decoded_input.options.mutation_rate, 
                        decoded_input.options.elitism);

    for _ in 0..RUN_SIZE {
        tsp.compute();
    }

    let fittest_solution: Tour = tsp.get_fittest_result();
    //let distance = fittest_solution.get_distance();
    /*
    
    println!("Tour: {}", fittest_solution);
    println!("Fittest final solution {}", fittest_solution.calc_fitness());

    */

    CString::new(json::encode(&fittest_solution).unwrap()).unwrap().as_ptr()
}

/// Aux input JSON structs
#[derive(RustcEncodable, RustcDecodable)]
struct Input {
	graph_type: String,
	graph: Graph,
	options: InputOptions,
}

#[derive(RustcEncodable, RustcDecodable)]
struct InputOptions {
	mutation_rate: f64,
	elitism: bool,
	population_size: usize,
	tournament_size: usize,
}

impl ToJson for Input {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("graph_type".to_string(), self.graph_type.to_json());
        d.insert("graph".to_string(), self.graph.to_json());
        d.insert("options".to_string(), self.options.to_json());
        Json::Object(d)
    }
}

impl ToJson for InputOptions {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("mutation_rate".to_string(), self.mutation_rate.to_json());
        d.insert("elitism".to_string(), self.elitism.to_json());
        d.insert("population_size".to_string(), self.population_size.to_json());
        d.insert("tournament_size".to_string(), self.tournament_size.to_json());
        Json::Object(d)
    }
}