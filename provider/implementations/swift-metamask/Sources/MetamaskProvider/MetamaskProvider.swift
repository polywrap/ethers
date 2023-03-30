import metamask_ios_sdk
import Foundation
import Combine
import PolywrapClient

public struct Transaction: CodableData {
    let to: String?
    let from: String
    let value: String?
    let data: String

    public init(to: String? = nil, from: String, value: String? = nil, data: String) {
        self.to = to
        self.from = from
        self.value = value
        self.data = data
    }
    
    public init?(json: [String: Any]) {
        guard let from = json["from"] as? String,
              let data = json["data"] as? String
        else {
            return nil
        }
        
        
        self.data = data
        self.from = from
        
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
    var params: String;
    var connection: String?;
    
    public init(method: String, params: String) {
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
        super.addMethod(name: "address", closure: address)
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
                print("response")
                print(response)
                return completion(.success(response))
            }
        }).store(in: &cancellables)
   }
        
    func request(args: ArgsRequest, completion: @escaping (Result<String, Error>) -> Void) {
        if !provider.connected {
            return completion(.failure(ProviderError.NotConnected))
        }
        
        if self.isTransactionMethod(args.method) {
            let jsonData = args.params.data(using: .utf8)!
            let params: [Transaction] = try! JSONDecoder().decode(
                [Transaction].self,
                from: jsonData
            )

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
            let jsonData = args.params.data(using: .utf8)!
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
    
    public func address(_ args: ArgsAddress) async -> String {
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
