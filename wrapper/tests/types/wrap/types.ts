// @ts-ignore
import * as Types from "./";

// @ts-ignore
import {
  Client,
  InvokeResult
} from "@polywrap/core-js";

export type UInt = number;
export type UInt8 = number;
export type UInt16 = number;
export type UInt32 = number;
export type Int = number;
export type Int8 = number;
export type Int16 = number;
export type Int32 = number;
export type Bytes = Uint8Array;
export type BigInt = string;
export type BigNumber = string;
export type Json = string;
export type String = string;
export type Boolean = boolean;

export interface TxRequest {
  to?: Types.String | null;
  from?: Types.String | null;
  data?: Types.String | null;
  type?: Types.UInt32 | null;
  chainId?: Types.BigInt | null;
  accessList?: Array<Types.AccessItem> | null;
  gasLimit?: Types.BigInt | null;
  maxFeePerGas?: Types.BigInt | null;
  maxPriorityFeePerGas?: Types.BigInt | null;
  gasPrice?: Types.BigInt | null;
  value?: Types.BigInt | null;
  nonce?: Types.UInt32 | null;
}

export interface AccessItem {
  address: Types.String;
  storageKeys: Array<Types.String>;
}

export interface TxResponse {
  hash: Types.String;
  to?: Types.String | null;
  from: Types.String;
  nonce: Types.UInt32;
  gasLimit: Types.BigInt;
  maxFeePerGas?: Types.BigInt | null;
  maxPriorityFeePerGas?: Types.BigInt | null;
  gasPrice?: Types.BigInt | null;
  value: Types.BigInt;
  chainId: Types.BigInt;
  blockNumber?: Types.BigInt | null;
  blockHash?: Types.String | null;
  timestamp?: Types.UInt32 | null;
  r?: Types.String | null;
  s?: Types.String | null;
  v?: Types.UInt32 | null;
  type?: Types.UInt32 | null;
  accessList?: Array<Types.AccessItem> | null;
}

export interface Log {
  blockNumber: Types.BigInt;
  blockHash: Types.String;
  transactionIndex: Types.UInt32;
  removed: Types.Boolean;
  address: Types.String;
  data: Types.String;
  topics: Array<Types.String>;
  transactionHash: Types.String;
  logIndex: Types.UInt32;
}

export interface TxReceipt {
  to: Types.String;
  from: Types.String;
  contractAddress: Types.String;
  transactionIndex: Types.UInt32;
  root?: Types.String | null;
  gasUsed: Types.BigInt;
  logsBloom: Types.String;
  transactionHash: Types.String;
  logs: Array<Types.Log>;
  blockNumber: Types.BigInt;
  blockHash: Types.String;
  confirmations: Types.UInt32;
  cumulativeGasUsed: Types.BigInt;
  effectiveGasPrice: Types.BigInt;
  type: Types.UInt32;
  status?: Types.UInt32 | null;
}

export interface TxOptions {
  gasLimit?: Types.BigInt | null;
  maxFeePerGas?: Types.BigInt | null;
  maxPriorityFeePerGas?: Types.BigInt | null;
  gasPrice?: Types.BigInt | null;
  value?: Types.BigInt | null;
  nonce?: Types.UInt32 | null;
}

export interface StaticTxResult {
  result: Types.String;
  error: Types.Boolean;
}

export interface Eip1559FeesEstimate {
  maxFeePerGas: Types.BigInt;
  maxPriorityFeePerGas: Types.BigInt;
}

/// Imported Objects START ///

/* URI: "wrap://ens/interface.ethereum-provider.polywrap.eth" */
export interface IProvider_Connection {
  node?: Types.String | null;
  networkNameOrChainId?: Types.String | null;
}

/// Imported Objects END ///

/// Imported Modules START ///

/* URI: "wrap://ens/interface.ethereum-provider.polywrap.eth" */
interface IProvider_Module_Args_request {
  method: Types.String;
  params?: Types.String | null;
  connection?: Types.IProvider_Connection | null;
}

/* URI: "wrap://ens/interface.ethereum-provider.polywrap.eth" */
interface IProvider_Module_Args_signMessage {
  message: Types.Bytes;
  connection?: Types.IProvider_Connection | null;
}

/* URI: "wrap://ens/interface.ethereum-provider.polywrap.eth" */
interface IProvider_Module_Args_signTransaction {
  rlp: Types.Bytes;
  connection?: Types.IProvider_Connection | null;
}

/* URI: "wrap://ens/interface.ethereum-provider.polywrap.eth" */
interface IProvider_Module_Args_address {
  connection?: Types.IProvider_Connection | null;
}

/* URI: "wrap://ens/interface.ethereum-provider.polywrap.eth" */
interface IProvider_Module_Args_chainId {
  connection?: Types.IProvider_Connection | null;
}

/* URI: "wrap://ens/interface.ethereum-provider.polywrap.eth" */
export const IProvider_Module = {
  request: async (
    args: IProvider_Module_Args_request,
    client: Client,
    uri: string = "wrap://ens/interface.ethereum-provider.polywrap.eth"
  ): Promise<InvokeResult<Types.String>> => {
    return client.invoke<Types.String>({
      uri,
      method: "request",
      args: args as unknown as Record<string, unknown>
    });
  },

  signMessage: async (
    args: IProvider_Module_Args_signMessage,
    client: Client,
    uri: string = "wrap://ens/interface.ethereum-provider.polywrap.eth"
  ): Promise<InvokeResult<Types.String>> => {
    return client.invoke<Types.String>({
      uri,
      method: "signMessage",
      args: args as unknown as Record<string, unknown>
    });
  },

  signTransaction: async (
    args: IProvider_Module_Args_signTransaction,
    client: Client,
    uri: string = "wrap://ens/interface.ethereum-provider.polywrap.eth"
  ): Promise<InvokeResult<Types.String>> => {
    return client.invoke<Types.String>({
      uri,
      method: "signTransaction",
      args: args as unknown as Record<string, unknown>
    });
  },

  address: async (
    args: IProvider_Module_Args_address,
    client: Client,
    uri: string = "wrap://ens/interface.ethereum-provider.polywrap.eth"
  ): Promise<InvokeResult<Types.String>> => {
    return client.invoke<Types.String>({
      uri,
      method: "address",
      args: args as unknown as Record<string, unknown>
    });
  },

  chainId: async (
    args: IProvider_Module_Args_chainId,
    client: Client,
    uri: string = "wrap://ens/interface.ethereum-provider.polywrap.eth"
  ): Promise<InvokeResult<Types.String>> => {
    return client.invoke<Types.String>({
      uri,
      method: "chainId",
      args: args as unknown as Record<string, unknown>
    });
  }
}

/// Imported Modules END ///
