from pytest import fixture
from polywrap_client import PolywrapClient, ClientConfig
from polywrap_core import Uri
from polywrap_uri_resolvers import StaticResolver

from ethereum_provider_py import ethereum_provider_plugin
from ethereum_provider_py.connection import ConnectionConfig, Connection
from ethereum_provider_py.connections import Connections

@fixture
def client():
    ethereum_provider_uri = Uri.from_str("plugin/ethereum-provider")

    config = ConnectionConfig(provider="https://sepolia.infura.io/v3/1a8e6a8ab1df44ccb77d3e954082c5d4", signer=None)
    connection = Connection(config=config)
    connections = Connections(networks={"sepolia": connection}, default_network="sepolia")

    resolver = StaticResolver({ ethereum_provider_uri: ethereum_provider_plugin(connections=connections)})

    config = ClientConfig(resolver=resolver)
    return PolywrapClient(config)