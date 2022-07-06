#! /usr/bin/env bash


# setup and install pre-commit checks
python3 -m venv .venv
python3 -m pip install pre-commit
pre-commit install -c ./scripts/pre-commit-config.yaml
