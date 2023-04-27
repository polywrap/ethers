from typing import Any
from pytest import fixture
from eth_account import Account
from eth_account.account import LocalAccount
from polywrap_client import PolywrapClient, ClientConfig
from polywrap_core import Uri
from polywrap_uri_resolvers import StaticResolver
from web3 import EthereumTesterProvider

from polywrap_ethereum_provider import ethereum_provider_plugin
from polywrap_ethereum_provider.connection import Connection
from polywrap_ethereum_provider.connections import Connections
from polywrap_ethereum_provider.networks import KnownNetwork


@fixture
def provider():
    return EthereumTesterProvider()


@fixture
def account():
    return Account.from_key(
        "0x4f3edf983ac636a65a842ce7c78d9aa706d3b113bce9c46f30d7d21715b23b1d"
    )


@fixture
def client_factory(provider: Any, account: LocalAccount):
    def factory(with_signer: bool) -> PolywrapClient:
        ethereum_provider_uri = Uri.from_str("plugin/ethereum-provider")
        connections = Connections(
            connections={
                "mocknet": Connection(provider, None),
                "sepolia": Connection.from_network(KnownNetwork.sepolia, None)
            },
            default_network="sepolia",
            signer=account.key if with_signer else None,  # type: ignore
        )

        resolver = StaticResolver(
            {ethereum_provider_uri: ethereum_provider_plugin(connections=connections)}
        )

        client_config = ClientConfig(resolver=resolver)
        return PolywrapClient(client_config)
    return factory

