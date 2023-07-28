# FROM ekidd/rust-musl-builder:nightly as builder
# FROM ekidd/rust-musl-builder:nightly-2021-12-23 as builder
FROM nasqueron/rust-musl-builder as builder
ADD . ./
RUN sudo chown -R rust:rust /home/rust
RUN cargo build --release

FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/fixme /usr/local/bin/fixme
EXPOSE 8080
CMD ["/usr/local/bin/fixme"]
