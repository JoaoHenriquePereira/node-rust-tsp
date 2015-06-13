// node-rest-tsp 0.0.1
// Exposing rust-tsp via nodejs rest API
// Repo: https://github.com/JoaoHenriquePereira/node-rest-tsp

//
// Compute Controller
//

var chai 					= require('chai');
	Enum					= require('enum');
	hal 					= require('hal');
	pjson 					= require('../package.json');
	ResponseBuilder			= require('../response');
	acceptable_graph_types 	= new Enum(['u2d-cartesian']);				//For now all we'll have is this one

chai.use(require('chai-json-schema'));
var Response = null;

function filter_post_input(req_body) {

	var expected_input_schema = require('../schemas/compute-api-schema-input.json');

	var validation_result = chai.tv4.validateMultiple(req_body, expected_input_schema);

	// Check schema for incoherences
	if(!validation_result.valid) {
		Response = new ResponseBuilder.ErrorResponse('/'+pjson.name+'/compute')
												.build(validation_result.errors)
												.finish();
		return false;
	}

	// Check input graph type
	if(!acceptable_graph_types.get(req_body.graph_type)){
		Response = new ResponseBuilder.ErrorResponse('/'+pjson.name+'/compute')
												.build([{ 
    													message: 'Invalid graph_type, acceptable types: '+acceptable_graph_types.toString(),
    													dataPath: '/graph_type'
    											}])
												.finish();
		return false;
	}

	return true;
}

module.exports.setup = function (server, model) {
	
	// Compute POST handler 
	function compute_post(req, res, next) {

		if(filter_post_input(req.body)){

			// Process request and generate result
			var compute_result = model.compute(req.body);
			// Prepare response
			Response = new ResponseBuilder.ComputeResponse('/'+pjson.name+'/compute')
												.build(compute_result)
												.finish();

			
			res.send(200, Response);
		} else {
			res.send(400, Response);
		}

		return next();
	}

	// Result GET handler 
	function result_get(req, res, next) {

		var result = model.get(req.params.id);
		if(result != undefined){
			res.send(200, result);
		} else {
			Response = new ResponseBuilder.ErrorResponse('/'+pjson.name+'/result/'+req.params.id)
												.build([{ 
    													message: 'This id is invalid or it has expired',
    													dataPath: 'problem: ' + req.params.id
    											}])
    											.addLink('compute', '/'+pjson.name+'/compute')
												.finish();
			res.send(400, Response);
		}

		return next();
	}

	// Wiring
	var API_PATH = '/'+pjson.name;
	server.post({path: API_PATH+'/compute', version: '0.0.1'}, compute_post);
	server.get({path: API_PATH+'/result/:id', version: '0.0.1'}, result_get);
}