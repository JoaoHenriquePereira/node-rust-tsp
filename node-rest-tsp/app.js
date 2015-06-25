// node-rest-tsp 0.0.2
// Exposing rust-tsp via nodejs rest API
// Repo: https://github.com/JoaoHenriquePereira/node-rest-tsp

//
// App server definition
//

// Includes
var fs 			= require('fs');
	koa 		= require('koa');
    server 		= koa();
	
// Server Config
var port 	= process.env.PORT || 8080;
server.name = 'myserver@localhost'

// Cache Model params
var stdTTL = 0; 		//Default
var checkperiod = 600;	//Default

// Bluntly add our model
var modelFile = fs.readdirSync('model')[0];
var model;

if (modelFile.indexOf('.js') === -1) {                      
	return;                                                           
} else {                                                      
	modelFile = modelFile.replace('.js', '');             
	model = require('./model/' + modelFile); 
}

// Add the routing controllers
var computeModel = new model(stdTTL, checkperiod);
var controllerFiles = fs.readdirSync('controllers');

controllerFiles.forEach(function (controllerFile) { 
	if (controllerFile.indexOf('.js') === -1) {                      
		return;                                                           
	} else {                                                      
		controllerFile = controllerFile.replace('.js', '');             
		var controller = require('./controllers/' + controllerFile); 
		// Model is accessed by all controllers, no biggie since we are going for simplicity
		controller.setup(server, computeModel);                                          
	}
});

// Start listening
server.listen(port, function () {
	console.log('%s listening at %s', server.name, port);
});


