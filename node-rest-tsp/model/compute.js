var assert 			= require('assert');
	crypto 			= require('crypto');
	ffi 			= require('ffi');
	NodeCache		= require('node-cache');
	
var cache;

/**
 * Represents the ComputeModel for managing results and accessing rust-tsp,
 * In fact it's a cache manager and rust-tsp DAO
 * @param {number}  stdTTL     		the std time-to-life in seconds.
 * @param {number}  checkperiod     the .
 */


var library_name = '../target/debug/librust_tsp';
var rust_tsp_lib = ffi.Library(library_name, {
		'compute_adapter': ['string', ['string']]
});

var ComputeModel = (function () {
  	function ComputeModel(stdTTL, checkperiod) {
  		cache = new NodeCache( { stdTTL: stdTTL, checkperiod: checkperiod } );
  	}

	ComputeModel.prototype.set = function (key, val, ttl) {
		return cache.set( key, val, ttl);
	}  	

	ComputeModel.prototype.get = function (key) {
		return cache.get( key );
	}

	ComputeModel.prototype.compute = function (json_input_data) {

		// Call our lib
		
		console.log(rust_tsp_lib.compute_adapter(JSON.stringify(json_input_data)));

		return _id;
	}  	

  	return ComputeModel;
})();

module.exports = ComputeModel;