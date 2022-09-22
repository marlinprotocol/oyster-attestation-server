# // Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
# // SPDX-License-Identifier: MIT-0

FROM public.ecr.aws/amazonlinux/amazonlinux:2 AS builder


WORKDIR /enclave-server/
COPY ./enclave-server .

WORKDIR /tokio-proxy/
COPY ./tokio-proxy .

RUN yum install gcc -y	

# Get Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

# Build the Rust applications
WORKDIR /tokio-proxy/
RUN cargo build --release 

WORKDIR /enclave-server/
RUN cargo build --release 

FROM public.ecr.aws/amazonlinux/amazonlinux:2
RUN yum install iproute -y

WORKDIR /app

COPY --from=builder /enclave-server/target/release/enclave-server .
COPY --from=builder /tokio-proxy/target/release/vsock-to-ip .
COPY run.sh .

RUN chmod +x /app/run.sh
CMD ["/app/run.sh"]
