"""This module contains a connections class for an EVM network."""
from typing import Dict, Optional, Tuple, cast

from polywrap_ethereum_provider.connection import Connection
from polywrap_ethereum_provider.networks import KnownNetwork
from polywrap_ethereum_provider.wrap.types import Connection as SchemaConnection


class Connections:
    """Defines a set of connections to EVM networks."""
    __slots__: Tuple[str, str, str] = ("connections", "default_network", "signer")

    connections: Dict[str, Connection]
    default_network: str
    signer: Optional[str]

    def __init__(
        self,
        connections: Dict[str, Connection],
        default_network: Optional[str],
        signer: Optional[str] = None,
    ):
        """Initialize a set of connections to EVM networks."""
        self.connections = connections
        self.signer = signer

        if default_network:
            if default_network not in self.connections:
                raise ValueError(
                    f"Default network: {default_network} not in connections"
                )
            self.default_network = default_network
        elif "mainnet" in self.connections:
            self.default_network = "mainnet"
        else:
            self.default_network = "mainnet"
            self.connections["mainnet"] = Connection.from_network(
                KnownNetwork.mainnet
            )

    def get_connection(self, connection: Optional[SchemaConnection]) -> Connection:
        """Get a connection from a connection object."""
        if not connection:
            return self.with_signer(self.connections[self.default_network])

        if connection.get("networkNameOrChainId"):
            network = cast(str, connection["networkNameOrChainId"]).lower()
            if network in self.connections:
                return self.with_signer(self.connections[network])
            if KnownNetwork.has(network):
                if network in self.connections:
                    return self.with_signer(self.connections[network])
                return Connection.from_network(KnownNetwork[network], self.signer)
            raise ValueError(
                f"Network: {network} isn't a known network!\n"
                f"\tUse one of: {KnownNetwork.chain_ids()}\n"
                f"\tor set a custom RPC URL using the 'node' field."
            )

        if connection.get("node"):
            node = cast(str, connection["node"])
            return Connection.from_node(node, self.signer)

        return self.with_signer(self.connections[self.default_network])

    def with_signer(self, connection: Connection) -> Connection:
        """Return a connection with a signer."""
        if self.signer and not connection.has_signer():
            return Connection(connection.provider, self.signer)
        return connection
