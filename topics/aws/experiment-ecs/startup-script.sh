#!/bin/bash

yum install -y aws-cli
aws s3 cp s3://deploy-temp/<%= stackName %>-ecs.config /etc/ecs/ecs.config
