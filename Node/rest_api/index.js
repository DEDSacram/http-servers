const fastify = require("fastify")({
  logger: false,
 });
 fastify.get("/test", async (request, reply) => {
  reply.type("application/json").code(200);
  return {
  result: "Hello World",
  };
 })
 ;fastify.listen(8080, (err, address) => {
  if (err) throw err;
 });