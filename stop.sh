#! /bin/bash
docker compose -f alphanet/docker-compose.yml down --remove-orphans
rm -r alphanet/lib
rm -r alphanet/log
