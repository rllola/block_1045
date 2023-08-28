#!/bin/sh

install-deps:
	# We need a specific protoc version
	wget https://github.com/protocolbuffers/protobuf/releases/download/v3.16.3/protoc-3.16.3-linux-x86_64.zip
	unzip protoc-3.16.3-linux-x86_64.zip -d ./protoc

run:
	PATH="${PWD}/protoc/bin:${PATH}" cargo r