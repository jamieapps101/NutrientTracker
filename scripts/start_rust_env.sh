#! /usr/bin/env bash

# shell script to create a rust container env for rust dev
# add -refresh flag to ask docker to build from dockerfile


if ! [ -z "$NutrientTrackerSetup" ] && [ $NutrientTrackerSetup= 1]; then
    source sourceme.sh
fi

IMAGE_NAME="nutrient_tracker_env"

# build image if required
if [[ $(docker image list | grep -o $IMAGE_NAME ) != $IMAGE_NAME ]] || [[ $1 = "-refresh" ]] ; then
    echo "Did not find $IMAGE_NAME image"
    docker build \
        -f ./scripts/Dockerfile \
        -t $IMAGE_NAME:latest \
        --build-arg DOCKER_USER=$(whoami) \
        .
fi

# run container
docker run \
    -it \
    --network="host" \
    --rm \
    --name nutrient_tracker_container \
    --volume=`pwd`:/workspace \
    --volume /etc/passwd:/etc/passwd:ro \
    --volume /etc/group:/etc/group:ro \
    --user $(id -u):$(id -g) \
    $IMAGE_NAME:latest \
    bash

    # --volume=$HOME/.rustup:/workspace/.cargo \
