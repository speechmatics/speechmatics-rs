import sys
import yaml
import re

    
publish_messages = [
    "StartRecognition","AddAudio","EndOfStream","SetRecognitionConfig"
]

# Open the async spec
with open("../schemas/realtime.yml", 'r') as stream:
    try:
        spec = stream.read()
        async_spec = yaml.safe_load(spec)
    except yaml.YAMLError as exc:
        print(exc)

# Open a basic openapi template as starting document
with open("template-openapi.yaml", 'r') as stream:
    try:
        template = yaml.safe_load(stream)
    except yaml.YAMLError as exc:
        print(exc)

messages_models_yaml = async_spec['components']['messages']

# Add the payload field of 'components.messages' as schemas to the generated openapi spec
template["components"] = {"schemas": {}}
for model_name, model_content in messages_models_yaml.items():
    payload = model_content['payload']
    template['components']['schemas'][model_name] = payload

# Add the schemas from async spec to the openapi generated spec
template['components']['schemas'].update(async_spec['components']['schemas'])

# Save the generated openapi spec
with open('openapi-transformed.yaml', 'w') as outfile:
    yaml.dump(template, outfile)
