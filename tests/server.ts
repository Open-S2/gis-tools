/**
 * @returns - a Bun server
 */
export function buildServer() {
  return Bun.serve({
    port: 0, // Use port 0 to let Bun choose an available port
    /**
     * @param req - the request from the user
     * @returns - a response of the file to the user
     */
    async fetch(req) {
      // Extract the pathname from the request URL
      const { pathname } = new URL(req.url);
      // Build the absolute file path based on the request
      const file = Bun.file(`${__dirname}${pathname}`);
      // If the file does not exist or is empty, return 404
      if (!file || file.size === 0) return new Response(null, { status: 404 });
      // Return the file as the response
      return new Response(file);
    },
  });
}
