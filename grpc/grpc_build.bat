@echo off

:: Set the path to the protoc executable
set PROTOC=grpc\protoc\protoc-3.20.0-win64\bin\protoc

:: Set the path where .proto files are located
set PROTO_PATH=grpc\protos

:: Set the output directory where generated code will be placed
set OUT_DIR=grpc\protos

:: Define the .proto files to compile (can pass multiple files separated by spaces)
set PROTO_FILE=chat.proto

:: Run the protoc command to generate JavaScript and gRPC-Web code from the .proto file
%PROTOC% --proto_path=%PROTO_PATH% %PROTO_FILE% --js_out=import_style=commonjs:%OUT_DIR% --grpc-web_out=import_style=typescript,mode=grpcwebtext:%OUT_DIR%

:: Print a message when the code generation is complete
echo gRPC code generation completed successfully!
