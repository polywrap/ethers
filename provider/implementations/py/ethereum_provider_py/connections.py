from typing import Dict, Optional

from ethereum_provider_py.networks import KnownNetworkId, KnownNetwork, get_network_name
from ethereum_provider_py.connection import Connection
from ethereum_provider_py.wrap.types import Connection as SchemaConnection


class Connections:
    connections: Dict[str, Connection]
    default_network: str

    def __init__(self, networks: Dict[str, Connection], default_network: Optional[str]):
        self.connections = {}
        for network, connection in networks.items():
            self.set(network, connection)

        if default_network:
            self.set_default_network(default_network)
        elif "mainnet" in self.connections:
            self.set_default_network("mainnet")
        else:
            self.set_default_network(
                "mainnet", Connection.from_network(KnownNetwork.mainnet)
            )

    def get(self, network: Optional[str]) -> Optional[Connection]:
        if not network:
            return self.connections[self.default_network.lower()]
        else:
            return self.connections[network.lower()]

    def set(self, network: str, connection: Connection):
        network = network.lower()
        try:
            network_id = int(network)
            network_name = get_network_name(network_id)
            if not network_name:
                raise RuntimeError(f"Chain ID: {network_id} not valid")
            self.connections[network_name] = connection
        except ValueError:
            self.connections[network] = connection

    def set_default_network(
        self, network: str, connection: Optional[Connection] = None
    ):
        if connection:
            self.set(network, connection)

        if not self.get(network):
            raise RuntimeError(f"No connection found for network: {network}")

        self.default_network = network

    def get_default_network(self) -> str:
        return self.default_network

    def get_connection(self, connection: Optional[SchemaConnection]) -> Connection:
        if not connection:
            default_connection = self.get(self.default_network)
            if not default_connection:
                raise RuntimeError("Default connection not found")

            return default_connection

        result: Optional[Connection] = None

        if hasattr(connection, "network_name_or_chain_id"):
            network_str = connection.network_name_or_chain_id.lower()  # type: ignore
            network = self.get(network_str)
            if network:
                result = network
            else:
                try:
                    chain_id = int(network_str)
                    result = Connection.from_network(chain_id)
                except ValueError:
                    if network in KnownNetworkId.__members__:
                        result = Connection.from_network(KnownNetwork[network_str])
                    else:
                        result = self.get(self.default_network)

        # if hasattr(connection, "node"):

        if not result:
            raise RuntimeError("No connection found")
        else:
            return result
