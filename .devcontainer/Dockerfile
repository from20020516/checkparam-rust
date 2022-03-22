# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.209.6/containers/rust/.devcontainer/base.Dockerfile

# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT="buster"
FROM mcr.microsoft.com/vscode/devcontainers/rust:0-${VARIANT}

COPY --from=docker:latest /usr/local/bin/dockerd /usr/local/bin/dockerd
COPY --from=docker:latest /usr/local/bin/docker /usr/local/bin/docker
COPY --from=docker:latest /usr/local/bin/runc /usr/local/bin/runc
COPY --from=docker:latest /usr/local/bin/containerd /usr/local/bin/containerd
COPY --from=docker:latest /usr/local/bin/containerd-shim /usr/local/bin/containerd-shim
COPY --from=docker:latest /usr/local/bin/containerd-shim-runc-v2 /usr/local/bin/containerd-shim-runc-v2
COPY --from=amazon/aws-cli /usr/local/aws-cli/v2/current /usr/local

RUN apt update \
    && export DEBIAN_FRONTEND=noninteractive \
    && apt -y install default-mysql-client
