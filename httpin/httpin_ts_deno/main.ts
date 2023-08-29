const options = {
  onListen: async ({ hostname, port }) => {
    const host = `${hostname}:${port}`;
    const url = `http://${host}`;
    console.log(`Listen on ${url}`);
    console.log(`Request ${url}`, "=>", await $.request(url).text());
  },
};

const handler = async (req: Request) => {
  console.log(req)

  console.log(`content-type: ${req.headers.get("content-type")}`);
  console.log("")
  try {
    if (req.headers.get("content-type") == "application/json") {
      const jsonObj = await req.json();
      const jsonStr = JSON.stringify(jsonObj, null, 2);
      console.log("request body json:");
      console.log(jsonStr);

      const content = jsonStr + "\n\n";
      Deno.writeTextFileSync('test.txt', content, { append: true });
    }

  } catch (e) {
    console.log(e.getMessage)
  }

  return new Response("hello deno");
}

Deno.serve(handler, options);
