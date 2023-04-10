import pytest
from ethereum_provider_py import EthereumProvider
from polywrap_result import Ok

async def test_sign_transaction():
    provider = EthereumProvider()
    await provider.request() == Ok("works")