FROM mcr.microsoft.com/vscode/devcontainers/rust:buster

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends gcc
