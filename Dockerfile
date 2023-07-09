FROM rust:1.67

WORKDIR /usr/src/kb2
COPY . .

RUN cargo build --release

CMD ["target/release/kb2server"]