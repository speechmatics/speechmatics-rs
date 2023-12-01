#!/bin/bash

OUTPUT_BATCH_MODELS_DIRECTORY='../src/batch/models'
OUTPUT_RT_MODELS_DIRECTORY='../src/realtime/models'


pip3 install -r requirements.txt

# Get a 'fake' openapi spec from the async realtime-api spec (we just need the schemas)
python3 transform_async_to_openapi.py

# Generate models from the generated openapi spec using the openapi-generator tool
openapi-generator generate -i openapi-transformed.yaml -g rust -o ./openapi_models_tmp -c ../autogen.json

# Due to type being a reserved word, the type generator converts it to RHashType, so let's change it to something more friendly
find ./openapi_models_tmp/src/models/ -name '*.rs' -exec sed -i '' -e 's/RHashType/Type/g' {} \;
find ./openapi_models_tmp/src/models/ -name '*.rs' -exec sed -i '' -e 's/r#type/type_value/g' {} \;

find ./openapi_models_tmp/src/models/ -name '*.rs' -exec sed -i '' -e 's/#\[derive(Clone, Debug, PartialEq, Serialize, Deserialize)\]/#\[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)\]/g' {} \;

# Change the path to models so that it matches our crate structure
find ./openapi_models_tmp/src/models/ -name '*.rs' -exec sed -i '' -e 's/crate::models/crate::realtime::models/g' {} \;

mkdir -p ${OUTPUT_RT_MODELS_DIRECTORY}
rm -r  ${OUTPUT_RT_MODELS_DIRECTORY}/*
cp ./openapi_models_tmp/src/models/* ${OUTPUT_RT_MODELS_DIRECTORY}

# Delete temp files
rm openapi-transformed.yaml
rm -rf openapi_models_tmp

# Generate models from the generated openapi spec using the openapi-generator tool
openapi-generator generate -i ../schemas/batch.yml -g rust -o ./openapi_models_tmp -c ../autogen.json

# Due to type being a reserved word, the type generator converts it to RHashType, so let's change it to something more friendly
find ./openapi_models_tmp -name '*.rs' -exec sed -i '' -e 's/RHashType/Type/g' {} \;
find ./openapi_models_tmp -name '*.rs' -exec sed -i '' -e 's/r#type/type_value/g' {} \;

find ./openapi_models_tmp/src/models/ -name '*.rs' -exec sed -i '' -e 's/#\[derive(Clone, Debug, PartialEq, Serialize, Deserialize)\]/#\[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)\]/g' {} \;

# Change the path to models so that it matches our crate structure
find ./openapi_models_tmp -name '*.rs' -exec sed -i '' -e 's/crate::models/crate::batch::models/g' {} \;

mkdir -p ${OUTPUT_BATCH_MODELS_DIRECTORY}
rm -r  ${OUTPUT_BATCH_MODELS_DIRECTORY}/*
mv ./openapi_models_tmp/src/models/* ${OUTPUT_BATCH_MODELS_DIRECTORY}

# Delete temp files
rm -rf openapi_models_tmp
