from polywrap_core import Invoker, UriPackageOrWrapper, Env
from polywrap_plugin import PluginModule, PluginPackage

from web3.types import RPCEndpoint
import json
from typing import Optional, Dict, Any

from ethereum_provider_py.connections import Connections


class EthereumProviderPlugin(PluginModule[Connections]):
    def __init__(self, connections: Connections):
        super().__init__(connections)
        self.connections = connections

    async def request(
        self,
        args: Dict[str, Any], # TODO(cbrzn): Use generated types
        client: Invoker[UriPackageOrWrapper],
        env: Optional[Env] = None,
    ) -> str:
        connection = self.connections.get_connection(args.get("connection"))
        provider = connection.get_provider()
        response = provider.make_request(
            method=RPCEndpoint(args["method"]), params=args.get("params")
        )
        error = response.get("error")
        if error:
            raise RuntimeError(error)
        return json.dumps(response.get("result"))


def ethereum_provider_plugin(connections: Connections) -> PluginPackage[Connections]:
    return PluginPackage(module=EthereumProviderPlugin(connections=connections), manifest={})  # type: ignore
