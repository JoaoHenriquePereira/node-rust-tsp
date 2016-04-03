'use strict';

// node-rest-tsp 0.0.1
// Exposing rust-tsp via nodejs rest API
// Repo: https://github.com/JoaoHenriquePereira/node-rest-tsp

//
// Compute Controller
//

const	Enum = require('enum');
const	ResponseBuilder = require('../response');
const	acceptableGraphTypes = new Enum(['u2d-cartesian']);	//For now all we'll have is this one
const chai = require('chai');
const	hal = require('hal');
const	pjson = require('../package.json');
const	router = require('koa-router')();

chai.use(require('chai-json-schema'));

let Response = null;

function filter_post_input(req_body) {
	const expectedInput = require('../schemas/compute-api-schema-input.json');
	const validationResult = chai.tv4.validateMultiple(req_body, expectedInput);

	// Check schema for incoherences
	if(!validationResult.valid) {
		Response = new ResponseBuilder.ErrorResponse(`/${pjson.name}/compute`)
			.build(validationResult.errors)
			.finish();

		return false;
	}

	// Check input graph type
	if(!acceptableGraphTypes.get(req_body.graph_type)){
		Response = new ResponseBuilder.ErrorResponse(`/${pjson.name}/compute`)
			.build([{ dataPath: '/graph_type', message: `Invalid graph_type, acceptable types: ${acceptableGraphTypes.toString()}` }])
			.finish();

		return false;
	}

	return true;
}

module.exports.setup = function (server, model) {
	const compute = {

  	post: function *(){
			if(filter_post_input(this.request.body)){
				console.log(this.request.body);

				// Process request and generate result
				const result = model.compute(this.request.body);

				// Prepare response
				Response = new ResponseBuilder.ComputeResponse(`/${pjson.name}/compute`)
					.build(result)
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
				Response = new ResponseBuilder.ResultResponse(`/${pjson.name}/result/${id}`)
					.build(result)
					.finish();

				this.status = 200;
				this.body = Response;
			} else {
				Response = new ResponseBuilder.ErrorResponse('/'+pjson.name+'/result/'+id)
					.build([{ dataPath: `problem: ${id}`, message: 'This id is invalid or it has expired' }])
					.addLink('compute', `/${pjson.name}/compute`)
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
