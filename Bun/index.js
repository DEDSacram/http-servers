export default {
    port: 8080,
    fetch(request) {
      return new Response(JSON.stringify(`{result: "Hello world"}`));
    },
  };