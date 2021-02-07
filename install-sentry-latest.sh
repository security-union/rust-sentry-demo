#!/bin/bash

output_folder="sentry-client"

tags=$(curl \
  -H "Accept: application/vnd.github.v3+json" \
  https://api.github.com/repos/getsentry/onpremise/tags)

tag=$(eval echo $(echo $tags | jq '.[0] | .name'))

mkdir $output_folder

wget -c "http://github.com/getsentry/onpremise/archive/$tag.tar.gz" -O - | tar -xz -C "./$output_folder" --strip-components 1

cd $output_folder

chmod +x install.sh

./install.sh