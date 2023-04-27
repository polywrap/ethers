"""This module contains a list of known networks."""
from enum import IntEnum, unique
from typing import List


@unique
class KnownNetwork(IntEnum):
    """Defines a list of known networks."""
    mainnet = 1, "1", "mainnet"
    goerli = 5, "5", "goerli"
    sepolia = 11155111, "11155111", "sepolia"
    celo_mainnet = 42220, "42220", "celo_mainnet"
    celo_alfajores = 44787, "44787", "celo_alfajores"
    avalanche_mainnet = 43114, "43114", "avalanche_mainnet"
    avalanche_fuji = 43113, "43113", "avalanche_fuji"
    palm_mainnet = 11297108109, "11297108109", "palm_mainnet"
    palm_testnet = 11297108099, "11297108099", "palm_testnet"
    aurora_mainnet = 1313161554, "1313161554", "aurora_mainnet"
    aurora_testnet = 1313161555, "1313161555", "aurora_testnet"

    def __new__(cls, value: int, *aliases: str):
        """Construct a new member of the enum with aliases."""
        obj = int.__new__(cls)
        obj._value_ = value
        for alias in aliases:
            cls._value2member_map_[alias] = obj
        return obj

    @classmethod
    def has(cls, obj: object) -> bool:
        """Returns true if the object is a member of the enum."""
        if isinstance(obj, KnownNetwork):
            return obj.value in cls._value2member_map_
        return obj in cls._value2member_map_

    @classmethod
    def chain_ids(cls) -> List[int]:
        """Returns a list of chain IDs."""
        return [member.value for member in cls]
