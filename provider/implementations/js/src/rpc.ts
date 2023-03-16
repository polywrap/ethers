import ethers from "ethers";

// Ref: https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_sendtransaction
export namespace eth_sendTransaction {
  export interface Transaction {
    // DATA, 20 Bytes - The address the transaction is sent from.
    from: string;
    // DATA, 20 Bytes - (optional when creating new contract) The address the transaction is directed to.
    to?: string;
    // QUANTITY - (optional, default: 90000) Integer of the gas provided for the transaction execution. It will return unused gas.
    gas?: string;
    // QUANTITY - (optional, default: To-Be-Determined) Integer of the gasPrice used for each paid gas.
    gasPrice?: string;
    // QUANTITY - (optional) Integer of the value sent with this transaction.
    value?: string;
    // DATA - The compiled code of a contract OR the hash of the invoked method signature and encoded parameters.
    data: string;
    // QUANTITY - (optional) Integer of a nonce. This allows to overwrite your own pending transactions that use the same nonce.
    nonce?: string;
  }

  export type Parameters = [Transaction];

  // DATA, 32 Bytes - the transaction hash, or the zero hash if the transaction is not yet available.
  export type Returns = string;

  export function deserializeParameters(input: string): Parameters {
    const params = JSON.parse(input);
    if (params.length < 1 || typeof params[0] !== "object") {
      throw new Error(
        "Invalid JSON-RPC parameters provided for eth_sendTransaction method. Reference: " +
        "https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_sendtransaction"
      );
    }

    const transaction: Transaction = params[0];

    if (!transaction.from) {
      throw new Error(
        "The 'from' property on the transaction object parameter is required for the eth_sendTransaction method. Reference: " +
        "https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_sendtransaction"
      );
    }

    if (!transaction.data) {
      throw new Error(
        "The 'data' property on the transaction object parameter is required for the eth_sendTransaction method. Reference: " +
        "https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_sendtransaction"
      );
    }

    return [transaction];
  }

  export function toEthers(
    transaction: Transaction
  ): ethers.providers.TransactionRequest {
    const result: ethers.providers.TransactionRequest = {
      ...transaction,
      // Ethers.js expects `gasLimit` instead of `gas`
      gasLimit: transaction.gas
    };

    delete (result as unknown as Record<string, unknown>).gas

    // Ethers.js expects "0" | "1" | "2"
    // but it's being received as hex (e.g: "0x02")
    if ("type" in transaction) {
      result.type = parseInt(
        (transaction as unknown as Record<string, string>).type
      );
    }

    return result;
  }
}

// Ref: https://github.com/ethereum/EIPs/blob/master/EIPS/eip-712.md
export namespace eth_signTypedData {
  export interface TypedData {
    types: {
      EIP712Domain: unknown[];
      [key: string]: {
        name: string;
        type: string;
        [key: string]: unknown;
      }[] | unknown;
    };
    primaryType: string;
    domain: { [key: string]: unknown };
    message: { [key: string]: unknown };
    [key: string]: unknown;
  }

  export type Parameters = [
    // Address - 20 Bytes - Address of the account that will sign the messages.
    string,
    // TypedData - Typed structured data to be signed.
    TypedData
  ];

  // DATA, 129 Bytes - the signature, as described here:
  // https://github.com/ethereum/EIPs/blob/master/EIPS/eip-712.md#returns
  export type Returns = string;

  export function deserializeParameters(input: string): Parameters {
    const params = JSON.parse(input);
    if (
      params.length < 2 ||
      typeof params[0] !== "string" ||
      typeof params[1] !== "object"
    ) {
      throw new Error(
        "Invalid JSON-RPC parameters provided for eth_signTypedData method. Reference: " +
        "https://github.com/ethereum/EIPs/blob/master/EIPS/eip-712.md#parameters"
      );
    }

    return params;
  }

  export type EthersTypedData = {
    domain: TypedData["domain"]
    types: {
      [key: string]: unknown;
    }
    message: TypedData["message"]
  };

  export function toEthers(
    typedData: TypedData
  ): EthersTypedData {
    let types: Omit<TypedData["types"], "EIP712Domain"> = typedData.types;
    delete types.EIP712Domain;
    return {
      domain: typedData.domain,
      types,
      message: typedData.message
    }
  }
}

export namespace eth_encodePacked {
  export function deserializeParameters(paramsStr: string): { types: string[], values: unknown[] } {
    const params = JSON.parse(paramsStr);
    if (
      "types" in params && "values" in params
      && Array.isArray(params.types) && Array.isArray(params.values)
      && typeof params.types[0] === "string"
    ) {
      return {
        types: params.types,
        values: parseValues(params.values)
      };
    }

    throw new Error(
      "Invalid JSON-RPC parameters provided for eth_encodePacked method. " +
      "Expected JSON of the form: { types: string[], values: string[] }"
    );
  }

  function parseValues(values?: string[] | null): unknown[] {
    if (!values) {
      return [];
    }

    return values.map((arg: string) =>
      (arg.startsWith("[") && arg.endsWith("]")) ||
      (arg.startsWith("{") && arg.endsWith("}"))
        ? JSON.parse(arg)
        : arg
    );
  }
}