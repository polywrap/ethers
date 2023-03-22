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
//    var provider: Ethereum
    var cancellables: Set<AnyCancellable> = []
    
    var result: String = ""
    public init(provider:Ethereum, dapp:Dapp) {
        provider.connect(dapp)?.sink(receiveCompletion: { completion in
            switch completion {
            case let .failure(error):
                print("Error connecting to address: \(error)")
            default: break
            }
        }, receiveValue: { value in
            print("Wallet connected! \(value)")
        }).store(in: &cancellables)
    }

    public func request(provider:Ethereum, args: ArgsRequest) -> String {
//        if (!provider.connected) {
//
//        }

        let request = EthereumRequest(method: args.method, params: args.params)
        provider.request(request)?.sink(receiveCompletion: { completion in
            switch completion {
            case let .failure(error):
                print("Add chain error: \(error.localizedDescription)")
            default: break
            }
        }, receiveValue: { value in
            print("Add chain result: \(value)")
        }).store(in: &cancellables)
        
        return self.result
    };

    public func waitForTransaction() {
        
    }
    
    public func signTransaction() {
        
    }
    
    public func signMessage() {
        
    }
    
    public func address() {
        
    }
    
    public func chainId() {
        
    }
}
