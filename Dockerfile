# FROM ekidd/rust-musl-builder:nightly as builder
# FROM ekidd/rust-musl-builder:nightly-2021-12-23 as builder
FROM nasqueron/rust-musl-builder as builder
ADD . ./
RUN sudo chown -R rust:rust /home/rust
RUN cargo build --release

# FROM alpine:latest

# Python 3.5+
FROM python:slim-bullseye

WORKDIR /home/rust/src
COPY ./requirements.txt /home/rust/src/

# RUN pip install manifest-tool
# Clone v1.5.2
# RUN git clone https://github.com/PelionIoT/manifest-tool/commit/0ec41e24d92c2b14ce1c9631fa34c02fd9d4a09b

# python:slim-bullseye -- Use `--no-cache-dir` since Docker has its own cache.
RUN pip install --no-cache-dir -r /home/rust/src/requirements.txt
# alpine:latest -- Use `--no-cache` since Docker has its own cache.
# RUN apk --no-cache add ca-certificates

# Copy the executable and its dependencies from the builder image into the Python-based image.
RUN mkdir -p /home/rust/src/templates
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/fixme /usr/local/bin/fixme
COPY --from=builder /home/rust/src/templates/ /home/rust/src/templates/

# Still need to map the host port to container port via `-p 8080:8080`
EXPOSE 8080

CMD ["/usr/local/bin/fixme", "run"]
