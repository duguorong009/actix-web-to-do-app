#! /usr/bin/env bash
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH
cd ..
docker build -t rust_app .
docker tag rust_app:latest test/actix_web_application:latest

docker login
docker push test/actix_web_application:latest