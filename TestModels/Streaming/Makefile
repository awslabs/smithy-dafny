# Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0

CORES=2

USE_DAFNY_STANDARD_LIBRARIES=1

include ../SharedMakefile.mk

NAMESPACE=simple.streaming

PROJECT_SERVICES := \
	SimpleStreaming

SERVICE_NAMESPACE_SimpleStreaming=simple.streaming

SERVICE_DEPS_SimpleStreaming :=

SMITHY_DEPS=dafny-dependencies/Model/traits.smithy

# This project has no dependencies 
# DEPENDENT-MODELS:= 

# Python

PYTHON_MODULE_NAME=simple_streaming

TRANSLATION_RECORD_PYTHON := \
    --translation-record ../dafny-dependencies/StandardLibrary/runtimes/python/src/smithy_dafny_standard_library/internaldafny/generated/dafny_src-py.dtr

# LIBRARIES :=
