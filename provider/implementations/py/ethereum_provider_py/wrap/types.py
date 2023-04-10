from dataclasses import dataclass
from typing import Optional

@dataclass(slots=True, kw_only=True)
class Connection:
    node: Optional[str]
    network_name_or_chain_id: Optional[str]


@dataclass(slots=True, kw_only=True)
class ArgsRequest:
    method: str
    params: str
    connection: Optional[Connection]


# @dataclass(slots=True, kw_only=True)
# class ArgsWaitForTransaction(TypedDict):
#     tx_hash: str
#     confirmations: int
#     timeout: Optional[int]
#     connection: Optional[Connection]
