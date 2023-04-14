from polywrap_core import Invoker, UriPackageOrWrapper, Env
from polywrap_plugin import PluginModule, PluginPackage

from eth_account.messages import encode_structured_data
from web3 import Web3
from web3.types import RPCEndpoint
import json
from typing import Optional, Dict, Any, cast

from polywrap_ethereum_provider.connections import Connections


class EthereumProviderPlugin(PluginModule[Connections]):
    def __init__(self, connections: Connections):
        super().__init__(connections)
        self.connections = connections

    async def request(
        self,
        args: Dict[str, Any],  # TODO(cbrzn): Use generated types
        client: Invoker[UriPackageOrWrapper],
        env: Optional[Env] = None,
    ) -> str:
        connection = self.connections.get_connection(args.get("connection"))
        provider = connection.get_provider()
        method = args["method"]
        params = args.get("params") if args.get("params") else "[]"

        if method == "eth_signTypedData_v4":
            signer = connection.get_signer()
            data = json.loads(cast(str, params))
            structured_data = encode_structured_data(primitive=data[1])
            signed_message = signer.sign_message(structured_data)  # type: ignore
            return json.dumps(signed_message.signature.hex())  # type: ignore

        if method == "eth_sendTransaction":
            web3 = Web3(provider)
            signer = connection.get_signer()
            signed_transaction = web3.eth.account.sign_transaction(json.loads(params), signer.key)  # type: ignore
            hash = web3.eth.send_raw_transaction(signed_transaction)
            return json.dumps(hash)

        response = provider.make_request(
            method=RPCEndpoint(method), params=json.loads(cast(str, params))
        )
        error = response.get("error")
        if error:
            raise RuntimeError(error)
        return json.dumps(response.get("result"))

    # TODO(cbrzn): Handle confirmations & timeout
    async def wait_for_transaction(
        self,
        args: Dict[str, Any],
        client: Invoker[UriPackageOrWrapper],
        env: Optional[Env] = None,
    ) -> bool:
        connection = self.connections.get_connection(args.get("connection"))
        provider = connection.get_provider()
        web3 = Web3(provider)
        web3.eth.wait_for_transaction_receipt(args["tx_hash"])
        return True

    async def signer_address(
        self,
        args: Dict[str, Any],
        client: Invoker[UriPackageOrWrapper],
        env: Optional[Env] = None,
    ) -> Optional[str]:
        try:
            connection = self.connections.get_connection(args.get("connection"))
            signer = connection.get_signer()
            return signer.address  # type: ignore
        except:
            return None


def ethereum_provider_plugin(connections: Connections) -> PluginPackage[Connections]:
    return PluginPackage(module=EthereumProviderPlugin(connections=connections), manifest={})  # type: ignore
