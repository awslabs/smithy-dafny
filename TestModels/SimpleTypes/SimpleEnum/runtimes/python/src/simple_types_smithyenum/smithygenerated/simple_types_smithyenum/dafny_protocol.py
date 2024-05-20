# Code generated by smithy-python-codegen DO NOT EDIT.

import module_
from simple_types_smithyenum_internaldafny_types import (
    GetEnumInput_GetEnumInput as DafnyGetEnumInput,
)


import Wrappers
from typing import Union

class DafnyRequest:
    operation_name: str

    # dafny_operation_input can take on any one of the types
    # of the input values passed to the Dafny implementation
    dafny_operation_input: Union[
        DafnyGetEnumInput,
    ]

    def __init__(self, operation_name, dafny_operation_input):
        self.operation_name = operation_name
        self.dafny_operation_input = dafny_operation_input

class DafnyResponse(Wrappers.Result):
    def __init__(self):
        super().__init__(self)
