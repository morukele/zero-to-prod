#!/usr/bin/env bash
set -x
set -eo pipefail

# if a redis container is running, print instructions to kill it and exit
RUNNING_CONTAINER=$(docker ps --filter 'name=redis' --format '{{.ID}}')
if [[ -n $RUNNING_CONTAINER ]]; then
    echo >&2 "there is a redis container already running, kill it with"
    echo >&2 "      docker kill ${RUNNING_CONTAINER}"
    exit 1
fi 

# Lunch Redis container using docker
docker run \
    -p "6379:6379" \
    -d \
    --name "redis_$(data '+%s')" \
    redis:6

>&2 echo "Redis is ready to go!"