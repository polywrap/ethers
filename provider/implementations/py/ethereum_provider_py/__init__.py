from polywrap_core import Invoker, UriPackageOrWrapper, Env
from polywrap_plugin import PluginModule, PluginPackage

from eth_account.messages import encode_structured_data
from web3.types import RPCEndpoint
import json
from typing import Optional, Dict, Any, cast

from ethereum_provider_py.connections import Connections


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
            parsed_array = json.loads(cast(str, params))
            structured_data = encode_structured_data(primitive=parsed_array[1])
            signer = connection.get_signer()
            signed_message = signer.sign_message(structured_data)  # type: ignore
            return json.dumps(signed_message.signature.hex()) # type: ignore

        response = provider.make_request(
            method=RPCEndpoint(method), params=json.loads(cast(str, params))
        )
        error = response.get("error")
        if error:
            raise RuntimeError(error)
        return json.dumps(response.get("result"))


def ethereum_provider_plugin(connections: Connections) -> PluginPackage[Connections]:
    return PluginPackage(module=EthereumProviderPlugin(connections=connections), manifest={})  # type: ignore
