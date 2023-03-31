import metamask_ios_sdk
import Foundation
import Combine
import PolywrapClient
import SocketIO

public struct Transaction: CodableData {
    let from: String?
    let data: String
    let type: String?
    let value: String?
    let to: String?


    public init(to: String? = nil, from: String? = nil, value: String? = nil, data: String, type: String? = nil) {
        self.to = to
        self.from = from
        self.value = value
        self.data = data
        self.type = type
    }
    
    public init?(json: [String: Any]) {
        guard let data = json["data"] as? String
        else {
            return nil
        }
        
        
        self.data = data
        
        if let type = json["type"] as? String {
            self.type = type
        } else {
            self.type = nil
        }

        if let from = json["from"] as? String {
            self.from = from
        } else {
            self.from = nil
        }
        
        if let to = json["to"] as? String {
            self.to = to
        } else {
            self.to = nil
        }
        
        if let value = json["value"] as? String {
            self.value = value
        } else {
            self.value = nil
        }
    }

    public func socketRepresentation() -> NetworkData {
        [
            "to": to,
            "from": from,
            "value": value,
            "data": data
        ]
    }
}

public struct ArgsWaitForTransaction: Codable {
    let txHash: String
    let confirmations: UInt32
    let timeout: UInt32?
    let connection: String?
    
    public init(txHash: String, confirmations: UInt32, timeout: UInt32? = nil) {
        self.txHash = txHash
        self.confirmations = confirmations
        self.timeout = timeout
        self.connection = nil
    }
}

public struct ArgsRequest: Codable {
    var method: String;
    var params: String?;
    var connection: String?;
    
    public init(method: String, params: String? = "") {
        self.method = method
        self.params = params
        self.connection = nil
    }
}

public struct ArgsSignMessage {
    var message: [UInt8]
    var connection: Any?;
    
    public init(message: [UInt8]) {
        self.message = message
        self.connection = nil
    }
}

public struct ArgsSignTransaction {
    var rlp: [UInt8]
    var connection: Any?;
    
    public init(rlp: [UInt8]) {
        self.rlp = rlp
        self.connection = nil
    }
}

public struct ArgsAddress: Codable {
    public init() {}
}


public struct ArgsChainId: Codable {
    public init() {}
}

enum ProviderError: Error {
    case NotConnected
    case EncodeError
    case MethodNotSupported
    case DataCorruptedError
}
enum StringOrBool: Codable {
    case string(String)
    case bool(Bool)
        
    init(from decoder: Decoder) throws {
        let container = try decoder.singleValueContainer()
        if let stringValue = try? container.decode(String.self) {
            self = .string(stringValue)
        } else if let boolValue = try? container.decode(Bool.self) {
            self = .bool(boolValue)
        } else {
            throw ProviderError.DataCorruptedError
            
        }
    }
    
   public func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()
        switch self {
        case .string(let stringValue):
            try container.encode(stringValue)
        case .bool(let boolValue):
            try container.encode(boolValue)
        }
    }
    
    public func toString() -> String {
        switch self {
        case .string(let v):
            return v
        case .bool(let b):
            if b {
                return "true"
            } else {
                return "false"
            }
        }
        
    }
}

enum TxOrString: Codable {
    case string(String)
    case transaction(Transaction)
    
    init(from decoder: Decoder) throws {
        let container = try decoder.singleValueContainer()
        
        if let txValue = try? container.decode(Transaction.self) {
            self = .transaction(txValue)
        } else if let stringValue = try? container.decode(String.self) {
            self = .string(stringValue)
        } else {
            throw DecodingError.dataCorruptedError(
                in: container,
                debugDescription: "Unable to decode Element"
            )
        }
    }
        
    func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()

        switch self {
        case .string(let stringValue):
            try container.encode(stringValue)
        case .transaction(let txValue):
            try container.encode(txValue)
        }
    }
    
    func toString() -> String {
        switch self {
        case .transaction(let v):
            let encoder = JSONEncoder()
            let jsonData = try! encoder.encode(v)
            let string = String(data: jsonData, encoding: .utf8)!
            return string
        case.string(let v):
            return v
        }
    }
    
    func toTransaction() -> Transaction? {
        switch self {
        case .transaction(let v):
            return v
        case .string(_):
            return nil
        }
    }
}

public struct ParamsEthCall: CodableData {
    public var tx: Transaction
    public var tag: String
    
    public init(tx: Transaction, tag: String) {
        self.tx = tx
        self.tag = tag
    }
    
    public func socketRepresentation() throws -> SocketData {
        return [
         [
            "data": self.tx.data,
            "type": self.tx.type,
            "to": self.tx.to
         ], self.tag
        ]
    }
}

public struct CustomBoolOrStringArray: CodableData {
    let tag: String
    let include: Bool
    
    public func socketRepresentation() -> NetworkData {
        [self.tag, self.include]
    }
}

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
        
    
    func handleGetBlockByNumber(block: String, includeTx: Bool, completion: @escaping (Result<String, Error>) -> Void) {
        let endpoint = URL(string: "https://eth-goerli.g.alchemy.com/v2/xs7E_AOsOwBTRspEDnkoldxihsKveaOn")!
        var httpRequest = URLRequest(url: endpoint)
        httpRequest.httpMethod = "post"
        
        let jsonData = try? JSONSerialization.data(withJSONObject:[
            "id": "1",
            "jsonrpc":"2.0",
            "method": "eth_getBlockByNumber",
            "params": [block, includeTx]
        ])

        httpRequest.httpBody = jsonData
        
        httpRequest.addValue("application/json", forHTTPHeaderField: "accept")
        httpRequest.addValue("application/json", forHTTPHeaderField: "content-type")
       let task = URLSession.shared.dataTask(with: httpRequest) { (data, response, error) in
            if let error = error {
                print("error :/ \(error)")
                return completion(.failure(error))
            } else if let data = data {
                let json = try! JSONSerialization.jsonObject(with: data, options: []) as! [String : Any]
                let stringJson = try! JSONSerialization.data(withJSONObject: json["result"])
                return completion(.success(String(data:stringJson, encoding: .utf8)!))
            } else {
                print("unexpected error")
            }
        }
           
       task.resume()
    }
    
    func handleFeeHistory(blockCount: String, newestBlock: String, rewardPercentiles: [Float]?, completion: @escaping (Result<String, Error>) -> Void) {
        let endpoint = URL(string: "https://eth-goerli.g.alchemy.com/v2/xs7E_AOsOwBTRspEDnkoldxihsKveaOn")!
        var httpRequest = URLRequest(url: endpoint)
        httpRequest.httpMethod = "post"
        
        let jsonData = try? JSONSerialization.data(withJSONObject:[
            "id": "1",
            "jsonrpc":"2.0",
            "method": "eth_feeHistory",
            "params": [blockCount, newestBlock, rewardPercentiles]
        ])

        httpRequest.httpBody = jsonData
        
        httpRequest.addValue("application/json", forHTTPHeaderField: "accept")
        httpRequest.addValue("application/json", forHTTPHeaderField: "content-type")
       let task = URLSession.shared.dataTask(with: httpRequest) { (data, response, error) in
            if let error = error {
                print("error :/ \(error)")
                return completion(.failure(error))
            } else if let data = data {
                let json = try! JSONSerialization.jsonObject(with: data, options: []) as! [String : Any]
                let stringJson = try! JSONSerialization.data(withJSONObject: json["result"])
                return completion(.success(String(data:stringJson, encoding: .utf8)!))
            } else {
                print("unexpected error")
            }
        }
           
       task.resume()
    }
    
    func handleEthCall(transaction: Transaction, completion: @escaping (Result<String, Error>) -> Void) {
        let endpoint = URL(string: "https://eth-goerli.g.alchemy.com/v2/xs7E_AOsOwBTRspEDnkoldxihsKveaOn")!
        var httpRequest = URLRequest(url: endpoint)
        httpRequest.httpMethod = "post"
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

        httpRequest.httpBody = jsonData
        
        httpRequest.addValue("application/json", forHTTPHeaderField: "accept")
        httpRequest.addValue("application/json", forHTTPHeaderField: "content-type")
       let task = URLSession.shared.dataTask(with: httpRequest) { (data, response, error) in
            if let error = error {
                return completion(.failure(error))
            } else if let data = data {
                let json = try! JSONSerialization.jsonObject(with: data, options: []) as! [String : String]
                let result = json["result"]!
                return completion(.success("\"\(result)\""))
            } else {
                print("unexpected error")
            }
        }
           
       task.resume()
    }
    
    func request(args: ArgsRequest, completion: @escaping (Result<String, Error>) -> Void) {
        if !provider.connected {
            return completion(.failure(ProviderError.NotConnected))
        }

        let initialParamsTx: [Transaction] = []
        if self.isTransactionMethod(args.method) {
            if args.method == "eth_call" {
                if let params = args.params {
                    print("params: \(params)")
//                    print("transaction maybe? \(params[0])")
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
                executeRequest(publisher: publisher) { result in
                    switch result {
                    case .success(let response):
                        return completion(.success(response))
                    case .failure(let error):
                        return completion(.failure(error))
                    }
                }
            }
        } else {
            if args.method == "eth_getBlockByNumber" {
                handleGetBlockByNumber(block: "latest", includeTx: false, completion: completion)
            } else if args.method == "eth_feeHistory" {
                handleFeeHistory(blockCount: "0xa", newestBlock: "latest", rewardPercentiles: [5.0], completion: completion)
            } else {
                if let params = args.params {
                    if params != "null" {
                        let jsonData = params.data(using: .utf8)!
                        let params: [String] = try! JSONDecoder().decode([String].self, from: jsonData)
                        let request = EthereumRequest(method: args.method, params: params)
                        let publisher = provider.request(request)
                        executeRequest(publisher: publisher) { result in
                            switch result {
                            case .success(let response):
                                return completion(.success(response))
                            case .failure(let error):
                                return completion(.failure(error))
                            }
                        }
                    } else {
                        let request = EthereumRequest(method: args.method, params: "")
                        let publisher = provider.request(request)
                        executeRequest(publisher: publisher) { result in
                            switch result {
                            case .success(let response):
                                return completion(.success(response))
                            case .failure(let error):
                                return completion(.failure(error))
                            }
                        }
                    }
                } else {
                    let request = EthereumRequest(method: args.method, params: "")
                    let publisher = provider.request(request)
                    executeRequest(publisher: publisher) { result in
                        switch result {
                        case .success(let response):
                            return completion(.success(response))
                        case .failure(let error):
                            return completion(.failure(error))
                        }
                    }
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
    
    func isTransactionMethod(_ method: String) -> Bool {
        let transactionMethods = [
            "eth_sendTransaction",
            "eth_estimateGas",
            "eth_call"
        ]
        
        return transactionMethods.contains(method)
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
