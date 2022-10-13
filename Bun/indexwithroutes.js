import Siopao from 'siopao'
const app = new Siopao()

app.get('/test', () => new Response(JSON.stringify({result: "Hello world"})))

Bun.serve({
  port: 8080,
  fetch: (request) => {
    // Custom logic here

    return app.fetch(request)
  }
})

