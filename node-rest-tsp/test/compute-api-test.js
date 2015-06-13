// node-rest-tsp 0.0.1
// Exposing rust-tsp via nodejs rest API
// Repo: https://github.com/JoaoHenriquePereira/node-rest-tsp

//
// Compute Controller Test
//

var chai 			= require('chai');
	expect 			= require('chai').expect,
	hal 			= require('hal');
	pjson 			= require('../package.json');
	supertest 		= require('supertest'),
	api 			= supertest('http://localhost:8080');

chai.use(require('chai-json-schema'));



describe('compute', function() {

  	it('Must filter input for bad requests', function(done) {
  		
  		var entry_point = '/'+pjson.name+'/compute'
  		var invalid_version = '9'+pjson.version;

  		var expected_json_schema = require('../schemas/compute-api-schema-output.json');

  		var bad_request_json = {
  			"graph_type": "u2d-cartesian",	//Unsigned 2d-cartesian coordinates
  			"graph": [
  				{ "name": "1", "coordinates": [ "41.1621", "8.6220" ] }, //Not enough nodes and wrong type
  			],
  			"options": {
  				"mutation_rate": 0.015,
  				"elitism": true,
  				"population_size": 30,
  				"tournament_size": 5
  			}
  		}

  		var bad_request_bounds_json = {
  			"graph_type": "u2d-cartesian",	//Unsigned 2d-cartesian coordinates
  			"graph": [
  				{ "name": "1", "coordinates": [ 41.1621, 8.6220 ] },
				{ "name": "2", "coordinates": [ 40.4000, 3.7167 ] },
				{ "name": "3", "coordinates": [ 13.7563, 100.5018 ] }, 
				{ "name": "4", "coordinates": [ 33.9253, 18.4239 ] }, 
				{ "name": "5", "coordinates": [ 37.7833, 122.4167 ] }, 
				{ "name": "6", "coordinates": [ 37.3544, 121.9692 ] }, 
				{ "name": "7", "coordinates": [ 23.5500, 46.6333 ] }, 
				{ "name": "8", "coordinates": [ 33.4500, 70.6667 ] }, 
				{ "name": "9", "coordinates": [ 32.6500, 16.9167 ] }, 
				{ "name": "10", "coordinates": [ 48.2000, 16.3667 ] },
				{ "name": "11", "coordinates": [ 48.2012, 34.3667 ] }
  			],
  			"options": {
  				"mutation_rate": 0.015,
  				"elitism": true,
  				"population_size": 300000000000000,	// TODO must test the remaining optinal bounds
  				"tournament_size": 5
  			}
  		}

		// Test bad request
		api.post(entry_point)
		.set('Accept', 'application/json')
		.set('Accept-Version', pjson.version)
		.send(bad_request_json)
		.expect(400)
		.end( function(err, res) {
			if (err) return done(err);
			expect(res.body).to.be.jsonSchema(expected_json_schema);
		});

		// Test bad version
		api.post(entry_point)
		.set('Accept', 'application/json')
		.set('Accept-Version', invalid_version)
		.send(bad_request_bounds_json)
		.expect(400)
		.end( function(err, res) {
			if (err) return done(err);
		});
		done();
  	});

	

	it('Must allow access to result', function(done) {

		var expected_compute_json_schema = require('../schemas/compute-api-schema-output.json');
  		var expected_result_json_schema = require('../schemas/result-api-schema-output.json');
		var compute_entry_point = '/'+pjson.name+'/compute';
		var result_entry_point = '/'+pjson.name+'/result';

		var good_request_json = {
  			"graph_type": "u2d-cartesian",	//Unsigned 2d-cartesian coordinates
  			"graph": [
  				{ "name": "1", "coordinates": [ 41.1621, 8.6220 ] },
				{ "name": "2", "coordinates": [ 40.4000, 3.7167 ] },
				{ "name": "3", "coordinates": [ 13.7563, 100.5018 ] }, 
				{ "name": "4", "coordinates": [ 33.9253, 18.4239 ] }, 
				{ "name": "5", "coordinates": [ 37.7833, 122.4167 ] }, 
				{ "name": "6", "coordinates": [ 37.3544, 121.9692 ] }, 
				{ "name": "7", "coordinates": [ 23.5500, 46.6333 ] }, 
				{ "name": "8", "coordinates": [ 33.4500, 70.6667 ] }, 
				{ "name": "9", "coordinates": [ 32.6500, 16.9167 ] }, 
				{ "name": "10", "coordinates": [ 48.2000, 16.3667 ] },
				{ "name": "11", "coordinates": [ 48.2012, 34.3667 ] }
  			],
  			"options": {
  				"mutation_rate": 0.015,
  				"elitism": true,
  				"population_size": 30,	// TODO must test the remaining optinal bounds
  				"tournament_size": 5
  			},
  		}

		// Test good request
		api.post(compute_entry_point)
		.set('Accept', 'application/json')
		.set('Accept-Version', pjson.version)
		.send(good_request_json)
		.expect(200)
		.end( function(err, res) {
			if (err) return done(err);
			expect(res.body).to.be.jsonSchema(expected_compute_json_schema);
			//Get the result
			api.get(result_entry_point+'/'+res.body.code)
			.set('Accept', 'application/json')
			.set('Accept-Version', pjson.version)
			.expect(200)
			.end( function(err, res) {
				if (err) return done(err);
				expect(res.body).to.be.jsonSchema(expected_result_json_schema);
			});
		});

		done();
  	});

});