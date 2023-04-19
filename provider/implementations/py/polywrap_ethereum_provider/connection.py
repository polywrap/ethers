from eth_account import Account
from web3 import Web3
from web3.providers.base import JSONBaseProvider
from typing import Optional

from polywrap_ethereum_provider.networks import KnownNetwork


class Connection:
    """Defines a connection to an EVM network."""

    __slots__ = ("_provider", "_signer")

    _provider: JSONBaseProvider
    _signer: Optional[Account]

    def __init__(self, provider: JSONBaseProvider | str, signer: Optional[Account]):
        self._provider = Web3.HTTPProvider(provider) if isinstance(provider, str) else provider
        self._signer = signer
    
    @property
    def provider(self) -> JSONBaseProvider:
        return self._provider

    @property
    def signer(self) -> Account:
        if not self._signer:
            raise ValueError(f"signer is not set for {self}")
        return self._signer

    @classmethod
    def from_node(cls, node: str):
        return cls(provider=node, signer=None)

    @classmethod
    def from_network(cls, network: KnownNetwork):
        provider = (
            f"https://{network.name}.infura.io/v3/1a8e6a8ab1df44ccb77d3e954082c5d4"
        )
        return cls(provider=provider, signer=None)

    def __str__(self) -> str:
        return f"Connection: {self.provider}"
