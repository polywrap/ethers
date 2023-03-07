import {
  Module,
  manifest,
  CoreClient,
  IProvider_Module_Args_request as Args_request,
  IProvider_Module_Args_signMessage as Args_signMessage,
  IProvider_Module_Args_signTransaction as Args_signTransaction,
  IProvider_Module_Args_address as Args_address,
  IProvider_Module_Args_chainId as Args_chainId,
  IProvider_Module_Args_waitForTransaction as Args_waitForTransaction,
  IProvider_Connection as SchemaConnection
} from "./wrap";
import { PluginFactory, PluginPackage } from "@polywrap/plugin-js";
import { Connection } from "./Connection";
import { Connections } from "./Connections";
import { ethers } from "ethers";
export * from "./Connection";
export * from "./Connections";

export interface ProviderConfig {
  connections: Connections;
}

export class EthereumProviderPlugin extends Module<ProviderConfig> {
  private _connections: Connections;

  constructor(config: ProviderConfig) {
    super(config)
    this._connections = config.connections;
  }

  public async request(
    args: Args_request,
    _client: CoreClient
  ): Promise<string> {
    const connection = await this._getConnection(args.connection);
    const params = JSON.parse(args?.params ?? "[]");
    const provider = connection.getProvider();

    try {
      const req = await provider.send(args.method, params);
      return JSON.stringify(req);
    } catch (err) {
      // Ethers-rs defines the type of EIP 1559 tx
      // as 0x02, but metamask expects it as 0x2,
      // hence, the need of this workaround. Related:
      // https://github.com/foundry-rs/foundry/issues/3890
      if (
        err && err.message &&
        err.message.indexOf("0x2") > -1 &&
        params[0].type === "0x02" &&
        args.method === "eth_sendTransaction"
      ) {
        params[0].type = "0x2";
        const req = await provider.send(args.method, params);
        return JSON.stringify(req);
      } else {
        throw err;
      }
    }
  }

  async waitForTransaction(
    args: Args_waitForTransaction,
    _client: CoreClient
  ): Promise<boolean> {
    const connection = await this._getConnection(args.connection);
    const provider = connection.getProvider();

    await provider.waitForTransaction(
      args.txHash,
      args.confirmations,
      args.timeout ?? undefined
    );

    return true;
  }

  public async signMessage(
    args: Args_signMessage,
    _client: CoreClient
  ): Promise<string> {
    const connection = await this._getConnection(args.connection);
    return await connection.getSigner().signMessage(args.message);
  }

  public async signTransaction(
    args: Args_signTransaction,
    _client: CoreClient
  ): Promise<string> {
    const connection = await this._getConnection(args.connection);
    const request = this._parseTransaction(args.rlp);
    const signedTxHex = await connection.getSigner().signTransaction(request);
    const signedTx = ethers.utils.parseTransaction(signedTxHex);
    return ethers.utils.joinSignature(signedTx as { r: string; s: string; v: number | undefined });
  }

  public async address(
    args: Args_address,
    _client: CoreClient
  ): Promise<string> {
    const connection = await this._getConnection(args.connection);
    return await connection.getSigner().getAddress();
  }

  public async chainId(
    args: Args_chainId,
    _client: CoreClient
  ): Promise<string> {
    const connection = await this._getConnection(args.connection);
    const network = await connection.getProvider().getNetwork();
    return network.chainId.toString();
  }

  private async _getConnection(connection?: SchemaConnection | null): Promise<Connection> {
    return this._connections.getConnection(connection ?? this.env.connection);
  }

  private _parseTransaction(rlp: Uint8Array): ethers.providers.TransactionRequest {
    const tx = ethers.utils.parseTransaction(rlp);

    // r, s, v can sometimes be set to 0, but ethers will throw if the keys exist at all
    let request: Record<string, any> = { ...tx, r: undefined, s: undefined, v: undefined };

    // remove undefined and null values
    request = Object.keys(request).reduce((prev, curr) => {
      const val = request[curr];
      if (val !== undefined && val !== null) prev[curr] = val
      return prev;
    }, {} as Record<string, unknown>)

    return request;
  }
}

export const ethereumProviderPlugin: PluginFactory<ProviderConfig> = (
  config: ProviderConfig
) => new PluginPackage<ProviderConfig>(new EthereumProviderPlugin(config), manifest);

export const plugin = ethereumProviderPlugin;

