// node-rest-tsp 0.0.1
// Exposing rust-tsp via nodejs rest API
// Repo: https://github.com/JoaoHenriquePereira/node-rest-tsp

//
// Root Controller
//

var hal 	= require('hal');
	pjson 	= require('../package.json');

module.exports.setup = function (server) {

	//Server root GET
	function root_get(req, res, next) {
		res.json('{"message": "Hello!"}');
	}

	//API root GET
	function api_root_get(req, res, next) {

		var api_root = new hal.Resource({
			name: pjson.name,
			version: pjson.version,
			repository: pjson.repository,
			cacheable: true
		}, '/'+pjson.name);

		api_root.link('compute', '/'+pjson.name+'/compute');

		res.send(api_root);
		return next();
	}

	// Wiring
	var API_PATH = '/'+pjson.name;
   	server.get('/', root_get);
   	server.get({path: API_PATH, version: '0.0.1'}, api_root_get);
}