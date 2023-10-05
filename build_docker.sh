#!/bin/zsh

# docker build --tag <name:tag> - < Dockerfile
docker build --file Dockerfile --tag rust_tenma:0.2 .
