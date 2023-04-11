from dataclasses import dataclass
from web3 import Web3
from eth_account.signers.local import LocalAccount
from typing import Optional, Union, cast

from ethereum_provider_py.networks import KnownNetwork, get_network_name

EthereumProvider = str | Web3.HTTPProvider


@dataclass(slots=True, kw_only=True)
class ConnectionConfig:
    provider: EthereumProvider
    signer: Optional[LocalAccount]


class Connection:
    provider: Web3.HTTPProvider
    signer: Optional[LocalAccount]

    def __init__(self, config: ConnectionConfig):
        self.set_provider(config.provider)

        if config.signer:
            self.set_signer(config.signer)

    def set_provider(self, provider: EthereumProvider):
        if isinstance(provider, str):
            self.provider = Web3.HTTPProvider(provider)
        else:
            self.provider = provider

    def set_signer(self, signer: LocalAccount):
        self.signer = signer

    def get_provider(self) -> Web3.HTTPProvider:
        return self.provider
    
    def get_signer(self) -> LocalAccount:
        if not self.signer:
            raise RuntimeError("Signer not found")
        return self.signer

    @staticmethod
    def from_node(node: str):
        config = ConnectionConfig(provider=node, signer=None)
        return Connection(config=config)

    @staticmethod
    def from_network(network: Union[KnownNetwork, int]):
        network_name = (
            get_network_name(network)
            if isinstance(network, int)
            else cast(str, network)
        )
        if not network_name:
            raise RuntimeError(f"Network: {str(network)} not found")

        provider = (
            f"https://{network_name}.infura.io/v3/1a8e6a8ab1df44ccb77d3e954082c5d4"
        )
        config = ConnectionConfig(provider=provider, signer=None)
        return Connection(config=config)
