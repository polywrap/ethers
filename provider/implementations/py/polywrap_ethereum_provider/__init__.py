from polywrap_core import Invoker, UriPackageOrWrapper, Env
from polywrap_plugin import PluginModule, PluginPackage

from eth_account.messages import encode_structured_data  # type: ignore
from eth_account.account import SignedMessage, SignedTransaction
from web3 import Web3
from web3.types import RPCEndpoint
import json
from typing import Optional
from .wrap import ArgsRequest, manifest

from polywrap_ethereum_provider.connections import Connections


class EthereumProviderPlugin(PluginModule[Connections]):
    def __init__(self, connections: Connections):
        super().__init__(connections)
        self.connections = connections

    async def request(
        self,
        args: ArgsRequest,
        client: Invoker[UriPackageOrWrapper],
        env: Optional[Env] = None,
    ) -> str:
        connection = self.connections.get_connection(args.get("connection"))
        web3 = Web3(connection.provider)

        method = args["method"]
        params = json.loads(args.get("params") or "[]")

        if method == "eth_signTypedData_v4":
            structured_data = encode_structured_data(primitive=params[1])
            signed_message: SignedMessage = web3.eth.account.sign_message(structured_data, connection.signer)
            return json.dumps(signed_message.signature.hex())  

        if method == "eth_sendTransaction":
            signed_transaction: SignedTransaction = web3.eth.account.sign_transaction(params, connection.signer)  
            tx_hash = web3.eth.send_raw_transaction(signed_transaction.rawTransaction)
            return json.dumps(tx_hash.hex())

        response = connection.provider.make_request(
            method=RPCEndpoint(method), params=params)
        if error := response.get("error"):
            raise RuntimeError(error)
        return json.dumps(response.get("result"))

    # # TODO(cbrzn): Handle confirmations & timeout
    # async def wait_for_transaction(
    #     self,
    #     args: Dict[str, Any],
    #     client: Invoker[UriPackageOrWrapper],
    #     env: Optional[Env] = None,
    # ) -> bool:
    #     connection = self.connections.get_connection(args.get("connection"))
    #     provider = connection.get_provider()
    #     web3 = Web3(provider)
    #     web3.eth.wait_for_transaction_receipt(args["tx_hash"])
    #     return True

    # async def signer_address(
    #     self,
    #     args: Dict[str, Any],
    #     client: Invoker[UriPackageOrWrapper],
    #     env: Optional[Env] = None,
    # ) -> Optional[str]:
    #     try:
    #         connection = self.connections.get_connection(args.get("connection"))
    #         signer = connection.get_signer()
    #         return signer.address  
    #     except:
    #         return None


def ethereum_provider_plugin(connections: Connections) -> PluginPackage[Connections]:
    return PluginPackage(module=EthereumProviderPlugin(connections=connections), manifest=manifest)  
