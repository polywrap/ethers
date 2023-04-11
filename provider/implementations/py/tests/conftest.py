from pytest import fixture
from eth_account import Account
from polywrap_client import PolywrapClient, ClientConfig
from polywrap_core import Uri
from polywrap_uri_resolvers import StaticResolver

from ethereum_provider_py import ethereum_provider_plugin
from ethereum_provider_py.connection import ConnectionConfig, Connection
from ethereum_provider_py.connections import Connections


@fixture
def client_without_signer():
    config = ConnectionConfig(
        provider="https://sepolia.infura.io/v3/1a8e6a8ab1df44ccb77d3e954082c5d4",
        signer=None,
    )
    return create_client(config)


@fixture
def client_with_signer():
    signer = Account.from_key(
        "0x4f3edf983ac636a65a842ce7c78d9aa706d3b113bce9c46f30d7d21715b23b1d"
    )

    config = ConnectionConfig(
        provider="https://sepolia.infura.io/v3/1a8e6a8ab1df44ccb77d3e954082c5d4",
        signer=signer,
    )

    return create_client(config)


def create_client(config: ConnectionConfig):
    ethereum_provider_uri = Uri.from_str("plugin/ethereum-provider")
    connection = Connection(config=config)
    connections = Connections(
        networks={"sepolia": connection}, default_network="sepolia"
    )

    resolver = StaticResolver(
        {ethereum_provider_uri: ethereum_provider_plugin(connections=connections)}
    )

    client_config = ClientConfig(resolver=resolver)
    return PolywrapClient(client_config)
