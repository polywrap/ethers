import {
  Module,
  manifest,
  CoreClient,
  Args_request,
  Args_signMessage,
  Args_signTransaction,
  Args_waitForTransaction,
  Connection as SchemaConnection,
  Args_signerAddress,
} from "./wrap";
import { Connection, SignerType } from "./Connection";
import { Connections } from "./Connections";
import {
  eth_encodePacked,
  eth_sendTransaction,
  eth_signTypedData
} from "./rpc";

import { PluginFactory, PluginPackage } from "@polywrap/plugin-js";
import { ethers } from "ethers";

export * from "./Connection";
export * from "./Connections";

export interface ProviderConfig {
  connections: Connections;
}

export class EthereumProviderPlugin extends Module<ProviderConfig> {
  private _connections: Connections;

  constructor(config: ProviderConfig) {
    super(config);
    this._connections = config.connections;
  }

  public async request(
    args: Args_request,
    _client: CoreClient
  ): Promise<string> {
    const connection = await this._getConnection(args.connection);
    const paramsStr = args?.params ?? "[]";
    const provider = connection.getProvider();

    // Optimizations, utilizing the cache within ethers
    if (args.method === "eth_chainId") {
      const network = await provider.getNetwork();
      return JSON.stringify("0x" + network.chainId.toString(16));
    }

    if (
      args.method === "eth_sendTransaction" &&
      connection.getSignerType() == SignerType.CUSTOM_SIGNER
    ) {
      const signer = await connection.getSigner();
      const parameters = eth_sendTransaction.deserializeParameters(
        paramsStr
      );
      const request = eth_sendTransaction.toEthers(
        parameters[0]
      );
      const res = await signer.sendTransaction(request);
      return JSON.stringify(res.hash);
    }

    if (
      args.method === "eth_signTypedData_v4" &&
      connection.getSignerType() == SignerType.CUSTOM_SIGNER
    ) {
      const signer = await connection.getSigner();
      const parameters = eth_signTypedData.deserializeParameters(
        paramsStr
      );
      let signature = "";
      // This is a hack because in ethers v5.7 this method is experimental
      // when we update to ethers v6 this won't be needed. More info:
      // https://github.com/ethers-io/ethers.js/blob/ec1b9583039a14a0e0fa15d0a2a6082a2f41cf5b/packages/abstract-signer/src.ts/index.ts#L53
      if ("_signTypedData" in signer) {
        const [_, data] = parameters
        const payload = eth_signTypedData.toEthers(data)
        // @ts-ignore
        signature = await signer._signTypedData(
          payload.domain,
          payload.types,
          payload.message
        )
      }
      return JSON.stringify(signature)
    }

    if (args.method === "eth_encodePacked") {
      const params = eth_encodePacked.deserializeParameters(paramsStr);
      const result = ethers.utils.solidityPack(params.types, params.values);
      return JSON.stringify(result);
    }

    const params = JSON.parse(paramsStr);
    try {
      const req = await provider.send(
        args.method,
        params
      );
      return JSON.stringify(req);
    } catch (err) {
      /**
       * Hotfix:
       * Ethers-rs defines the type of EIP 1559 tx
       * as 0x02, but metamask expects it as 0x2,
       * hence, the need of this workaround. Related:
       * https://github.com/MetaMask/metamask-extension/issues/18076
       *
       * We check if the parameters comes as array, if the error
       * contains 0x2 and if the type is 0x02, then we change it
       */
      const paramsIsArray = Array.isArray(params) && params.length > 0;
      const messageContains0x2 =
        err && err.message && err.message.indexOf("0x2") > -1;
      if (messageContains0x2 && paramsIsArray && params[0].type === "0x02") {
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

  public async signerAddress(
    args: Args_signerAddress,
    _client: CoreClient
  ): Promise<string | null> {
    try {
      const connection = await this._getConnection(args.connection);
      return await connection.getSigner().getAddress();
    } catch (_error) {
      return null;
    }
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
    return ethers.utils.joinSignature(
      signedTx as { r: string; s: string; v: number | undefined }
    );
  }

  private async _getConnection(
    connection?: SchemaConnection | null
  ): Promise<Connection> {
    return this._connections.getConnection(connection ?? this.env.connection);
  }

  private _parseTransaction(
    rlp: Uint8Array
  ): ethers.providers.TransactionRequest {
    const tx = ethers.utils.parseTransaction(rlp);

    // r, s, v can sometimes be set to 0, but ethers will throw if the keys exist at all
    let request: Record<string, any> = {
      ...tx,
      r: undefined,
      s: undefined,
      v: undefined,
    };

    // remove undefined and null values
    request = Object.keys(request).reduce((prev, curr) => {
      const val = request[curr];
      if (val !== undefined && val !== null) prev[curr] = val;
      return prev;
    }, {} as Record<string, unknown>);

    return request;
  }
}

export const ethereumProviderPlugin: PluginFactory<ProviderConfig> = (
  config: ProviderConfig
) =>
  new PluginPackage<ProviderConfig>(
    new EthereumProviderPlugin(config),
    manifest
  );

export const plugin = ethereumProviderPlugin;
