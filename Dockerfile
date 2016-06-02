FROM centos:7
MAINTAINER Denis Kolodin <deniskolodin@gmail.com>

EXPOSE 8080

ENV SOURCES=/sources

RUN yum update -y
RUN yum install -y file gcc openssl-devel
RUN curl -sSf https://static.rust-lang.org/rustup.sh | sh -s -- --disable-sudo

RUN mkdir -p $SOURCES
ADD ./ $SOURCES

WORKDIR $SOURCES
RUN cargo build --release

CMD cargo run --release
