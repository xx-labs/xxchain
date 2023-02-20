FROM phusion/baseimage:jammy-1.0.0
LABEL maintainer "@xxfoundation"
LABEL description="xx network public node"
RUN apt-get update && apt-get install wget -y
WORKDIR /usr/local/bin
RUN wget https://github.com/xx-labs/xxchain/releases/latest/download/xxnetwork-chain \
    && chmod +x xxnetwork-chain
EXPOSE 15974 63007 9933
VOLUME ["/data"]
ENTRYPOINT ["xxnetwork-chain"]