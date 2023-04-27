
from time import time, sleep
from typing import Any

from web3 import Web3
from web3.types import Wei, RPCEndpoint


class Benchmark:
    start: float
    end: float
    elapsed: float

    def __enter__(self):
        self.start = time()
        return self

    def __exit__(self, *args: Any):
        self.end = time()
        self.elapsed = self.end - self.start


def mine_blocks(w3: Web3, num_blocks: int, block_time: float):
    for _ in range(num_blocks):
        sleep(block_time)
        w3.provider.make_request(RPCEndpoint("evm_mine"), [Wei(1)])
