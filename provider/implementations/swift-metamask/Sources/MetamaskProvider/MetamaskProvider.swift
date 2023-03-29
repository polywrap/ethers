import metamask_ios_sdk
import Foundation
import Combine
//import PolywrapClient

public struct Transaction: CodableData {
    let to: String
    let from: String
    let value: String
    let data: String?

    public init(to: String, from: String, value: String, data: String? = nil) {
        self.to = to
        self.from = from
        self.value = value
        self.data = data
    }
    
    public init?(json: [String: Any]) {
        guard let to = json["to"] as? String,
              let from = json["from"] as? String,
              let value = json["value"] as? String
        else {
            return nil
        }
        
        
        self.to = to
        self.from = from
        self.value = value
        
        if let data = json["data"] as? String {
            self.data = data
        } else {
            self.data = nil
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
    let connection: String? = nil
    
    public init(txHash: String, confirmations: UInt32, timeout: UInt32? = nil) {
        self.txHash = txHash
        self.confirmations = confirmations
        self.timeout = timeout
    }
}

public struct ArgsRequest: Codable {
    var method: String;
    var params: Data;
    var connection: String?;
    
    public init(method: String, params: Data) {
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

enum ProviderError: Error {
    case NotConnected
    case EncodeError
    case MethodNotSupported
}

public class MetamaskProvider {
    var provider: Ethereum
    var cancellables: Set<AnyCancellable> = []
    var result: Data? = nil;

    public init(provider:Ethereum, dapp:Dapp) {
        self.provider = provider
        self.provider.connect(dapp)?.sink(receiveCompletion: { completion in
            switch completion {
            case let .failure(error):
                print("Error connecting to address: \(error)")
            default: break
            }
        }, receiveValue: { value in
            print("Wallet connected! \(value)")
        }).store(in: &cancellables)
    }
        
    func request(args: ArgsRequest, completion: @escaping (Result<Data, Error>) -> Void) {
        if !provider.connected {
            completion(.failure(ProviderError.NotConnected))
            return
        }
        
        if self.isTransactionMethod(args.method) {
            let params: [Transaction] = try! JSONDecoder().decode(
                [Transaction].self,
                from: args.params
            )

            let request = EthereumRequest(method: args.method, params: params)
            provider.request(request)?.sink(receiveCompletion: { completionResult in
                switch completionResult {
                case .finished:
                    break
                case let .failure(error):
                    completion(.failure(error))
                }
            }, receiveValue: { value in
                let response = (value as! String).data(using: .utf8)!
                completion(.success(response))
            }).store(in: &cancellables)

        } else {
            let params: [String] = try! JSONDecoder().decode([String].self, from: args.params)
            let request = EthereumRequest(method: args.method, params: params)

            provider.request(request)?.sink(receiveCompletion: { completionResult in
                switch completionResult {
                case .finished:
                    break
                case let .failure(error):
                    completion(.failure(error))
                }
            }, receiveValue: { value in
                if let stringValue = value as? String {
                    let response = stringValue.data(using: .utf8)!
                    completion(.success(response))
                }
                
                if let jsonValue = value as? [String: Any] {
                    do {
                        let jsonData = try JSONSerialization.data(withJSONObject: jsonValue, options: [])
                        completion(.success(jsonData))
                    } catch {
                        completion(.failure(ProviderError.EncodeError))
                    }
                } else {
                    completion(.success(Data()))
                }
            }).store(in: &cancellables)
        }
    }
    
    public func request(_ args: ArgsRequest) async -> Data {
        return await withCheckedContinuation { continuation in
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
    public func waitForTransaction(_ args: ArgsWaitForTransaction) async -> Bool {
        let startTime = Date()
        while true {
            do {
                let jsonParams = try JSONSerialization.data(withJSONObject: [args.txHash], options: [])
                let request = ArgsRequest(
                    method: "eth_getTransactionReceipt",
                    params: jsonParams
                )
                let receipt = await self.request(request)
                if receipt != Data() {
                    if  try JSONSerialization.jsonObject(with: receipt, options: []) is [String: Any] {
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
    
    public func address() -> String {
        self.provider.selectedAddress
    }

    public func chainId() -> String {
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
