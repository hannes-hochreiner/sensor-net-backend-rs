FROM fedora:36 AS builder
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN mkdir -p /opt/sensor-net-backend
COPY src /opt/sensor-net-backend/src
COPY Cargo.* /opt/sensor-net-backend/
RUN dnf install gcc fontconfig-devel freetype-devel pkgconf-pkg-config -y
RUN source $HOME/.cargo/env && cd /opt/sensor-net-backend && cargo build --release --locked

FROM fedora:36
MAINTAINER Hannes Hochreiner <hannes@hochreiner.net>
RUN dnf install fontconfig freetype -y
COPY --from=builder /opt/sensor-net-backend/target/release/sensor-net-backend /opt/sensor-net-backend
CMD ["/opt/sensor-net-backend"]