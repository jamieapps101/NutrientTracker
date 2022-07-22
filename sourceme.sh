#! /usr/bin/env bash

# setup and install pre-commit checks
python3 -m venv .venv
python3 -m pip install pre-commit
pre-commit install -c ./scripts/pre-commit-config.yaml

# setup build cache dir for docker build environment
export CARGO_HOME=`pwd`/.cargo
mkdir -p .cargo
export RUSTUP_HOME=`pwd`/.rustup
mkdir -p .rustup

# set env var to signal this file has been sourced
export NutrientTrackerSetup=1
