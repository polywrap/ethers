from polywrap_plugin import PluginModule, PluginPackage

from web3.types import RPCEndpoint
import json

from ethereum_provider_py.connections import Connections
from ethereum_provider_py.wrap.types import ArgsRequest


class EthereumProviderPlugin(PluginModule[Connections]):
    def __init__(self, connections: Connections):
        super().__init__(connections)
        self.connections = connections

    async def request(self, args: ArgsRequest) -> str:
        connection = self.connections.get_connection(args.connection)
        provider = connection.get_provider()
        response = provider.make_request(
            method=RPCEndpoint(args.method), params=args.params
        )
        error = response.get("error")
        if error:
            raise RuntimeError(error)
        return json.dumps(response.get("result"))


def ethereum_provider_plugin(connections: Connections) -> PluginPackage[Connections]:
    return PluginPackage(module=EthereumProviderPlugin(connections=connections), manifest={})  # type: ignore
