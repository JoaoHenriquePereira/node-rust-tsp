'use strict';

// node-rest-tsp 0.0.1
// Exposing rust-tsp via nodejs rest API
// Repo: https://github.com/JoaoHenriquePereira/node-rest-tsp

//
// Compute Controller Test
//

const chai = require('chai');
const expect = require('chai').expect;
const hal = require('hal');
const pjson = require('../package.json');
const supertest = require('supertest');
const	api = supertest('http://localhost:8080');

chai.use(require('chai-json-schema'));

describe('compute', () => {
  it('Must filter input for bad requests', (done) => {
		const entryPoint = '/'+pjson.name+'/compute'
		const expectedSchema = require('../schemas/compute-api-schema-output.json');
		const badRequest = {
			graph: { nodes: [{ coordinates: [ 41.1621, 8.6220 ] }] },
			graphType: 'u2d-cartesian',	//Unsigned 2d-cartesian coordinates
			options: { mutationRate: 0.015, elitism: true, populationSize: 30, tournamentSize: 5 }
		}

		// Test bad request
		api.post(entry_point)
		.set('Accept', 'application/json')
		.set('Accept-Version', pjson.version)
		.send(badRequest)
		.expect(400)
		.end((err, res) => {
			if (err) return done(err);
			expect(res.body).to.be.jsonSchema(expectedSchema);
		});

		done();
  });



	it('Must allow access to result', function(done) {
		const expectedCompute = require('../schemas/compute-api-schema-output.json');
  	const expectedResult = require('../schemas/result-api-schema-output.json');
		const computeEntryPoint = '/'+pjson.name+'/compute';
		const resultEntryPoint = '/'+pjson.name+'/result';
		const goodRequest = {
			graphType: 'u2d-cartesian',	//Unsigned 2d-cartesian coordinates
			graph: {
				nodes: [
					{ name: '1', coordinates: [ 41.1621, 8.6220 ] },
					{ name: '2', coordinates: [ 40.4000, 3.7167 ] },
					{ name: '3', coordinates: [ 13.7563, 100.5018 ] },
					{ name: '4', coordinates: [ 33.9253, 18.4239 ] },
					{ name: '5', coordinates: [ 37.7833, 122.4167 ] },
					{ name: '6', coordinates: [ 37.3544, 121.9692 ] },
					{ name: '7', coordinates: [ 23.5500, 46.6333 ] },
					{ name: '8', coordinates: [ 33.4500, 70.6667 ] },
					{ name: '9', coordinates: [ 32.6500, 16.9167 ] },
					{ name: '10', coordinates: [ 48.2000, 16.3667 ] },
					{ name: '11', coordinates: [ 48.2012, 34.3667 ] }
			]},
			options: { mutationRate: 0.015, elitism: true, populationSize: 30,	// TODO must test the remaining optinal bounds tournamentSize: 5 },
		}

		// Test good request
		api.post(computeEntryPoint)
		.set('Accept', 'application/json')
		.set('Accept-Version', pjson.version)
		.send(goodRequest)
		.expect(200)
		.end( function(err, res) {
			if (err) return done(err);
			expect(res.body).to.be.jsonSchema(expectedCompute);
			//Get the result
			api.get(resultEntryPoint+'/'+res.body.code)
			.set('Accept', 'application/json')
			.set('Accept-Version', pjson.version)
			.expect(200)
			.end((err, res) => {
				if (err) return done(err);
				expect(res.body).to.be.jsonSchema(expected_result_json_schema);
			});
		});

		done();
  });
});
