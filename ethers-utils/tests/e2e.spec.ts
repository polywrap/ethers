import { ClientConfigBuilder, PolywrapClient } from "@polywrap/client-js";
import { keccak256 } from "js-sha3";


describe("Ethereum Wrapper", () => {
  let client: PolywrapClient;

  const dirname: string = path.resolve(__dirname);
  const wrapperPath: string = path.join(dirname, "..");
  const uri = `fs/${wrapperPath}/build`;

  beforeAll(async () => {

    const config = new ClientConfigBuilder()
      .addDefaults()
      .addInterfaceImplementation(
        "wrap://ens/wraps.eth:ethereum-provider@1.1.0",
        "wrap://package/ethereum-provider"
      )
      .build();

    client = new PolywrapClient(config);


  });


  describe("ethereum utils", () => {
    it("should calculate create 2 address", async () => {
      const response = await client.invoke<string>({
        uri,
        method: "generateCreate2Address",
        args: {
          address: "0x1d90fCc0423cCC9650392E799d4d6da9530aCA43",
          salt: "0x1233388b1647069152c5f8794f8ed87af2edb8cfc397d78e053347bbfe6398b3",
          initCode: "0x608060405234801561001057600080fd5b506040516101e63803806101e68339818101604052602081101561003357600080fd5b8101908080519060200190929190505050600073ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1614156100ca576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260228152602001806101c46022913960400191505060405180910390fd5b806000806101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505060ab806101196000396000f3fe608060405273ffffffffffffffffffffffffffffffffffffffff600054167fa619486e0000000000000000000000000000000000000000000000000000000060003514156050578060005260206000f35b3660008037600080366000845af43d6000803e60008114156070573d6000fd5b3d6000f3fea264697066735822122003d1488ee65e08fa41e58e888a9865554c535f2c77126a82cb4c0f917f31441364736f6c63430007060033496e76616c69642073696e676c65746f6e20616464726573732070726f7669646564000000000000000000000000b7f8bc63bbcad18155201308c8f3540b07f84f5e"
        }
      });
      if (!response.ok) throw response.error;
      expect(response.value).toEqual("0x57d4d0c68057Cc9446F93307082D63466BC3D731".toLowerCase());
    });

    it.skip("should encode packed", async () => {
      const response = await client.invoke<string>({
        uri,
        method: "wEncodePacked",
        args: {
          bytes: "0x2fe2c0ec0d2f63b668a3389b17cfed8ec8554e2cd759b305b8873ea03353a360",
          uint: "0x0000000000000000000000000000000000000000000000000000000000000042",
        }
      });
      if (!response.ok) throw response.error;
      expect(response.value).toEqual("0x169b91711c9e5fc8418feaca506caa84243dc031eb336f195d6399e79978f138".toLowerCase());
   
    });

    it("should encode bytes and convert to keccak", async () => {
      const response = await client.invoke<string>({
        uri,
        method: "keccak256BytesEncodePacked",
        args: {
          bytes: "0x2fe2c0ec0d2f63b668a3389b17cfed8ec8554e2cd759b305b8873ea03353a3600000000000000000000000000000000000000000000000000000000000000042",
        }
      });
      if (!response.ok) throw response.error;
      expect(response.value).toEqual("0x169b91711c9e5fc8418feaca506caa84243dc031eb336f195d6399e79978f138".toLowerCase());
   
    });

    it("should encode keccak256", async () => {
      let input = "0xe1c7392a"
      const response = await client.invoke<string>({
        uri,
        method: "keccak256Bytes",
        args: {
          bytes: input,
        }
      });
      if (!response.ok) throw response.error;
      expect(response.value).toEqual(keccak256(input));
   
    });
  })
});