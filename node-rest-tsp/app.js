// node-rest-tsp 0.0.1
// Exposing rust-tsp via nodejs rest API
// Repo: https://github.com/JoaoHenriquePereira/node-rest-tsp

//
// App server definition
//

// Includes
var fs 		= require('fs');
	restify = require('restify');
	
// Server Config
var port 	= process.env.PORT || 8080;

// Cache Model params
var stdTTL = 0; 		//Default
var checkperiod = 600;	//Default

// Serve
var server = restify.createServer({'name': 'node-rest-tsp'});

server.use(restify.fullResponse()); // Set up default headers
server.use(restify.bodyParser()); 	// Remap the body content of a request


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
	console.log('%s listening at %s', server.name, server.url);
});


