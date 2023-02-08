FROM lukechannings/deno:v1.30.3 as deno-arm64
ENV DENO_SQLITE_PATH=/usr/lib/aarch64-linux-gnu/libsqlite3.so.0

FROM denoland/deno:1.30.3 as deno-amd64
ENV DENO_SQLITE_PATH=/usr/lib/x86_64-linux-gnu/libsqlite3.so.0

FROM deno-${TARGETARCH}
EXPOSE 8000

ENV TZ=Europe/Amsterdam
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

RUN apt update && apt install -y wget curl build-essential tclsh

# dl sqlite3
RUN \
  wget \
  -O sqlite.tar.gz \
  https://www.sqlite.org/src/tarball/sqlite.tar.gz?r=release && \
  tar xvfz sqlite.tar.gz

# Configure and make SQLite3 binary
RUN \
  ./sqlite/configure --prefix=/usr && \
  make && \
  make install \
  && \
  # Smoke test
  sqlite3 --version

# Purge build dependencies
RUN apt purge -y wget curl build-essential tclsh

WORKDIR /app

# These steps will be re-run upon each file change in your working directory:
ADD . .

# Compile the main app so that it doesn't need to be compiled each startup/entry.
RUN deno cache app.ts

CMD ["task", "start"]