#! /usr/bin/env bash


## sets up postgres instance in foregroumd

docker run \
    --interactive \
    --tty \
    --env POSTGRES_PASSWORD=example_password \
    --name  postgres_inst \
    --rm \
    --expose 127.0.0.1:8080:8080 \
    postgres:14.3-bullseye
