FROM golang:1.24 AS builder

WORKDIR /app

RUN apt-get update && \
    apt-get install -y make git perl bash && \
    rm -rf /var/lib/apt/lists/*

RUN git clone https://github.com/minio/minio.git .

RUN git fetch --tags
RUN git checkout RELEASE.2025-09-07T16-13-09Z

RUN make

FROM debian:stable-slim

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/minio /usr/local/bin/minio

ENTRYPOINT ["minio"]
CMD ["server", "/data", "--console-address", ":9001"]
