'use strict';

// node-rest-tsp 0.0.1
// Exposing rust-tsp via nodejs rest API
// Repo: https://github.com/JoaoHenriquePereira/node-rest-tsp

//
// Root Controller
//

const hal = require('hal');
const	pjson = require('../package.json');
const	router = require('koa-router')();

module.exports.setup = function (server) {

	//API root GET
	function *api_root_get() {
      let api_root = new hal.Resource({
				name: pjson.name,
				version: pjson.version,
				repository: pjson.repository,
				cacheable: true
		}, '/'+pjson.name);

		api_root.link('compute', '/'+pjson.name+'/compute');

		this.body = api_root;
	}

	// Wiring
	const API_PATH = '/'+pjson.name;
	router.get(API_PATH, api_root_get);
  server.use(router.routes());
}
