#!/bin/bash

# List requested command
echo "$@"

# List environment - TODO: Remove as this will contain API Key
env

# Write out config
echo "$CONFIG" > sendgrid.connector.configuration.json

# Kick off the requested command
"$@"