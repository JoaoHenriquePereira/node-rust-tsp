node-rest-tsp
==========

Playing with nodejs, TDD and REST API's to expose <a href="https://github.com/JoaoHenriquePereira/rust-tsp">rust-tsp</a>

# Libs

* <a href="http://chaijs.com/">chai</a>
* <a href="http://chaijs.com/plugins/chai-json-schema">chai json validator</a>
* <a href="https://www.npmjs.com/package/enum">enum</a>
* <a href="https://www.npmjs.com/package/hal">hal</a>
* <a href="https://www.npmjs.com/package/istanbul">istanbul</a>
* <a href="http://mochajs.org/">mocha</a>
* <a href="https://www.npmjs.com/package/node-cache">node-cache</a>
* <a href="http://mcavage.me/node-restify/">restify</a>
* <a href="https://github.com/visionmedia/supertest">supertest</a>
* <a href="http://underscorejs.org/">underscore</a>

# Comments

Status: Work in progress

I started by the very basics by having a sort of a "Hello World" in the server root "/". Then I coded the api entry point at /node-rest-tsp which on itself describes the app and provides "hypermedia"'ish directioning with JSON. (hypermedia'ish as JSON is not an official format).

I am focusing mainly on architecturing it properly from the start and using a test driven development to achieve routing requests.

# References

* <a href="http://blog.stateless.co/post/13296666138/json-linking-with-hal">JSON linking with HAL</a>
* <a href="https://scotch.io/">scotch.io</a>
* <a href="https://github.com/jedwood/express-for-APIs/blob/master/server.js">express API</a>
* <a href="http://www.infoq.com/articles/webber-rest-workflow">How to GET a Cup of Coffee</a>
* <a href="http://martinfowler.com/articles/richardsonMaturityModel.html">Richardson maturity model explained</a>

# TODO

* Actually call rust-tsp, atm is just mocked
* Add caching
