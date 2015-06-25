// node-rest-tsp 0.0.1
// Exposing rust-tsp via nodejs rest API
// Repo: https://github.com/JoaoHenriquePereira/node-rest-tsp

//
// Root Controller
//

var hal 	= require('hal');
	pjson 	= require('../package.json');
	router 	= require('koa-router')();

module.exports.setup = function (server) {

	//API root GET
	function *api_root_get() {
        var api_root = new hal.Resource({
			name: pjson.name,
			version: pjson.version,
			repository: pjson.repository,
			cacheable: true
		}, '/'+pjson.name);

		api_root.link('compute', '/'+pjson.name+'/compute');

		this.body = api_root;
	}

	// Wiring
	var API_PATH = '/'+pjson.name;
	router.get(API_PATH, api_root_get);
   	server.use(router.routes());  
}