FROM lukechannings/deno:v1.30.3 as deno-arm64

FROM denoland/deno:1.30.3 as deno-amd64

FROM deno-${TARGETARCH}

EXPOSE 8000

WORKDIR /app

# These steps will be re-run upon each file change in your working directory:
ADD . .

# Compile the main app so that it doesn't need to be compiled each startup/entry.
RUN deno cache app.ts

CMD ["task", "start"]
