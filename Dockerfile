FROM ubuntu:20.04
SHELL ["/bin/bash", "-c"]

# metadata
ARG VCS_REF
ARG BUILD_DATE
ARG DEBIAN_FRONTEND=noninteractive

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

COPY ./node-subtensor /usr/local/bin

RUN /usr/local/bin/node-subtensor --version

EXPOSE 30333 9933 9944
