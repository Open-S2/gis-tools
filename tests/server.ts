/**
 * @returns - a Bun server
 */
export function buildServer() {
  return Bun.serve({
    port: 8008, // Use port 0 to let Bun choose an available port
    /**
     * @param req - the request from the user
     * @returns - a response of the file to the user
     */
    async fetch(req) {
      // take the req as a JSON object
      const { path } = (await req.json()) as { path: string };
      // path describes the file to serve
      const file = Bun.file(`${__dirname}/${path}`);
      if (file.length === 0) return new Response(null, { status: 404 });
      return new Response(file);
    },
  });
}
