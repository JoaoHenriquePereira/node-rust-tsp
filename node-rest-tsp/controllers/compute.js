'use strict';

// node-rest-tsp 0.0.1
// Exposing rust-tsp via nodejs rest API
// Repo: https://github.com/JoaoHenriquePereira/node-rest-tsp

//
// Compute Controller
//

const	Enum = require('enum');
const	ResponseBuilder = require('../response');
const	acceptable_graph_types = new Enum(['u2d-cartesian']);	//For now all we'll have is this one
const chai = require('chai');
const	hal = require('hal');
const	pjson = require('../package.json');
const	router = require('koa-router')();

chai.use(require('chai-json-schema'));

let Response = null;

function filter_post_input(req_body) {

	let expected_input_schema = require('../schemas/compute-api-schema-input.json');
	let validation_result = chai.tv4.validateMultiple(req_body, expected_input_schema);

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

	let compute = {

		// Compute POST handler
  		post: function *(){
  			if(filter_post_input(this.request.body)){
  				console.log(this.request.body);

					// Process request and generate result
					var compute_result = model.compute(this.request.body);
					// Prepare response
					Response = new ResponseBuilder.ComputeResponse('/'+pjson.name+'/compute')
													.build(compute_result)
													.finish();


					this.status = 200;
					this.body = Response;
				} else {
					this.status = 400;
					this.body = Response;
				}
  		},

  		// Result GET handler
  		result_get: function *(id){

			if(id != undefined){
				Response = new ResponseBuilder.ResultResponse('/'+pjson.name+'/result/'+id)
													.build(result)
													.finish();

				this.status = 200;
				this.body = Response;
			} else {
				Response = new ResponseBuilder.ErrorResponse('/'+pjson.name+'/result/'+id)
													.build([{
    														message: 'This id is invalid or it has expired',
    														dataPath: 'problem: ' + id
    												}])
    												.addLink('compute', '/'+pjson.name+'/compute')
													.finish();
				this.status = 400;
				this.body = Response;
			}
		},

	}

	// Wiring
	const API_PATH = '/'+pjson.name;

	router
		.post(API_PATH+'/compute', compute.post)
		.get(API_PATH+'/result/:id', compute.result_get);

	server.use(router.routes());

}
