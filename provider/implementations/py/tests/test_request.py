from polywrap_client import PolywrapClient
from polywrap_core import InvokerOptions, UriPackageOrWrapper, Uri
import json

provider_uri = Uri.from_str("plugin/ethereum-provider")


async def test_eth_chain_id(client_without_signer: PolywrapClient):
    options: InvokerOptions[UriPackageOrWrapper] = InvokerOptions(
        uri=provider_uri,
        method="request",
        args={"method": "eth_chainId"},
        encode_result=False,
    )
    result = await client_without_signer.invoke(options)

    assert result == json.dumps("0xaa36a7")


async def test_eth_get_transaction_count(client_without_signer: PolywrapClient):
    options: InvokerOptions[UriPackageOrWrapper] = InvokerOptions(
        uri=provider_uri,
        method="request",
        args={
            "method": "eth_getTransactionCount",
            "params": '["0xf3702506acec292cfaf748b37cfcea510dc37714","latest"]',
        },
        encode_result=False,
    )
    result = await client_without_signer.invoke(options)

    assert int(json.loads(result), base=16) > 0


async def test_sign_transaction(client_with_signer: PolywrapClient):
    domain = {
        "name": "Ether Mail",
        "version": "1",
        "chainId": 1,
        "verifyingContract": "0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC",
    }

    types = {
        "EIP712Domain": [
            {"type": "string", "name": "name"},
            {"type": "string", "name": "version"},
            {
                "type": "uint256",
                "name": "chainId",
            },
            {
                "type": "address",
                "name": "verifyingContract",
            },
        ],
        "Person": [
            {"name": "name", "type": "string"},
            {"name": "wallet", "type": "address"},
        ],
        "Mail": [
            {"name": "from", "type": "Person"},
            {"name": "to", "type": "Person"},
            {"name": "contents", "type": "string"},
        ],
    }

    message = {
        "from": {"name": "Cow", "wallet": "0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826"},
        "to": {"name": "Bob", "wallet": "0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB"},
        "contents": "Hello, Bob!",
    }

    params = json.dumps(
        [
            "0x90F8bf6A479f320ead074411a4B0e7944Ea8c9C1",
            {
                "domain": domain,
                "primaryType": "Mail",
                "types": types,
                "message": message,
            },
        ]
    )

    options: InvokerOptions[UriPackageOrWrapper] = InvokerOptions(
        uri=provider_uri,
        method="request",
        args={
            "method": "eth_signTypedData_v4",
            "params": params,
        },
        encode_result=False,
    )
    result = await client_with_signer.invoke(options)

    assert result == json.dumps(
        "0x12bdd486cb42c3b3c414bb04253acfe7d402559e7637562987af6bd78508f38623c1cc09880613762cc913d49fd7d3c091be974c0dee83fb233300b6b58727311c"
    )


async def test_encode_packed_int16_uint48(client_without_signer: PolywrapClient):
    options: InvokerOptions[UriPackageOrWrapper] = InvokerOptions(
        uri=provider_uri,
        method="request",
        args={
            "method": "eth_encodePacked",
            "params": json.dumps({
                "types": ["int16", "uint48"],
                "values": ["-1", "12"],
            }),
        },
        encode_result=False,
    )
    result = await client_without_signer.invoke(options)

    assert result == json.dumps("0xffff00000000000c")


async def test_encode_packed_uint256_uint256(client_without_signer: PolywrapClient):
    options: InvokerOptions[UriPackageOrWrapper] = InvokerOptions(
        uri=provider_uri,
        method="request",
        args={
            "method": "eth_encodePacked",
            "params": json.dumps({
                "types": ["uint256", "uint256"],
                "values": ["8", "16"],
            }),
        },
        encode_result=False,
    )
    result = await client_without_signer.invoke(options)

    assert result == json.dumps("0x00000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000010")


async def test_encode_packed_string_uint8(client_without_signer: PolywrapClient):
    options: InvokerOptions[UriPackageOrWrapper] = InvokerOptions(
        uri=provider_uri,
        method="request",
        args={
            "method": "eth_encodePacked",
            "params": json.dumps({
                "types": ["string", "uint8"],
                "values": ["Hello", "3"],
            }),
        },
        encode_result=False,
    )
    result = await client_without_signer.invoke(options)

    assert result == json.dumps("0x48656c6c6f03")
