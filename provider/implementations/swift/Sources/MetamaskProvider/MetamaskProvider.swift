import metamask_ios_sdk
import Foundation
import Combine

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
    var message: [UInt8]
    var connection: Any?;
    
    public init(message: [UInt8]) {
        self.message = message
        self.connection = nil
    }
}

enum ProviderError: Error {
    case NotConnected
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
    
    func request(args: ArgsRequest, completion: @escaping (Result<String, Error>) -> Void) {
        if !provider.connected {
            completion(.failure(ProviderError.NotConnected))
            return
        }
        
        let params = try! JSONDecoder().decode([String].self, from: args.params)
        let request = EthereumRequest(method: args.method, params: params)

        provider.request(request)?.sink(receiveCompletion: { completionResult in
            switch completionResult {
            case .finished:
                break
            case let .failure(error):
                completion(.failure(error))
            }
        }, receiveValue: { value in
            completion(.success(value as! String))
        }).store(in: &cancellables)
    }
    
    public func request(_ args: ArgsRequest) async throws -> String {
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
    
    public func signTransaction() {

    }

    public func signMessage(_ message: String) async throws -> String {
        let messageData: Data = Data(message.utf8)
        let messageHex = "0x" + messageData.hexString
        let address = self.provider.selectedAddress.lowercased()

        let jsonParams = try JSONSerialization.data(withJSONObject: [messageHex, address], options: [])
        let request = ArgsRequest(
            method: "personal_sign",
            params: jsonParams
        )

        return try await self.request(request)
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
