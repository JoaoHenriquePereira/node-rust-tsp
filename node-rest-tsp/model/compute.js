var assert 			= require('assert');
	crypto 			= require('crypto');
	MongoClient 	= require('mongodb').MongoClient;
	NodeCache		= require('node-cache');

var cache;
var uri;

/**
 * Inserts a document into our mongodb
 * @param {object}  	db object
 * @param {json}  		json_input_data
 * @param {function} 	callback
 */
var insertDocuments = function(db, json_input_data, callback) {
  // Get the documents collection
  var collection = db.collection('input');
  // Insert some documents
  collection.insert(json_input_data, function(err, result) {
    assert.equal(err, null);
    assert.equal(1, result.result.n);
    assert.equal(1, result.ops.length);
    callback(result);
  });
}

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
		
		// Generate key
		var current_date = (new Date()).valueOf().toString();
		var random = Math.random().toString();
		var _id = crypto.createHash('sha1').update(current_date + random).digest('hex');

		// Set extra fields
		json_input_data._id = _id;
		json_input_data.insert_utimestamp = current_date;

		// Store request in Mongo for rust-tsp usage
		MongoClient.connect(uri, function(err, db) {
			assert.equal(null, err);
			insertDocuments(db, json_input_data, function() {
    			db.close();
  			});
		});

		//Send _id to rust-tsp
		

		return _id;
	}  	

  	return ComputeModel;
})();

module.exports = ComputeModel;