#!/bin/bash

cd openapi-generator
./mvnw package
cd ..
curl -O https://raw.githubusercontent.com/zulip/zulip/master/zerver/openapi/zulip.yaml
java -jar openapi-generator/modules/openapi-generator-cli/target/openapi-generator-cli.jar \
    generate --input-spec zulip.yaml --generator-name rust -o zapi_gen