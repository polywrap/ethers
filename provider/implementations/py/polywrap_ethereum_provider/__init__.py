"""This package provides a Polywrap plugin for interacting with EVM networks."""
# pylint: disable=no-value-for-parameter
# pylint: disable=protected-access
import json
from typing import Optional, cast

from eth_account import Account
from eth_account._utils.signing import sign_message_hash  # type: ignore
from eth_account.datastructures import SignedMessage, SignedTransaction
from eth_account.messages import encode_defunct, encode_structured_data  # type: ignore
from eth_utils.crypto import keccak
from hexbytes import HexBytes
from polywrap_core import Env, InvokerClient, UriPackageOrWrapper
from polywrap_plugin import PluginPackage
from web3 import Web3
from web3._utils.threads import Timeout
from web3.exceptions import TransactionNotFound
from web3.types import RPCEndpoint

from polywrap_ethereum_provider.connections import Connections

from .wrap import (
    ArgsRequest,
    ArgsSignerAddress,
    ArgsSignMessage,
    ArgsSignTransaction,
    ArgsWaitForTransaction,
    Module,
    manifest,
)


class EthereumProviderPlugin(Module[Connections]):
    """A Polywrap plugin for interacting with EVM networks."""
    def __init__(self, connections: Connections):
        super().__init__(connections)
        self.connections = connections

    async def request(
        self,
        args: ArgsRequest,
        client: InvokerClient[UriPackageOrWrapper],
        env: Optional[Env] = None,
    ) -> str:
        """Send a remote RPC request to the registered provider."""
        connection = self.connections.get_connection(args.get("connection"))
        web3 = Web3(connection.provider)

        method = args["method"]
        params = json.loads(args.get("params") or "[]")

        if method == "eth_signTypedData_v4":
            structured_data = encode_structured_data(primitive=params[1])
            signed_message: SignedMessage = web3.eth.account.sign_message(
                structured_data, connection.signer
            )
            return json.dumps(signed_message.signature.hex())

        if method == "eth_sendTransaction":
            signed_transaction: SignedTransaction = web3.eth.account.sign_transaction(
                params, connection.signer
            )
            tx_hash = web3.eth.send_raw_transaction(signed_transaction.rawTransaction)
            return json.dumps(tx_hash.hex())

        response = connection.provider.make_request(
            method=RPCEndpoint(method), params=params
        )
        if error := response.get("error"):
            raise RuntimeError(error)
        return json.dumps(response.get("result"))

    async def signer_address(
        self,
        args: ArgsSignerAddress,
        client: InvokerClient[UriPackageOrWrapper],
        env: Optional[Env] = None,
    ) -> Optional[str]:
        """Get the ethereum address of the signer. Return null if signer is missing."""
        connection = self.connections.get_connection(args.get("connection"))
        if connection.has_signer():
            return Account.from_key(
                connection.signer
            ).address
        return None

    async def wait_for_transaction(
        self,
        args: ArgsWaitForTransaction,
        client: InvokerClient[UriPackageOrWrapper],
        env: Optional[Env] = None,
    ) -> bool:
        """Wait for a transaction to be mined."""
        connection = self.connections.get_connection(args.get("connection"))
        web3 = Web3(connection.provider)
        poll_latency = 0.1
        confirmation_latency = 0.5
        timeout = args.get("timeout", 300)

        try:
            with Timeout(cast(float, timeout)) as _timeout:
                # Wait for the transaction receipt
                while (
                    tx_receipt := await self._get_transaction_receipt(args, client, env)
                ) is None:
                    _timeout.sleep(poll_latency)

                # Get the block number of the transaction
                tx_block_number = tx_receipt["block_number"]

                # Calculate the target block number
                target_block_number = tx_block_number + args.get("confirmations", 0)

                # Wait for the blockchain to reach the target block number
                while web3.eth.block_number < target_block_number:
                    _timeout.sleep(confirmation_latency)
            return True
        except Timeout as e:
            raise TimeoutError("Transaction timed out") from e

    async def sign_message(
        self,
        args: ArgsSignMessage,
        client: InvokerClient[UriPackageOrWrapper],
        env: None,
    ) -> str:
        """Sign a message and return the signature. Throws if signer is missing."""
        connection = self.connections.get_connection(args.get("connection"))
        web3 = Web3(connection.provider)
        signable_message = encode_defunct(args["message"])
        signed_message: SignedMessage = web3.eth.account.sign_message(
            signable_message, connection.signer
        )
        return signed_message.signature.hex()

    async def sign_transaction(
        self,
        args: ArgsSignTransaction,
        client: InvokerClient[UriPackageOrWrapper],
        env: None,
    ) -> str:
        """
        Sign a serialized unsigned transaction and return the signature.\
        Throws if signer is missing.\
        This method requires a wallet-based signer with a private key,\
        and is not needed for most use cases.\
        Typically, transactions are sent by `request` and signed by the wallet.
        """
        connection = self.connections.get_connection(args.get("connection"))
        tx_hash = keccak(args["rlp"])
        account = Account.from_key(
            connection.signer
        )
        key_obj = account._key_obj  # type: ignore
        (v, r, s, eth_signature_bytes) = sign_message_hash(key_obj, tx_hash)  # type: ignore
        return HexBytes(cast(bytes, eth_signature_bytes)).hex()

    async def _get_transaction_receipt(
        self,
        args: ArgsWaitForTransaction,
        client: InvokerClient[UriPackageOrWrapper],
        env: Optional[Env] = None,
    ):
        connection = self.connections.get_connection(args.get("connection"))
        try:
            response = connection.provider.make_request(
                method=RPCEndpoint("eth_getTransactionReceipt"), params=[args["txHash"]]
            )
            if error := response.get("error"):
                raise RuntimeError(error)
            return response.get("result")
        except TransactionNotFound:
            return None


def ethereum_provider_plugin(connections: Connections) -> PluginPackage[Connections]:
    """Create a Polywrap plugin instance for interacting with EVM networks."""
    return PluginPackage(
        module=EthereumProviderPlugin(connections=connections), manifest=manifest
    )
