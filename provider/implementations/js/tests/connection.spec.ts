import { Connection } from "../src";
import { Commands } from "@polywrap/cli-js";
import { ETH_ENS_IPFS_MODULE_CONSTANTS } from "polywrap";
import { Wallet } from "ethers";

jest.setTimeout(600000);

type BasicNetwork = "mainnet" | "goerli" | "sepolia";

const getRpcUri = (network: BasicNetwork): string => {
  return `https://${network}.infura.io/v3/d119148113c047ca90f0311ed729c466`;
}

describe("Connection", () => {
  const ethProvider = ETH_ENS_IPFS_MODULE_CONSTANTS.ethereumProvider;
  const signerAddress = "0x90F8bf6A479f320ead074411a4B0e7944Ea8c9C1";
  let testnet: Connection;

  beforeAll(async () => {
    await Commands.infra("up", { modules: ["eth-ens-ipfs"] });
    testnet = new Connection({
      provider: ethProvider,
      signer: new Wallet(
        "0x4f3edf983ac636a65a842ce7c78d9aa706d3b113bce9c46f30d7d21715b23b1d"
      ),
    });
  });

  afterAll(async () => {
    await Commands.infra("down", { modules: ["eth-ens-ipfs"] });
  });

  it("Constructs from Networkish", () => {
    const connection = Connection.fromNetwork("mainnet");
    expect(connection).toBeDefined();
    expect(connection.getProvider()).toBeDefined();
  });

  it("Constructs from Node", () => {
    const connection = Connection.fromNode(getRpcUri("mainnet"));
    expect(connection).toBeDefined();
    expect(connection.getProvider()).toBeDefined();
  });

  test("getProvider", () => {
    expect(testnet.getProvider()).toBeDefined();
  });

  test("setProvider", () => {
    const goerliUri = getRpcUri("goerli");
    const connection = new Connection({ provider: goerliUri });
    expect(connection.getProvider().connection.url).toEqual(goerliUri);
    connection.setProvider(ethProvider);
    expect(connection.getProvider().connection.url).toEqual(ethProvider);
    expect(connection.getSigner()).toBeDefined();
  });

  describe("getSigner", () => {
    it ("gets signer when explicitly provided", async () => {
      const signer = testnet.getSigner();
      expect(signer).toBeDefined();
      expect(await signer.getAddress()).toEqual(signerAddress);
    });

    it("gets signer from provider", async () => {
      const connection = new Connection({ provider: ethProvider });
      const signer = connection.getSigner();
      expect(signer).toBeDefined();
    });
  });

  describe("setSigner", () => {
    it ("sets signer from ethers Signer", async () => {
      const connection = new Connection({ provider: ethProvider });
      connection.setSigner(new Wallet(
        "0x4f3edf983ac636a65a842ce7c78d9aa706d3b113bce9c46f30d7d21715b23b1d"
      ));
      const signer = connection.getSigner();
      expect(signer).toBeDefined();
      expect(await signer.getAddress()).toEqual(signerAddress);
    });

    it("sets signer from account index", async () => {
      testnet.setSigner(1);
      const signer = testnet.getSigner();
      expect(signer).toBeDefined();
      expect(await signer.getAddress()).not.toEqual(signerAddress);
      testnet.setSigner(0);
    });

    it("sets signer from address", async () => {
      const connection = new Connection({ provider: ethProvider });
      connection.setSigner(signerAddress);
      const signer = connection.getSigner();
      expect(signer).toBeDefined();
      expect(await signer.getAddress()).toEqual(signerAddress);
    });
  });
});