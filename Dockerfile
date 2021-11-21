# **NOTE**: This docker file expects to be run in a directory outside of subtensor.
# It also expects two build arguments, the bittensor snapshot directory, and the bittensor
# snapshot file name. 

# This runs typically via the following command: 
# $ docker build -t subtensor . --platform linux/x86_64 --build-arg SNAPSHOT_DIR="DIR_NAME" --build-arg SNAPSHOT_FILE="FILENAME.TAR.GZ"  -f subtensor/Dockerfile


FROM ubuntu:20.04
SHELL ["/bin/bash", "-c"]

# metadata
ARG VCS_REF
ARG BUILD_DATE
ARG SNAPSHOT_DIR
ARG SNAPSHOT_FILE

LABEL ai.opentensor.image.authors="operations@opentensor.ai" \
        ai.opentensor.image.vendor="Opentensor Foundation" \
        ai.opentensor.image.title="opentensor/subtensor" \
        ai.opentensor.image.description="Opentensor Subtensor Blockchain" \
        ai.opentensor.image.revision="${VCS_REF}" \
        ai.opentensor.image.created="${BUILD_DATE}" \
        ai.opentensor.image.documentation="https://opentensor.gitbook.io/bittensor/"

# show backtraces
ENV RUST_BACKTRACE 1

# install tools and dependencies
RUN apt-get update && \
        DEBIAN_FRONTEND=noninteractive apt-get upgrade -y && \
        DEBIAN_FRONTEND=noninteractive apt-get install -y \
                libssl1.1 \
                ca-certificates \
                git \
                curl && \
# apt cleanup
        apt-get autoremove -y && \
        apt-get clean && \
        find /var/lib/apt/lists/ -type f -not -name lock -delete;


# Clone subtensor latest
RUN git clone https://github.com/opentensor/subtensor_exodus.git subtensor

# Install cargo and Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN subtensor/scripts/init.sh

COPY subtensor/target/release/node-subtensor /usr/local/bin

RUN /usr/local/bin/node-subtensor --version

COPY ${SNAPSHOT_DIR}/${SNAPSHOT_FILE} /subtensor

RUN mkdir -p /root/.local/share/node-subtensor/chains/nakamoto_mainnet/db
RUN tar -zxvf /subtensor/${SNAPSHOT_FILE} -C  /root/.local/share/node-subtensor/chains/nakamoto_mainnet/db

EXPOSE 30333 9933 9944