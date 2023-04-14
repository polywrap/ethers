from enum import Enum, unique
from typing import Optional

@unique
class KnownNetworkId(Enum):
    mainnet = 1
    goerli = 5
    sepolia = 11155111
    celo_mainnet = 42220
    celo_alfajores = 44787
    avalanche_mainnet = 43114
    avalanche_fuji = 43113
    palm_mainnet = 11297108109
    palm_testnet = 11297108099
    aurora_mainnet = 1313161554
    aurora_testnet = 1313161555

KnownNetwork = KnownNetworkId

def get_network_name(id: int) -> Optional[str]:
    for member in KnownNetworkId:
        if member.value == id:
            return member.name