from typing import Any, List
import json

from web3 import Web3
from eth_account.messages import encode_structured_data # type: ignore
from eth_account.datastructures import SignedMessage

from polywrap_ethereum_provider.rpc import EthEncodePacked

from .connection import Connection

class EthereumRequest:

    def __init__(self, connection: Connection) -> None:
        self.signer = connection.signer
        self.w3 = Web3(connection.provider)

    def eth_chainId(self):
        return self.w3.eth.chain_id

    def eth_sendTransaction(self, params: List[Any]):
        tx_signed = self.w3.eth.account.sign_transaction(params, self.signer.key) # type: ignore
        tx_hash = self.w3.eth.send_raw_transaction(tx_signed)
        return tx_hash.hex()

    def eth_signTypedData_v4(self, params: List[Any]):
        structured_data = encode_structured_data(primitive=params[1])
        signed_message: SignedMessage = self.signer.sign_message(structured_data)  
        return signed_message.signature.hex()

    def eth_encodePacked(self, params: List[Any]):
        pass
        # EthEncodePacked.deserialize_parameters(params)
        # return encode_abi(types, values, packed=True)
