// node-rest-tsp 0.0.1
// Exposing rust-tsp via nodejs rest API
// Date: 5th June 2015
// Repo: https://github.com/JoaoHenriquePereira/node-rest-tsp

//
// Root Controller Test
//

var chai 			= require('chai');
	expect 			= require('chai').expect,
	hal 			= require('hal');
	pjson 			= require('../package.json');
	supertest 		= require('supertest'),
	api 			= supertest('http://localhost:8080');

chai.use(require('chai-json-schema'));

describe('root', function() {

	it('Server root should return \"Hello!\"', function(done) {

		var entry_point = '/';
		var expected_json = {
			"message": "Hello!"
		}

		api.get(entry_point)
		.set('Accept', 'application/json')
		.expect(200)
		.end( function(err, res) {
			if (err) return done(err);
			expect(res.body).to.be.jsonSchema(expected_json);
			done();
		});
  	});

  	it('API root should provide HATEOAS navigation', function(done) {
  		
  		var entry_point = '/'+pjson.name;
  		var invalid_version = '9'+pjson.version;

  		var expected_json_schema = require('../schemas/root-api-schema-output.json');

  		var expected_json = new hal.Resource({
			name: pjson.name,
			version: pjson.version,
			repository: pjson.repository,
			cacheable: true
		}, '/'+pjson.name);

		expected_json.link('compute', '/'+pjson.name+'/compute');

		// No version specified
		api.get(entry_point)
		.set('Accept', 'application/json')
		.expect(200)
		.end( function(err, res) {
			if (err) return done(err);
			expect(res.body).to.be.jsonSchema(expected_json_schema);
			//Warning not a fan of comparing by stringify like this
			expect(JSON.stringify(res.body)).to.be.equal(JSON.stringify(expected_json));
		});

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

		// Incorrect version specified
		api.get(entry_point)
		.set('Accept', 'application/json')
		.set('Accept-Version', invalid_version)
		.expect(400) // Should be 501?
		.end( function(err, res) {
			if (err) return done(err);
			done();
		});
  	});

});