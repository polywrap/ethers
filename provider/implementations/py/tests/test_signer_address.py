from typing import Callable
from polywrap_client import PolywrapClient
from polywrap_core import InvokerOptions, UriPackageOrWrapper, Uri
from eth_account.account import LocalAccount

WithSigner = bool
provider_uri = Uri.from_str("plugin/ethereum-provider")


async def test_signer_address(client_factory: Callable[[WithSigner], PolywrapClient], account: LocalAccount):
    options: InvokerOptions[UriPackageOrWrapper] = InvokerOptions(
        uri=provider_uri,
        method="signerAddress",
        args={},
        encode_result=False,
    )
    client = client_factory(True)
    result = await client.invoke(options)

    assert result == account.address  # type: ignore


async def test_signer_address_no_signer(client_factory: Callable[[WithSigner], PolywrapClient]):
    options: InvokerOptions[UriPackageOrWrapper] = InvokerOptions(
        uri=provider_uri,
        method="signerAddress",
        args={},
        encode_result=False,
    )
    client = client_factory(False)
    result = await client.invoke(options)

    assert result is None