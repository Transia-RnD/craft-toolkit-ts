#! /bin/bash
docker compose -f smartnet/docker-compose.yml down --remove-orphans
rm -r smartnet/lib
rm -r smartnet/log
