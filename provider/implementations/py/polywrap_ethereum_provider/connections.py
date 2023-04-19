from typing import Dict, Optional, cast

from polywrap_ethereum_provider.networks import KnownNetwork
from polywrap_ethereum_provider.connection import Connection
from polywrap_ethereum_provider.wrap.types import Connection as SchemaConnection


class Connections:
    connections: Dict[KnownNetwork, Connection]
    default_network: KnownNetwork

    def __init__(
        self,
        connections: Dict[KnownNetwork, Connection],
        default_network: Optional[KnownNetwork],
    ):
        self.connections = connections

        if default_network:
            if default_network not in self.connections:
                raise ValueError(
                    f"Default network: {default_network} not in connections"
                )
            self.default_network = default_network
        elif KnownNetwork.mainnet in self.connections:
            self.default_network = KnownNetwork.mainnet
        else:
            self.default_network = KnownNetwork.mainnet
            self.connections[KnownNetwork.mainnet] = Connection.from_network(
                KnownNetwork.mainnet
            )

    def get_connection(self, connection: Optional[SchemaConnection]) -> Connection:
        if not connection:
            return self.connections[self.default_network]

        if connection.get("networkNameOrChainId"):
            network = cast(str, connection["networkNameOrChainId"]).lower()
            if KnownNetwork.has(network):
                known_network = KnownNetwork(network)
                if known_network in self.connections:
                    return self.connections[known_network]
                return Connection.from_network(KnownNetwork[network])
            raise ValueError(
                f"Network: {network} isn't a known network!\n"
                f"\tUse one of: {KnownNetwork.chain_ids()}\n"
                f"\tor set a custom RPC URL using the 'node' field."
            )

        if connection.get("node"):
            node = cast(str, connection["node"])
            return Connection.from_node(node)

        return self.connections[self.default_network]
