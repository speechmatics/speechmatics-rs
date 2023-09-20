import sys
import yaml
import re

# Add RT prefix to some names to dedupe from batch
map_names = [
#     "TranscriptionConfig",
#     "TranslationConfig",
#     "SpeakerChangeSensitivity",
#     "DiarizationConfig",
#     "SpeakerDiarizationConfig",
#     "RecognitionResult",
#     "RecognitionMetadata",
#     "RecognitionAlternative",
#     "RecognitionDisplay"
]
    
publish_messages = [
    "StartRecognition","AddAudio","EndOfStream","SetRecognitionConfig"
]

# Open the async spec
with open("../schemas/realtime.yml", 'r') as stream:
    try:
        spec = stream.read()
        for item in map_names:
            # We can't do a naive replace as DiarizationConfig is a substring of SpeakerDiarizationConfig
            spec = re.sub(f"(?![A-Za-z])(.)({item})", r"\1Realtime\2", spec)
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

# We generate an enum of all message types as it is useful to have them all in one place
publish_message_enum = []

# We generate an enum of all message types as it is useful to have them all in one place
subscribe_message_enum = []

# Add the payload field of 'components.messages' as schemas to the generated openapi spec
template["components"] = {"schemas": {}}
for model_name, model_content in messages_models_yaml.items():
    if model_name in publish_messages:
        publish_message_enum.append(model_name)
    else:
        subscribe_message_enum.append(model_name)
    payload = model_content['payload']
    template['components']['schemas'][model_name] = payload

# Append enum to the schemas
template['components']['schemas']['RealtimeMessage'] = {
    'type': "object",
    'properties': {
        'message': {
            "$ref": "#/definitions/Messages"
        }
    },
}

template['components']['schemas']['ServerMessages'] = {
    'type': "string",
    'enum': subscribe_message_enum,
}

template['components']['schemas']['ClientMessages'] = {
    'type': "string",
    'enum': subscribe_message_enum,
}

template['components']['schemas']['Messages'] = {
    'type': "string",
    'enum': subscribe_message_enum,
}

# Add the schemas from async spec to the openapi generated spec
template['components']['schemas'].update(async_spec['components']['schemas'])

# Save the generated openapi spec
with open('openapi-transformed.yaml', 'w') as outfile:
    yaml.dump(template, outfile)
