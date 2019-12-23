
################## Stage 1 ####################

FROM rust:latest as builder

COPY ./ ./

RUN cargo build --release

RUN mkdir -p /build-out

RUN cp target/release/br4infuck /build-out/

################## Stage 2 ####################

FROM alpine:latest

COPY --from=builder /build-out/br4infuck /

CMD ["/br4infuck"]
