#!/bin/bash

OUTPUT_MODELS_DIRECTORY='../speechmatics/src/models/realtime'

pip3 install -r requirements.txt

# Get a 'fake' openapi spec from the async realtime-api spec (we just need the schemas)
python3 transform_async_to_openapi.py

# Generate models from the generated openapi spec using the openapi-generator tool
openapi-generator generate -i openapi-transformed.yaml -g rust -o ./openapi_models_tmp -c ../autogen.json
mkdir -p ${OUTPUT_MODELS_DIRECTORY}
rm -r  ${OUTPUT_MODELS_DIRECTORY}/*
mv ./openapi_models_tmp/src/models/* ${OUTPUT_MODELS_DIRECTORY}

# Delete temp files
rm openapi-transformed.yaml
rm -rf openapi_models_tmp
