#! /usr/bin/env bash

## sets up example postgres instance in foregroumd

docker run \
    --interactive \
    --tty \
    --env POSTGRES_PASSWORD=example_password \
    --env POSTGRES_USER=nt_user \
    --env POSTGRES_DB=nt_db \
    --name  postgres_inst \
    --rm \
    --network="host" \
    postgres:14.3-bullseye
    # --expose=5432 \
