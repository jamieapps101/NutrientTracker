#! /usr/bin/env bash

## sets up example postgres instance in foregroumd

docker run \
    --interactive \
    --tty \
    --env POSTGRES_PASSWORD=example_password \
    --name  postgres_inst \
    --rm \
    --expose 8080 \
    --network="host" \
    postgres:14.3-bullseye
