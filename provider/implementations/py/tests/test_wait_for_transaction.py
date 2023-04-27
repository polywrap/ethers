import asyncio
from typing import Any, Callable, Dict
from polywrap_client import PolywrapClient
from polywrap_core import InvokeOptions, UriPackageOrWrapper, Uri, InvokerOptions
from pytest import fixture
import pytest
from web3 import Web3
from .utils import Benchmark, mine_blocks
from concurrent.futures import ThreadPoolExecutor


WithSigner = bool
provider_uri = Uri.from_str("plugin/ethereum-provider")
pool = ThreadPoolExecutor()


@fixture
def w3(provider: Any):
    return Web3(provider)


@fixture
def test_tx(w3: Web3):
    transaction: Dict[str, Any] = {
        'from': "0x8dFf5E27EA6b7AC08EbFdf9eB090F32ee9a30fcf",
        'to': "0xcb93799A0852d94B65166a75d67ECd923fD951E4",
        'value': 1000,
        'gas': 21000,
        'gasPrice': 50000000000,
        'nonce': 0,
    }
    return w3.eth.send_transaction(transaction).hex() # type: ignore



async def test_wait_for_transaction_no_confirmations(client_factory: Callable[[WithSigner], PolywrapClient], test_tx: str):
    args= {
        "txHash": test_tx,
        "confirmations": 0,
        "connection": {
            "networkNameOrChainId": "mocknet",
        }
    }
    options: InvokeOptions[UriPackageOrWrapper] = InvokerOptions(
        uri=provider_uri,
        method="waitForTransaction",
        args=args,
        encode_result=False,
    )
    client = client_factory(True)
    result = await client.invoke(options)

    assert result == True


async def test_wait_for_transaction_ten_confirmations(client_factory: Callable[[WithSigner], PolywrapClient], test_tx: str, w3: Web3):
    confirmations = 10
    block_time = 0.1
    args= {
        "txHash": test_tx,
        "confirmations": confirmations,
        "connection": {
            "networkNameOrChainId": "mocknet",
        }
    }
    options: InvokeOptions[UriPackageOrWrapper] = InvokerOptions(
        uri=provider_uri,
        method="waitForTransaction",
        args=args,
        encode_result=False,
    )
    client = client_factory(True)

    with Benchmark() as b:
        pool.submit(mine_blocks, w3, 10, block_time)
        result = pool.submit(asyncio.run, client.invoke(options)).result()

    assert result == True
    assert b.elapsed > block_time * confirmations


async def test_wait_for_transaction_timeout(client_factory: Callable[[WithSigner], PolywrapClient], test_tx: str, w3: Web3):
    confirmations = 10
    block_time = 0.1
    args= {
        "txHash": test_tx,
        "confirmations": confirmations,
        "timeout": 0.5,
        "connection": {
            "networkNameOrChainId": "mocknet",
        }
    }
    options: InvokeOptions[UriPackageOrWrapper] = InvokerOptions(
        uri=provider_uri,
        method="waitForTransaction",
        args=args,
        encode_result=False,
    )
    client = client_factory(True)

    with pytest.raises(Exception) as e:
        pool.submit(mine_blocks, w3, 10, block_time)
        pool.submit(asyncio.run, client.invoke(options)).result()

    assert isinstance(e.value.__cause__, TimeoutError)