import SocketIO
import metamask_ios_sdk
import Foundation
import Combine
import PolywrapClient

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
    let connection: [String: String?]?
    
    public init(txHash: String, confirmations: UInt32, timeout: UInt32? = nil, connection: [String: String]?) {
        self.txHash = txHash
        self.confirmations = confirmations
        self.timeout = timeout
        self.connection = connection
    }
}

public struct ArgsRequest: Codable {
    var method: String;
    var params: String?;
    var connection: [String: String?]?;
    
    public init(method: String, params: String? = "") {
        self.method = method
        self.params = params
        self.connection = nil
    }
}

public struct ArgsSignMessage: Codable {
    public init() {}
}

public struct ArgsSignTransaction: Codable {
    public init() {}
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

struct Domain: Codable {
    var chainId: Int
    var verifyingContract: String
}

struct Message: Codable {
    var baseGas: String
    var data: String
    var gasPrice: Int
    var gasToken: String
    var nonce: Int
    var operation: Int
    var refundReceiver: String
    var safeTxGas: Int
    var to: String
    var value: String
}

struct TypedData: Codable {
    var domain: Domain
    var message: Dictionary<String, StringOrInt>
    var primaryType: String
    var types: Dictionary<String, [Dictionary<String, String>]>
}

enum StringOrInt: Codable {
    case string(String)
    case int(Int)
    
    init(from decoder: Decoder) throws {
        let container = try decoder.singleValueContainer()
        
        if let typedDataValue = try? container.decode(Int.self) {
            self = .int(typedDataValue)
        } else if let stringValue = try? container.decode(String.self) {
            self = .string(stringValue)
        } else {
            throw DecodingError.dataCorruptedError(
                in: container,
                debugDescription: "Unable to decode StringOrInt"
            )
        }
    }

    func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()

        switch self {
        case .string(let stringValue):
            try container.encode(stringValue)
        case .int(let int):
            try container.encode(int)
        }
    }
    
}

enum AddressOrTypedData: Codable {
    case string(String)
    case typedData(TypedData)
    
    init(from decoder: Decoder) throws {
        let container = try decoder.singleValueContainer()
        
        if let typedDataValue = try? container.decode(TypedData.self) {
            self = .typedData(typedDataValue)
        } else if let stringValue = try? container.decode(String.self) {
            self = .string(stringValue)
        } else {
            throw DecodingError.dataCorruptedError(
                in: container,
                debugDescription: "Unable to decode AddressOrTypedData"
            )
        }
    }
    
    func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()

        switch self {
        case .string(let stringValue):
            try container.encode(stringValue)
        case .typedData(let typedDatavalue):
            try container.encode(typedDatavalue)
        }
    }
    
    func toString() -> String {
        switch self {
        case .typedData(let v):
            let encoder = JSONEncoder()
            let jsonData = try! encoder.encode(v)
            let string = String(data: jsonData, encoding: .utf8)!
            return string
        case.string(let v):
            return v
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
                debugDescription: "Unable to decode TxOrString"
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
