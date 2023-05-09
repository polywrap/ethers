# General thoughts:
- How should we handle the ethers-rs dependency between local crates?
- Remove redundant exposed methods from core wrap
- Move transaction & provider helpers to core crate
- Create basic building blocks to interact with ethereum, for example:
     - User input is a contract address, method, and args
     - enconde the transaction into a raw transaction
     - (sign if needed)
     - send or call the raw transaction
- Remove `ProviderError` from wrap, we're not interacting directly with providers from ethers-rs, but rather through the provider module

# New structure

Current exposed methods from ethereum wrap:

### Utility methods
- checkAddress(address: String, connection: Connection) -> Boolean

### Transaction methods
- awaitTransaction(txHash: String, confirmations: UInt32, timeout: UInt32, connection: Connection) -> TxReceipt
- callContractView(address: String, method: String, args: [String], connection: Connection) -> String
- sendTransaction(tx: TxRequest, connection: Connection) -> TxResponse
- sendTransactionAndWait(tx: TxRequest, connection: Connection) -> TxReceipt
- deployContract(abi: String, bytecode: String, args: [String], options: TxOptions, connection: Connection) -> String
- estimateContractCallGas(address: String, method: String, args: [String], options: TxOptions, connection: Connection) -> BigInt
- callContractMethod(address: String, method: String, args: [String], options: TxOptions, connection: Connection) -> TxResponse
- callContractMethodAndWait(address: String, method: String, args: [String], options: TxOptions, connection: Connection) -> TxReceipt

### Provider methods (RPC interactions: Dependent of ethereum wallet plugin)
- getBalance(address: String, blockTag: BigInt, connection: Connection) -> String
- getChainId(connection: Connection) -> String
- estimateTransactionGas(tx: TxRequest, connection: Connection) -> BigInt
- sendRpc(method: String, params: [String], connection: Connection) -> String
- estimateEip1559Fees(connection: connection) -> Eip1559FeesEstimate
- getGasPrice(connection: Connection) -> String

#### Signing methods
- signMessage(message: String, connection: Connection) -> String
- signMessageBytes(bytes: Bytes, connection: Connection) -> String
- signTransaction(tx: TxRequest, connection: Connection) -> String
- signTypedData(payload: JSON, connection: Connection) -> String
- getSignerAddress(connection: Connection) -> String
- getSignerBalance(connection: Connection) -> String
- getSignerTransactionCount(blockTag: BigInt, connection: Connection) -> BigInt

# Proposed exposed methods

### Provider methods (RPC interactions: Dependent of HTTP plugin)
- request(method: String, params: [String], connection: Connection) -> String
    - getBalance(address: String, blockTag: BigInt, connection: Connection) -> String
    - getChainId(connection: Connection) -> String
    - estimateEip1559Fees(connection: connection) -> Eip1559FeesEstimate
    - getGasPrice(connection: Connection) -> String

### Transaction methods
- encodeTransaction(tx: TxRequest) -> String
- awaitTransaction(txHash: String, confirmations: UInt32, timeout: UInt32, connection: Connection) -> TxReceipt
- estimateGas(o: String, data: String, options: TxOptions, connection: Connection) -> BigInt
- call(to: String, data: String, options: TxOptions, connection: Connection) -> String

## Signer methods (RPC interactions: Dependent of ethereum wallet plugin)
- signMessage(message: String, connection: Connection) -> String
- signTransaction(tx: TxRequest, connection: Connection) -> String
- signTypedData(payload: JSON, connection: Connection) -> String
- send(to: String, data: String, options: TxOptions, connection: Connection) -> TxResponse


