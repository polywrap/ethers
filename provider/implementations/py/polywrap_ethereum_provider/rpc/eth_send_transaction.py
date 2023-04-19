import json
from typing import Tuple, TypedDict, Optional

Transaction = TypedDict('Transaction', {
    # DATA, 20 Bytes - The address the transaction is sent from.
    'from': str,
    
    # DATA, 20 Bytes - (optional when creating new contract) The address the transaction is directed to.
    'to': Optional[str],
    
    # QUANTITY - (optional, default: 90000) Integer of the gas provided for the transaction execution. It will return unused gas.
    'gas': Optional[str],
    
    # QUANTITY - (optional, default: To-Be-Determined) Integer of the gasPrice used for each paid gas.
    'gasPrice': Optional[str],
    
    # QUANTITY - (optional) Integer of the value sent with this transaction.
    'value': Optional[str],
    
    # DATA - The compiled code of a contract OR the hash of the invoked method signature and encoded parameters.
    'data': str,
    
    # QUANTITY - (optional) Integer of a nonce. This allows to overwrite your own pending transactions that use the same nonce.
    'nonce': Optional[str],
}, total=False)


def deserialize_parameters(input: str) -> Tuple[Transaction]:
    params = json.loads(input)
    if len(params) < 1 or not isinstance(params[0], dict):
        raise ValueError(
            "Invalid JSON-RPC parameters provided for eth_sendTransaction method. Reference: "
            "https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_sendtransaction"
        )

    transaction: Transaction = params[0]

    if 'from' not in transaction:
        raise ValueError(
            "The 'from' property on the transaction object parameter is required for the eth_sendTransaction method. Reference: "
            "https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_sendtransaction"
        )

    if 'data' not in transaction:
        raise ValueError(
            "The 'data' property on the transaction object parameter is required for the eth_sendTransaction method. Reference: "
            "https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_sendtransaction"
        )

    return (transaction,)
