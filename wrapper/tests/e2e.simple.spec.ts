import { PolywrapClient } from "@polywrap/client-js";
import { ensResolverPlugin } from "@polywrap/ens-resolver-plugin-js";
import {
  buildWrapper,
  ensAddresses,
  providers,
} from "@polywrap/test-env-js";
import * as path from 'path'

import { ethers, Wallet } from "ethers";
import { keccak256 } from "js-sha3";
import { Connection, Connections, ethereumProviderPlugin } from "ethereum-provider-js";
import * as Schema from "./types/wrap";
import { initInfra, stopInfra } from "./utils/infra";

const { hash: namehash } = require("eth-ens-namehash");
const contracts = {
  StructArg: {
    abi: require("./contracts/StructArg.ABI.json"),
    bytecode: `0x${require("./contracts/StructArg.Bytecode.json").object}`,
  },
  SimpleStorage: {
    abi: require("./contracts/easy/SimpleStorage.ABI.json"),
    bytecode: `0x${require("./contracts/easy/SimpleStorage.Bytecode.json").object}`,
  },
};

jest.setTimeout(360000);

describe("Ethereum Wrapper", () => {
  let client: PolywrapClient;
  let ensAddress: string;
  let registrarAddress: string;
  const signer = "0x90F8bf6A479f320ead074411a4B0e7944Ea8c9C1";

  const dirname: string = path.resolve(__dirname);
  const wrapperPath: string = path.join(dirname, "..");
  const uri = `fs/${wrapperPath}/build`;

  beforeAll(async () => {
    await initInfra();

    ensAddress = ensAddresses.ensAddress;
    registrarAddress = ensAddresses.registrarAddress;

    const interfacePath = path.join(__dirname, "..", "..", "provider", "interface");
    await buildWrapper(interfacePath);
    const interfaceFsUri = `wrap://fs/${path.resolve(interfacePath)}/build`;

    client = new PolywrapClient({
      redirects: [{
        from: "wrap://ens/interface.ethereum-provider.polywrap.eth",
        to: interfaceFsUri,
      }],
      plugins: [
        {
          uri: "wrap://ens/ens-resolver.polywrap.eth",
          plugin: ensResolverPlugin({
            addresses: {
              testnet: ensAddress,
            },
          }),
        },
        {
          uri: "wrap://ens/js.ethereum-provider.polywrap.eth",
          plugin: ethereumProviderPlugin({
            connections: new Connections({
              networks: {
                testnet: new Connection({
                  provider: providers.ethereum,
                  signer: new Wallet(
                    "0x4f3edf983ac636a65a842ce7c78d9aa706d3b113bce9c46f30d7d21715b23b1d"
                  ),
                })
              },
              defaultNetwork: "testnet",
            })
          }),
        },
      ],
      interfaces: [
        {
          interface: "wrap://ens/interface.ethereum-provider.polywrap.eth",
          implementations: ["wrap://ens/js.ethereum-provider.polywrap.eth"]
        }
      ]
    });
  });

  afterAll(async () => {
    await stopInfra();
  });

  describe("Ethereum Wrapper", () => {
    it("chainId", async () => {
      const response = await client.invoke<string>({
        uri,
        method: "getChainId",
        args: {},
      });

      if (!response.ok) throw response.error;
      expect(response.value).toBeDefined();
      expect(response.value).toBe("1337");
    });

    it("getBalance", async () => {
      const response = await client.invoke<string>({
        uri,
        method: "getBalance",
        args: {
          address: signer,
        },
      });

      if (!response.ok) throw response.error;
      expect(response.value).toBeDefined();
    });

    it("checkAddress", async () => {
      const response = await client.invoke<string>({
        uri,
        method: "checkAddress",
        args: {
          address: signer,
        },
      });

      if (!response.ok) throw response.error;
      expect(response.value).toBeDefined();
      expect(response.value).toEqual(true);
    });

    it("getGasPrice", async () => {
      const response = await client.invoke<string>({
        uri,
        method: "getGasPrice"
      });

      if (!response.ok) throw response.error;
      expect(response.value).toBeDefined();
    });

    it("estimateEip1559Fees", async () => {
      const response = await client.invoke<Schema.Eip1559FeesEstimate>({
        uri,
        method: "estimateEip1559Fees"
      });

      if (!response.ok) throw response.error;
      expect(response.value).toBeDefined();
      expect(response.value.maxFeePerGas).toBeDefined();
      expect(response.value.maxPriorityFeePerGas).toBeDefined();
    });

    it("signMessage", async () => {
      const response = await client.invoke<string>({
        uri,
        method: "signMessage",
        args: {
          message: "Hello World"
        }
      });

      if (!response.ok) throw response.error;
      expect(response.value).toBe(
        "0xa4708243bf782c6769ed04d83e7192dbcf4fc131aa54fde9d889d8633ae39dab03d7babd2392982dff6bc20177f7d887e27e50848c851320ee89c6c63d18ca761c"
      );
    });

    it("getSignerAddress", async () => {
      const response = await client.invoke<string>({
        uri,
        method: "getSignerAddress",
      });

      if (!response.ok) throw response.error;
      expect(response.value).toBeDefined();
      expect(response.value?.startsWith("0x")).toBe(true);
    });

    it("getSignerBalance", async () => {
      const response = await client.invoke<string>({
        uri,
        method: "getSignerBalance",
      });

      if (!response.ok) throw response.error;
      expect(response.value).toBeDefined();
    });

    it("getSignerTransactionCount", async () => {
      const response = await client.invoke<string>({
        uri,
        method: "getSignerTransactionCount",
      });

      if (!response.ok) throw response.error;
      expect(response.value).toBeDefined();
      expect(Number(response.value)).toBeTruthy();
    });

    it("getGasPrice", async () => {
      const response = await client.invoke<string>({
        uri,
        method: "getGasPrice",
      });

      if (!response.ok) throw response.error;
      expect(response.value).toBeDefined();
      expect(Number(response.value)).toBeTruthy();
    });

    it("encodeParams", async () => {
      const response = await client.invoke<string>({
        uri,
        method: "encodeParams",
        args: {
          types: ["uint256", "uint256", "address"],
          values: ["8", "16", "0x0000000000000000000000000000000000000000"],
        },
      });

      if (!response.ok) throw response.error;
      expect(response.value).toBe(
        "0x000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000000"
      );
    });

    it("encodeParams - (uint256, uint256)", async () => {
      const response = await client.invoke<string>({
        uri,
        method: "encodeParams",
        args: {
          types: ["(uint256, uint256)"],
          values: ["(8,16)"],
        },
      });

      if (!response.ok) throw response.error;
      expect(response.value).toBe(
        "0x00000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000010"
      );

    });

    it("encodeParams - (uint256, uint256, address)", async () => {
      const response = await client.invoke<string>({
        uri,
        method: "encodeParams",
        args: {
          types: ["(uint256, uint256, address)"],
          values: ["(8,16,0x0000000000000000000000000000000000000000)"],
        },
      });

      if (!response.ok) throw response.error;
      expect(response.value).toBe(
        "0x000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000000"
      );
    });

    it("encodeFunction", async () => {
      const response = await client.invoke<string>({
        uri,
        method: "encodeFunction",
        args: {
          method: "function increaseCount(uint256)",
          args: ["100"],
        },
      });

      if (!response.ok) throw response.error;
      expect(response.value).toBe(
        "0x46d4adf20000000000000000000000000000000000000000000000000000000000000064"
      );
    });

    it("encodeFunction - array arg", async () => {
      const response = await client.invoke<string>({
        uri,
        method: "encodeFunction",
        args: {
          method: "function createArr(uint256[] memory)",
          args: [JSON.stringify([1, 2])],
        },
      });

      expect(response.ok).toBeTruthy();
    });

    it("toWei", async () => {
      const response = await client.invoke<string>({
        uri,
        method: "toWei",
        args: {
          eth: "20",
        },
      });

      if (!response.ok) throw response.error;
      expect(response.value).toBeDefined();
      expect(response.value).toEqual("20000000000000000000");
    });

    it("toEth", async () => {
      const response = await client.invoke<string>({
        uri,
        method: "toEth",
        args: {
          wei: "20000000000000000000",
        },
      });

      if (!response.ok) throw response.error;
      expect(response.value).toBeDefined();
      expect(response.value).toEqual("20");
    });

    it("sendRpc", async () => {
      const res = await client.invoke<string | undefined>({
        uri,
        method: "sendRpc",
        args: {
          method: "eth_blockNumber", params: []
        }
      });

      expect(res.ok).toBeTruthy();
      if (!res.ok) throw Error("never");
      expect(res.value).toBeDefined();
    });

    it("estimateTransactionGas", async () => {
      const data = contracts.SimpleStorage.bytecode;

      const response = await client.invoke<string>({
        uri,
        method: "estimateTransactionGas",
        args: {
          tx: {
            data: data,
          },
        },
      });

      if (!response.ok) throw response.error;
      expect(response.value).toBeDefined();
      const num = ethers.BigNumber.from(response.value);
      expect(num.gt(0)).toBeTruthy();
    });

    it("awaitTransaction", async () => {
      const data = contracts.SimpleStorage.bytecode;

      const response = await client.invoke<Schema.TxResponse>({
        uri,
        method: "sendTransaction",
        args: {
          tx: {
            data: data,
          },
        },
      });

      if (!response.ok) throw response.error;
      expect(response.value.hash).toBeTruthy();
      const txHash = response.value.hash as string;

      const awaitResponse = await client.invoke<Schema.TxReceipt>({
        uri,
        method: "awaitTransaction",
        args: {
          txHash: txHash,
          // timeout: 60000,
        },
      });

      if (!awaitResponse.ok) throw awaitResponse.error;
      expect(awaitResponse.value).toBeDefined();
      expect(awaitResponse.value.transactionHash).toBeDefined();
    });

    it("sendTransaction", async () => {
      const response = await client.invoke<Schema.TxResponse>({
        uri,
        method: "sendTransaction",
        args: {
          tx: { data: contracts.SimpleStorage.bytecode }
        }
      });

      if (!response.ok) throw response.error;
      expect(response.value).toBeDefined();
      expect(response.value.hash).toBeDefined();
    });

    it("sendTransactionAndWait", async () => {
      const response = await client.invoke<Schema.TxReceipt>({
        uri,
        method: "sendTransactionAndWait",
        args: {
          tx: { data: contracts.SimpleStorage.bytecode }
        }
      });

      if (!response.ok) throw response.error;
      expect(response.value).toBeDefined();
      expect(
        response.value.transactionHash
      ).toBeDefined();
    });

    it("estimateTransactionGas", async () => {
      const data = contracts.SimpleStorage.bytecode;

      const response = await client.invoke<string>({
        uri,
        method: "estimateTransactionGas",
        args: {
          tx: {
            data: data,
          },
        },
      });

      if (!response.ok) throw response.error;
      expect(response.value).toBeDefined();
      const num = ethers.BigNumber.from(response.value);
      expect(num.gt(0)).toBeTruthy();
    });

    it("deployContract", async () => {
      const response = await client.invoke<string>({
        uri,
        method: "deployContract",
        args: {
          abi: JSON.stringify(contracts.SimpleStorage.abi),
          bytecode: contracts.SimpleStorage.bytecode,
        }
      });

      if (!response.ok) throw response.error;
      expect(response.value).toBeDefined();
      expect(response.value).toContain("0x");
      expect(response.value.length).toBe(42);
    });

    it("estimateContractCallGas", async () => {
      const label = "0x" + keccak256("testwhatever2");
      const response = await client.invoke<string>({
        uri,
        method: "estimateContractCallGas",
        args: {
          address: registrarAddress,
          method: "function register(bytes32 label, address owner)",
          args: [label, signer],
        },
      });

      if (!response.ok) throw response.error;
      expect(response.value).toBeDefined();
      const num = ethers.BigNumber.from(response.value);
      expect(num.gt(0)).toBeTruthy();
    });

    it("callContractView", async () => {
      const node = namehash("whatever.eth");
      const response = await client.invoke<string>({
        uri,
        method: "callContractView",
        args: {
          address: ensAddress,
          method: "function resolver(bytes32 node) external view returns (address)",
          args: [node]
        }
      });

      if (!response.ok) throw response.error;
      expect(response.value).toBeDefined();
      expect(response.value).toBe("0x0000000000000000000000000000000000000000");
    });

    it("callContractStatic (no error)", async () => {
      const label = "0x" + keccak256("testwhatever");
      const response = await client.invoke<Schema.StaticTxResult>({
        uri,
        method: "callContractStatic",
        args: {
          address: registrarAddress,
          method: "function register(bytes32 label, address owner)",
          args: [label, signer],
        },
      });

      if (!response.ok) throw response.error;
      expect(response.value.error).toBeFalsy();
      expect(response.value.result).toBe("");
    });

    it("callContractStatic (expecting error)", async () => {
      const label = "0x" + keccak256("testwhatever");
      const response = await client.invoke<Schema.StaticTxResult>({
        uri,
        method: "callContractStatic",
        args: {
          address: registrarAddress,
          method: "function registerr(bytes32 label, address owner)",
          args: [label, signer],
        },
      });

      if (!response.ok) throw response.error;
      expect(response.value).toBeDefined();
      expect(response.value.error).toBeTruthy();
      expect(response.value.result).toContain(
        "VM Exception while processing transaction: revert"
      );
    });

    it("callContractStatic (expecting error) - TxOptions", async () => {
      const label = "0x" + keccak256("testwhatever");
      const response = await client.invoke<Schema.StaticTxResult>({
        uri,
        method: "callContractStatic",
        args: {
          address: registrarAddress,
          method: "function register(bytes32 label, address owner)",
          args: [label, signer],
          options: {
            maxPriorityFeePerGas: "40000000",
            maxFeePerGas: "400000000",
            gasLimit: "1",
          },
        },
      });

      if (!response.ok) throw response.error;
      expect(response.value?.error).toBeTruthy();
      expect(response.value?.result).toContain("out of gas");
    });

    it("callContractMethod", async () => {
      const label = "0x" + keccak256("testwhatever");
      const response = await client.invoke({
        uri,
        method: "callContractMethod",
        args: {
          address: registrarAddress,
          method: "function register(bytes32 label, address owner)",
          args: [label, signer],
          options: {
            maxPriorityFeePerGas: "40000000",
            maxFeePerGas: "400000000",
            gasLimit: "200000"
          },
        },
      });

      if (!response.ok) throw response.error;
      expect(response.value).toBeDefined();
    });

    it("callContractMethodAndWait", async () => {
      const label = "0x" + keccak256("testwhatever");
      const response = await client.invoke<Schema.TxReceipt>({
        uri,
        method: "callContractMethodAndWait",
        args: {
          address: registrarAddress,
          method: "function register(bytes32 label, address owner)",
          args: [label, signer],
          options: {
            gasPrice: "400000000",
            gasLimit: "200000"
          },
        }
      });

      if (!response.ok) throw response.error;
      expect(response.value).toBeDefined();
    });
  });
});
