from web3 import Web3
from web3.providers.base import JSONBaseProvider
from typing import Optional, Tuple

from polywrap_ethereum_provider.networks import KnownNetwork


class Connection:
    """Defines a connection to an EVM network."""

    __slots__: Tuple[str, str] = ("_provider", "_signer")

    _provider: JSONBaseProvider
    _signer: Optional[str]  # Private key

    def __init__(self, provider: JSONBaseProvider | str, signer: Optional[str]):
        self._provider = Web3.HTTPProvider(provider) if isinstance(provider, str) else provider
        self._signer = signer
    
    @property
    def provider(self) -> JSONBaseProvider:
        return self._provider

    @property
    def signer(self) -> str:
        if not self._signer:
            raise ValueError(f"signer is not set for {self}")
        return self._signer

    @classmethod
    def from_node(cls, node: str, signer: Optional[str] = None):
        return cls(provider=node, signer=signer)

    @classmethod
    def from_network(cls, network: KnownNetwork, signer: Optional[str] = None):
        provider = (
            f"https://{network.name}.infura.io/v3/1a8e6a8ab1df44ccb77d3e954082c5d4"
        )
        return cls(provider=provider, signer=signer)

    def __str__(self) -> str:
        return f"Connection: {self.provider}"

    def has_signer(self) -> bool:
        return self._signer is not None 
