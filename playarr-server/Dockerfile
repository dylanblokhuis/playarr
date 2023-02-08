FROM lukechannings/deno:v1.30.3 as deno-arm64
ENV DENO_SQLITE_PATH=/usr/lib/aarch64-linux-gnu/libsqlite3.so.0

FROM denoland/deno:1.30.3 as deno-amd64
ENV DENO_SQLITE_PATH=/usr/lib/x86_64-linux-gnu/libsqlite3.so.0

FROM deno-${TARGETARCH}
EXPOSE 8000

RUN apt-get update && \
    DEBIAN_FRONTEND=unstable apt-get -yq --no-install-recommends install sqlite3 curl && \
    rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/* 

WORKDIR /app

# These steps will be re-run upon each file change in your working directory:
ADD . .

# Compile the main app so that it doesn't need to be compiled each startup/entry.
RUN deno cache app.ts

CMD ["task", "start"]