'use strict';

// node-rest-tsp 0.0.2
// Exposing rust-tsp via nodejs rest API
// Repo: https://github.com/JoaoHenriquePereira/node-rest-tsp

//
// App server definition
//

// Includes
const fs = require('fs');
const	koa = require('koa');
const server = koa();

// Server Config
const port 	= process.env.PORT || 8080;
server.name = 'server@localhost'

// Cache Model params
const stdTTL = 0; 				//Default
const checkperiod = 600;	//Default

// Bluntly add our model
let modelFile = fs.readdirSync('model')[0];
let model;

if (modelFile.indexOf('.js') === -1) {
	return;
} else {
	modelFile = modelFile.replace('.js', '');
	model = require('./model/' + modelFile);
}

// Add the routing controllers
let computeModel = new model(stdTTL, checkperiod);
let controllerFiles = fs.readdirSync('controllers');

controllerFiles.forEach(function (controllerFile) {
	if (controllerFile.indexOf('.js') === -1) {
		return;
	} else {
		controllerFile = controllerFile.replace('.js', '');
		let controller = require('./controllers/' + controllerFile);
		// Model is accessed by all controllers, no biggie since we are going for simplicity
		controller.setup(server, computeModel);
	}
});

// Start listening
server.listen(port, function () {
	console.log('%s listening at %s', server.name, port);
});


