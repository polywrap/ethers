import { Connections, Connection } from "../src";

jest.setTimeout(10000);

type BasicNetwork = "mainnet" | "goerli" | "sepolia";

const getRpcUri = (network: BasicNetwork): string => {
  return `https://${network}.infura.io/v3/d119148113c047ca90f0311ed729c466`;
}

describe("Connections Store", () => {
  let connections: Connections;
  let testnet: Connection;
  let goerli: Connection;


  beforeAll(async () => {
    testnet = new Connection({ provider: getRpcUri("sepolia") });
    goerli = new Connection({ provider: getRpcUri("goerli") });
    connections = new Connections({
      networks: { testnet, goerli },
      defaultNetwork: "testnet",
    });
  });


  test("get", async () => {
    expect(connections.get("testnet")).toBe(testnet);
    expect(connections.get("goerli")).toBe(goerli);
    expect(connections.get()).toBe(testnet);
    expect(connections.get("rinkeby")).toBeUndefined();
  });

  describe("set", () => {
    it("adds a new network", async () => {
      const sepolia = new Connection({ provider: getRpcUri("sepolia") });
      expect(connections.get("sepolia")).toBeUndefined();
      connections.set("sepolia", sepolia);
      expect(connections.get("sepolia")).toBe(sepolia);
    });

    it("adds new network by passing provider directly", async () => {
      const goerliUri = getRpcUri("goerli");
      connections.set("goerli", goerliUri);
      const providerUri = connections.get("goerli")?.getProvider().connection.url;
      expect(providerUri).toEqual(goerliUri);
    });

    it("replaces existing network", async () => {
      const ropsten = new Connection({ provider: getRpcUri("goerli") });
      connections.set("existingNetwork", ropsten);
      expect(connections.get("existingNetwork")).toBe(ropsten);
      connections.set("existingNetwork", goerli);
      expect(connections.get("existingNetwork")).toBe(goerli);
    });
  });

  test("getDefaultNetwork", async () => {
    expect(connections.getDefaultNetwork()).toEqual("testnet");
  });

  describe("setDefaultNetwork", () => {
    it("replaces defaultNetwork with existing network", async () => {
      connections.setDefaultNetwork("goerli");
      expect(connections.getDefaultNetwork()).toEqual("goerli");
    });

    it("replaces default network by passing a new connection", async () => {
      connections.setDefaultNetwork("newDefault", goerli);
      expect(connections.getDefaultNetwork()).toEqual("newDefault");
      expect(connections.get("newDefault")).toBe(goerli);
    });
  });
});