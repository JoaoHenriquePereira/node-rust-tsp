'use strict';

// node-rest-tsp 0.0.1
// Exposing rust-tsp via nodejs rest API
// Repo: https://github.com/JoaoHenriquePereira/node-rest-tsp

//
// Root Controller Test
//

const chai = require('chai');
const	expect = require('chai').expect;
const	hal = require('hal');
const	pjson = require('../package.json');
const	supertest = require('supertest');
const	api = supertest('http://localhost:8080');

chai.use(require('chai-json-schema'));

describe('root', function() {

  	it('API root should provide HATEOAS navigation', function(done) {

  		let entry_point = '/'+pjson.name;

  		let expected_json_schema = require('../schemas/root-api-schema-output.json');

  		let expected_json = new hal.Resource({
			name: pjson.name,
			version: pjson.version,
			repository: pjson.repository,
			cacheable: true
		}, '/'+pjson.name);

		expected_json.link('compute', '/'+pjson.name+'/compute');

		// Correct version specified
		api.get(entry_point)
		.set('Accept', 'application/json')
		.set('Accept-Version', pjson.version)
		.expect(200)
		.end( function(err, res) {
			if (err) return done(err);
			expect(res.body).to.be.jsonSchema(expected_json_schema);
			expect(JSON.stringify(res.body)).to.be.equal(JSON.stringify(expected_json));
		});

		done();
  	});

});
