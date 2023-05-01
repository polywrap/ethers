import metamask_ios_sdk
import Foundation
import Combine
import PolywrapClient


public class MetamaskProvider: Plugin {
    var provider: Ethereum
    var cancellables: Set<AnyCancellable> = []
    var result: Data? = nil;

    public init(ethereum: Ethereum, dapp: Dapp) {
        self.provider = ethereum
        
        self.provider.connect(dapp)?.sink(receiveCompletion: { completion in
            switch completion {
            case let .failure(error):
                print("Error connecting to address: \(error)")
            default: break
            }
        }, receiveValue: { value in
            print("Wallet connected! \(value)")
        }).store(in: &cancellables)

        super.init()
        super.addMethod(name: "request", closure: request)
        super.addMethod(name: "waitForTransaction", closure: waitForTransaction)
        super.addMethod(name: "signerAddress", closure: signerAddress)
        super.addMethod(name: "chainId", closure: chainId)
    }
    
    func executeRequest(publisher: EthereumPublisher?, completion: @escaping (Result<String, Error>) -> Void) -> Void {
        publisher?.sink(receiveCompletion: { completionResult in
            switch completionResult {
            case .finished:
                break
            case let .failure(error):
                return completion(.failure(error))
            }
        }, receiveValue: { value in
            if let response = value as? String {
                 return completion(.success("\"\(response)\""))
            }
            
            if let tx = value as? Transaction {
                let encoder = JSONEncoder()
                let jsonData = try! encoder.encode(tx)
                let string = String(data: jsonData, encoding: .utf8)!
                return completion(.success(string))

            }
        }).store(in: &cancellables)
   }
    
    func request(args: ArgsRequest, completion: @escaping (Result<String, Error>) -> Void) {
        if !provider.connected {
            return completion(.failure(ProviderError.NotConnected))
        }

        let initialParamsTx: [Transaction] = []
        if self.isTransactionMethod(args.method) {
            if args.method == "eth_call" {
                if let params = args.params {
                    let json = params.data(using: .utf8)!
                    let jsonDecoder = JSONDecoder()
                    let mixedArray = try! jsonDecoder.decode([TxOrString].self, from: json)
                    
                    let transaction = mixedArray[0].toTransaction()!
                    handleEthCall(transaction: transaction, completion: completion)
                    
                } else {
                    // todo make error
                    print("Params must be in eth_call")
                }
            } else {
                var request = EthereumRequest(method: args.method, params: initialParamsTx)
                if let jsonData = args.params {
                    let json = jsonData.data(using: .utf8)!
                    let params: [Transaction] = try! JSONDecoder().decode(
                        [Transaction].self,
                        from: json
                    )
                    request = EthereumRequest(method: args.method, params: params)
                }
                let publisher = provider.request(request)
                executeRequest(publisher: publisher, completion: completion)
            }
        } else {
            if args.method == "eth_getBlockByNumber" {
                handleGetBlockByNumber(block: "latest", includeTx: false, completion: completion)
            } else if args.method == "eth_signTypedData_v4" {
                if let params = args.params {
                    let json = params.data(using: .utf8)!
                    let jsonDecoder = JSONDecoder()
                    let mixedArray = try! jsonDecoder.decode([AddressOrTypedData].self, from: json)
                    let params = [mixedArray[0].toString(), mixedArray[1].toString()]
                    let request = EthereumRequest(method: args.method, params: params)
                    let publisher = provider.request(request)
                    executeRequest(publisher: publisher, completion: completion)
                }
            } else if args.method == "eth_feeHistory" {
                handleFeeHistory(blockCount: "0xa", newestBlock: "latest", rewardPercentiles: [5.0], completion: completion)
            } else {
                if let params = args.params {
                    if params != "null" {
                        let jsonData = params.data(using: .utf8)!
                        let params: [String] = try! JSONDecoder().decode([String].self, from: jsonData)
                        let request = EthereumRequest(method: args.method, params: params)
                        let publisher = provider.request(request)
                        executeRequest(publisher: publisher, completion: completion)
                    } else {
                        let request = EthereumRequest(method: args.method, params: "")
                        let publisher = provider.request(request)
                        executeRequest(publisher: publisher, completion: completion)
                    }
                } else {
                    let request = EthereumRequest(method: args.method, params: "")
                    let publisher = provider.request(request)
                    executeRequest(publisher: publisher, completion: completion)
                }
            }
        }
    }

    public func request(args: ArgsRequest) async -> String {
        await withCheckedContinuation { continuation in
            request(args: args) { result in
                switch result {
                case .success(let value):
                    continuation.resume(returning: value)
                case .failure(let error):
                   print("Error in request: \(error)")
                }
            }
        }
    }
    private var delayUnit: DelayUnit = .shortest

    
    // Inspired from https://github.com/web3swift-team/web3swift/blob/develop/Sources/web3swift/Transaction/TransactionPollingTask.swift#L11
    // Probably it would be easier to add this library and use it?
    public func waitForTransaction(args: ArgsWaitForTransaction) async -> Bool {
        let startTime = Date()
        while true {
            do {
                let jsonParams = try JSONSerialization.data(withJSONObject: [args.txHash], options: [])
                let request = ArgsRequest(
                    method: "eth_getTransactionReceipt",
                    params: String(data: jsonParams, encoding: .utf8)!
                )
                let receipt = await self.request(args: request)
                if receipt != "" {
                    if  try JSONSerialization.jsonObject(with: receipt.data(using: .utf8)!, options: []) is [String: Any] {
                        // Successfully converted Data to [String: Any]
                        // TODO: Handle confirmations
                        return true
                    }
                    
                    if delayUnit.shouldIncreaseDelay(startTime) {
                        delayUnit = delayUnit.nextDelayUnit
                    }
                }
                try await Task.sleep(nanoseconds: delayUnit.rawValue)
            } catch {
                print("Error in JSON Serialization from wait for transaction method \(error)")
            }


        }
    }
    
    public func signTransaction(_ args: ArgsSignTransaction) async throws -> String {
        throw ProviderError.MethodNotSupported
    }

    public func signMessage(_ args: ArgsSignMessage) async throws -> String {
        throw ProviderError.MethodNotSupported
    }
    
    public func signerAddress(_ args: ArgsAddress) async -> String {
        self.provider.selectedAddress
    }

    public func chainId(_ args: ArgsChainId) async -> String {
        self.provider.chainId
    }
    
    private func isTransactionMethod(_ method: String) -> Bool {
        let transactionMethods = [
            "eth_sendTransaction",
            "eth_estimateGas",
            "eth_call"
        ]
        
        return transactionMethods.contains(method)
    }
    
    // Metamask does not handle well arguments with different types, this is a hack to make the plugin work as expected
    // but this should not be the case. The way it's commented in the readme doesn't really work:
    // https://github.com/MetaMask/metamask-ios-sdk#using-a-struct
    private func handleGetBlockByNumber(block: String, includeTx: Bool, completion: @escaping (Result<String, Error>) -> Void) {
        let jsonData = try? JSONSerialization.data(withJSONObject:[
            "id": "1",
            "jsonrpc":"2.0",
            "method": "eth_getBlockByNumber",
            "params": [block, includeTx]
        ])

        let httpRequest = buildHttpRequest(body: jsonData)
        handleHttpRequest(req: httpRequest, completion: completion)
    }
    
    private func handleFeeHistory(blockCount: String, newestBlock: String, rewardPercentiles: [Float]?, completion: @escaping (Result<String, Error>) -> Void) {
        let jsonData = try? JSONSerialization.data(withJSONObject:[
            "id": "1",
            "jsonrpc":"2.0",
            "method": "eth_feeHistory",
            "params": [blockCount, newestBlock, rewardPercentiles]
        ])

        let httpRequest = buildHttpRequest(body: jsonData)
        handleHttpRequest(req: httpRequest, completion: completion)
    }
    
    private func handleEthCall(transaction: Transaction, completion: @escaping (Result<String, Error>) -> Void) {
        let jsonData = try? JSONSerialization.data(withJSONObject:[
            "id": "1",
            "jsonrpc":"2.0",
            "method": "eth_call",
            "params": [
                [
                    "to": transaction.to!,
                    "data": transaction.data,
                    "type": transaction.type!
                ]
            ]
        ])

        let httpRequest = buildHttpRequest(body: jsonData)
        handleHttpRequest(req: httpRequest, completion: completion)
    }
    
    private enum DelayUnit: UInt64 {
        case shortest = 1
        case medium = 5
        case longest = 60

        func shouldIncreaseDelay(_ startTime: Date) -> Bool {
            let timePassed = Date().timeIntervalSince1970 - startTime.timeIntervalSince1970
            switch self {
            case .shortest:
                return timePassed > 10
            case .medium:
                return timePassed > 120
            case .longest:
                return false
            }
        }

        var nextDelayUnit: DelayUnit {
            switch self {
            case .shortest:
                return .medium
            case .medium, .longest:
                return .longest
            }
        }
    }
}

extension Data {
    var hexString: String {
        return map { String(format: "%02x", $0) }.joined()
    }
}
