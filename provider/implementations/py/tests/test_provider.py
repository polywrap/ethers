from polywrap_client import PolywrapClient
from polywrap_core import InvokerOptions, UriPackageOrWrapper, Uri
import json

async def test_request(client: PolywrapClient):
    options: InvokerOptions[UriPackageOrWrapper] = InvokerOptions(
        uri=Uri.from_str("plugin/ethereum-provider"),
        method="request",
        args={"method": "eth_chainId"},
        encode_result=False,
    )
    result = await client.invoke(options)

    assert result == json.dumps("0xaa36a7")
