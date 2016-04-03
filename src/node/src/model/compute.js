'use strict';

// node-rest-tsp 0.0.1
// Exposing rust-tsp via nodejs rest API
// Repo: https://github.com/JoaoHenriquePereira/node-rest-tsp

//
// Compute Model
//

const	NodeCache	= require('node-cache');
const assert = require('assert');
const	crypto = require('crypto');
const	ffi = require('ffi');

/**
 * Represents the ComputeModel for managing results and accessing rust-tsp,
 * In fact it's a cache manager and rust-tsp DAO
 * @param {number}  stdTTL     		the std time-to-life in seconds.
 * @param {number}  checkperiod     the .
 */

let cache;
const library_name = '../target/debug/librusttsp';
let rust_tsp_lib = ffi.Library(library_name, {
	'compute_adapter': ['string', ['string']]
});

function ComputeModel(stdTTL, checkperiod) {
  cache = new NodeCache( { stdTTL: stdTTL, checkperiod: checkperiod } );
}

ComputeModel.prototype.set = function (key, val, ttl) {
	return cache.set( key, val, ttl);
}

ComputeModel.prototype.get = function (key) {
	return cache.get( key );
}

ComputeModel.prototype.compute = function (input) {
	// Generate Key
	const currentDate = (new Date()).valueOf().toString();
	const random = Math.random().toString();
	const _id = crypto.createHash('sha1').update(`${currentDate}${random}`).digest('hex');

	//Call lib
	const result = rust_tsp_lib.compute_adapter(JSON.stringify(input));

	this.set(_id, result);

	return _id;
}

module.exports = ComputeModel;
