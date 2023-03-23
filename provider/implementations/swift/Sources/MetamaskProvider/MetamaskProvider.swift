import metamask_ios_sdk
import Foundation
import Combine


public struct ArgsRequest {
    var method: String;
    var params: Data;
    
    public init(method: String, params: Data) {
        self.method = method
        self.params = params
    }
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
//            completion(.failure(SomeError.notConnected)) // Replace with an appropriate error type
            return
        }

        let request = EthereumRequest(method: args.method, params: args.params)

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
    
    public func request(args: ArgsRequest) async throws -> String {
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
    
    public func signMessage() {
        
    }
    
    public func address() {
        
    }
    
    public func chainId() {
//        self.provider.$chainId
    }
}
