FROM mcr.microsoft.com/vscode/devcontainers/rust:0-1-bullseye
#FROM ubuntu:latest
#LABEL authors="y_mc"

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install clang lld \
    && apt-get autoremove -Y && apt-get clean -y

#ENTRYPOINT ["top", "-b"]