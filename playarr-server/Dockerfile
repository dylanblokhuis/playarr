FROM denoland/deno:1.30.3

EXPOSE 8000

RUN apt-get update && apt-get install -y curl sqlite3 libsqlite3-dev
ENV DENO_SQLITE_PATH=/usr/lib/x86_64-linux-gnu/libsqlite3.so.0

WORKDIR /app

# These steps will be re-run upon each file change in your working directory:
ADD . .

# Compile the main app so that it doesn't need to be compiled each startup/entry.
RUN deno cache app.ts

CMD ["task", "start"]