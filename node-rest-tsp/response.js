// node-rest-tsp 0.0.1
// Exposing rust-tsp via nodejs rest API
// Repo: https://github.com/JoaoHenriquePereira/node-rest-tsp

//
// Builds responses to be used by the api
//

var _	    = require('underscore');
    Enum  = require('enum');
    hal	  = require('hal');
    pjson = require('./package.json');
    util  = require('util');

//
// 'Abstract' Response
//

var Response = (function () {

  	function Response() {
   	    this.response = null;
  	}

  	/**
   	 * Add a link to the response
   	 * @param  {String} rel link relationship.
   	 * @param  {String} href the link.
   	 */
  	Response.prototype.addLink = function (rel, href) {
  		  this.response.link(rel, href);
  	};

    return Response;
})();

//
// Error Response
//

var ErrorResponse = (function () {

    util.inherits(ErrorResponse, Response);

    ErrorResponse.prototype.constructor = ErrorResponse;

    function ErrorResponse(uri) {
        this.response = new hal.Resource({
          code: null,
          message: null,
          description: null,
          _errors: []
        }, uri);
        return this;
    }

    ErrorResponse.prototype.build = function (validation_errors) {

        _.map(validation_errors, function(error){

            _error = {
                "message": error.message,
                "description": 'problem: '+error.dataPath
            };

            this.response._errors.push(_error);

        }, this);

        this.response.code        = '400';
        this.response.message     = 'Validation error(s)';
        this.response.description = 'Please fix all errors before proceeding';

        return this;
    }

    ErrorResponse.prototype.finish = function () {
        return this.response;
    }

    return ErrorResponse;
})();

module.exports.ErrorResponse = ErrorResponse;

//
// Compute Response
//

var ComputeResponse = (function () {

    util.inherits(ComputeResponse, Response);

    ComputeResponse.prototype.constructor = ComputeResponse;

    function ComputeResponse(uri) {
        this.response = new hal.Resource({
          code: null,
          message: null,
          description: null,
          _links: []
        }, uri);
        return this;
    }

    ComputeResponse.prototype.build = function (id_generated) {

        this.response.code        = id_generated;
        this.response.message     = 'Success';
        this.response.description = 'You can access your result via the _links provided using the key provided';
        this.response.link('result', '/'+pjson.name+'/result/'+id_generated);

        return this;
    }

    ComputeResponse.prototype.finish = function () {
        return this.response;
    }

    return ComputeResponse;
})();

module.exports.ComputeResponse = ComputeResponse;

//
// Result Response
//

var ResultResponse = (function () {

    util.inherits(ResultResponse, Response);

    ResultResponse.prototype.constructor = ResultResponse;

    function ResultResponse(uri) {
        this.response = new hal.Resource({
          tour: null,
          distance: null,
          fitness: null,
          _links: []
        }, uri);
        return this;
    }

    ResultResponse.prototype.build = function (cached_result) {

        var parsed_cached_result = JSON.parse(cached_result);

        this.response.tour          = parsed_cached_result.tour;
        this.response.distance      = 1 / parsed_cached_result.fitness;
        this.response.fitness       = parsed_cached_result.fitness;
        this.response.link('compute', '/'+pjson.name+'/compute');

        return this;
    }

    ResultResponse.prototype.finish = function () {
        return this.response;
    }

    return ResultResponse;
})();

module.exports.ResultResponse = ResultResponse;


/*

// node-rest-tsp 0.0.1
// Exposing rust-tsp via nodejs rest API
// Repo: https://github.com/JoaoHenriquePereira/node-rest-tsp

//
// Builds responses to be used by the api
//

var _	    = require('underscore');
    Enum  = require('enum');
    hal	  = require('hal');
    pjson = require('./package.json');
    util  = require('util');

//
// 'Abstract' Response
//

class Response {

  	constructor() {
   	    this.response = null;
  	}


  	addLink (rel, href) {
  		  this.response.link(rel, href);
  	}
}

//
// Error Response
//

class ErrorResponse extends Response {

    constructor(uri) {
        this.response = new hal.Resource({
          code: null,
          message: null,
          description: null,
          _errors: []
        }, uri);
        return this;
    }

    build(validation_errors) {

        _.map(validation_errors, function(error){

            _error = {
                "message": error.message,
                "description": 'problem: '+error.dataPath
            };

            this.response._errors.push(_error);

        }, this);

        this.response.code        = '400';
        this.response.message     = 'Validation error(s)';
        this.response.description = 'Please fix all errors before proceeding';

        return this;
    }

    finish() {
        return this.response;
    }

}

module.exports.ErrorResponse = ErrorResponse;

//
// Compute Response
//

class ComputeResponse extends Response {

    constructor(uri) {
        this.response = new hal.Resource({
          code: null,
          message: null,
          description: null,
          _links: []
        }, uri);
        return this;
    }

    build(id_generated) {

        this.response.code        = id_generated;
        this.response.message     = 'Success';
        this.response.description = 'You can access your result via the _links provided using the key provided';
        this.response.link('result', '/'+pjson.name+'/result/'+id_generated);

        return this;
    }

    finish() {
        return this.response;
    }
}

module.exports.ComputeResponse = ComputeResponse;

//
// Result Response
//

class ResultResponse extends Response {

    constructor(uri) {
        this.response = new hal.Resource({
          tour: null,
          distance: null,
          fitness: null,
          _links: []
        }, uri);
        return this;
    }

    build(cached_result) {

        var parsed_cached_result = JSON.parse(cached_result);

        this.response.tour          = parsed_cached_result.tour;
        this.response.distance      = 1 / parsed_cached_result.fitness;
        this.response.fitness       = parsed_cached_result.fitness;
        this.response.link('compute', '/'+pjson.name+'/compute');

        return this;
    }

    finish() {
        return this.response;
    }
}

module.exports.ResultResponse = ResultResponse;


*/
