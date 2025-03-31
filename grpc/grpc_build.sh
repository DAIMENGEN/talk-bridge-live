#!/bin/bash

# Define path variables

# Path to the protoc executable
PROTOC="./grpc/protoc/protoc-3.20.0-win64/bin/protoc"

# Path to the directory containing .proto files
PROTO_PATH="./grpc/protos"

# Output directory for generated code
OUT_DIR="./grpc/protos"

# Name of the .proto file to compile (can pass multiple files separated by spaces)
PROTO_FILE="chat.proto"

# Execute the Protobuf compilation command
$PROTOC --proto_path=$PROTO_PATH $PROTO_FILE \
    --js_out=import_style=commonjs:$OUT_DIR \
    --grpc-web_out=import_style=typescript,mode=grpcwebtext:$OUT_DIR

# Print a success message when the code generation is complete
echo "gRPC code generation completed successfully!"
