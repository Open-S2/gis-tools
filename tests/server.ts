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
    fetch(req) {
      const { pathname } = new URL(req.url);
      const filePath = `${__dirname}${pathname}`;
      const file = Bun.file(filePath);

      if (file.size === 0) return new Response(null, { status: 404 });

      // Handle range request
      const rangeHeader = req.headers.get('Range');
      if (rangeHeader !== null) {
        const [unit, range] = rangeHeader.split('=');
        if (unit === 'bytes') {
          const [start, endParsed] = range.split('-').map(Number);
          const end = !isNaN(endParsed) && endParsed !== 0 ? endParsed : undefined;

          const fileSize = file.size;
          const endByte = end !== undefined ? Math.min(end, fileSize - 1) : fileSize - 1;
          const rangeStart = Math.max(start, 0);

          // Read the specified byte range from the file
          const chunk = file.slice(rangeStart, endByte + 1);

          return new Response(chunk, {
            status: 206,
            headers: {
              'Content-Range': `bytes ${rangeStart}-${endByte}/${fileSize}`,
              'Content-Length': String(endByte - rangeStart + 1),
              'Accept-Ranges': 'bytes',
            },
          });
        }
      }

      // If no range is requested, serve the whole file
      return new Response(file);
    },
  });
}
