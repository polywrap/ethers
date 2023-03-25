import metamask_ios_sdk
import Foundation
import Combine
import RLPSwift

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

public struct ArgsRequest {
    var method: String;
    var params: Data;
    var connection: Any?;
    
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
        
        if (args.method == "eth_sendTransaction") {
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
                let response = (value as! String).data(using: .utf8)!
                completion(.success(response))
            }).store(in: &cancellables)
        }
    }
    
    public func request(_ args: ArgsRequest) async throws -> Data {
        return try await withCheckedThrowingContinuation { continuation in
            request(args: args) { result in
                switch result {
                case .success(let value):
                    continuation.resume(returning: value)
                case .failure(let error):
                    continuation.resume(throwing: error)
                }
            }
        }
    }

    public func waitForTransaction() {
        
    }
    
    public func signTransaction(_ args: ArgsSignTransaction) async throws -> String {
//        throw Error("")
        return ""
    }

    public func signMessage(_ args: ArgsSignMessage) async throws -> String {
        let messageData = Data(args.message)
        let messageHex = "0x" + messageData.hexString
        let address = self.provider.selectedAddress.lowercased()

        let jsonParams = try JSONSerialization.data(withJSONObject: [messageHex, address], options: [])
        let request = ArgsRequest(
            method: "personal_sign",
            params: jsonParams
        )
        
        let response = try await self.request(request)
        return String(data: response, encoding: .utf8)!
    }
    
    public func address() -> String {
        self.provider.selectedAddress
    }

    public func chainId() -> String {
        self.provider.chainId
    }
}

extension Data {
    var hexString: String {
        return map { String(format: "%02x", $0) }.joined()
    }
}
