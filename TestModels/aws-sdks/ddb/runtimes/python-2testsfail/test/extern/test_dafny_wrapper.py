"""
Wrapper file for executing Dafny tests from pytest.
This allows us to import modules required by Dafny-generated tests
before executing Dafny-generated tests.
pytest will find and execute the `test_dafny` method below,
which will execute the `test.py` file in the `dafny` directory.
"""

# Import modules required for Dafny-generated tests.
# This is not generated; these must be manually added.

from com_amazonaws_dynamodb.extern import wrapped_ddb_shim

# End import modules required for Dafny-generated tests

def test_dafny():
  # Dafny tests are executed when importing `internaldafny_test_executor`
  import internaldafny_test_executor