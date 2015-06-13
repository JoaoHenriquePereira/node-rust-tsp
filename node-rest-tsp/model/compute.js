var assert 			= require('assert');
	crypto 			= require('crypto');
	NodeCache		= require('node-cache');

var cache;
var uri;

/**
 * Represents the ComputeModel for managing results and accessing rust-tsp,
 * In fact it's a cache manager and rust-tsp DAO
 * @param {number}  stdTTL     		the std time-to-life in seconds.
 * @param {number}  checkperiod     the .
 */

var ComputeModel = (function () {
  	function ComputeModel(db_config, stdTTL, checkperiod) {
  		cache = new NodeCache( { stdTTL: stdTTL, checkperiod: checkperiod } );
  		uri = db_config.db_uri;
  	}

	ComputeModel.prototype.set = function (key, val, ttl) {
		return cache.set( key, val, ttl);
	}  	

	ComputeModel.prototype.get = function (key) {
		return cache.get( key );
	}

	ComputeModel.prototype.compute = function (json_input_data) {

		// Call our lib
		

		return _id;
	}  	

  	return ComputeModel;
})();

module.exports = ComputeModel;