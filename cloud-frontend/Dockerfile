FROM docker.io/denoland/deno:alpine

WORKDIR /app

# 2. Copy all source
COPY . .

RUN deno install

# 3. Build SvelteKit app for Deno
# This internally caches dependencies as needed
RUN deno task build

EXPOSE 8000

CMD ["deno", "run", "-A", ".deno-deploy/server.ts"]
