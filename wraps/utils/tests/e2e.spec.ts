import { PolywrapClient, PolywrapClientConfigBuilder } from "@polywrap/client-js";
import * as path from "path";
import { ethers, utils } from "ethers";

jest.setTimeout(360000);

describe("Ethereum Wrapper", () => {
  let client: PolywrapClient;

  const dirname: string = path.resolve(__dirname);
  const wrapperPath: string = path.join(dirname, "..");
  const uri = `fs/${wrapperPath}/build`;

  beforeAll(async () => {
    const config = new PolywrapClientConfigBuilder()
      .addDefaults()
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
          initCode:
            "0x608060405234801561001057600080fd5b506040516101e63803806101e68339818101604052602081101561003357600080fd5b8101908080519060200190929190505050600073ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1614156100ca576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260228152602001806101c46022913960400191505060405180910390fd5b806000806101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505060ab806101196000396000f3fe608060405273ffffffffffffffffffffffffffffffffffffffff600054167fa619486e0000000000000000000000000000000000000000000000000000000060003514156050578060005260206000f35b3660008037600080366000845af43d6000803e60008114156070573d6000fd5b3d6000f3fea264697066735822122003d1488ee65e08fa41e58e888a9865554c535f2c77126a82cb4c0f917f31441364736f6c63430007060033496e76616c69642073696e676c65746f6e20616464726573732070726f7669646564000000000000000000000000b7f8bc63bbcad18155201308c8f3540b07f84f5e",
        },
      });
      if (!response.ok) throw response.error;
      expect(response.value).toEqual(
        "0x57d4d0c68057Cc9446F93307082D63466BC3D731".toLowerCase()
      );
    });

    it("should encode bytes and convert to keccak", async () => {
      const response = await client.invoke<string>({
        uri,
        method: "keccak256BytesEncodePacked",
        args: {
          value:
            "0x2fe2c0ec0d2f63b668a3389b17cfed8ec8554e2cd759b305b8873ea03353a3600000000000000000000000000000000000000000000000000000000000000042",
        },
      });
      if (!response.ok) throw response.error;
      expect(response.value).toEqual(
        "0x169b91711c9e5fc8418feaca506caa84243dc031eb336f195d6399e79978f138".toLowerCase()
      );
    });

    it("should encode keccak256", async () => {
      let input = "0xe1c7392a";
      const response = await client.invoke<string>({
        uri,
        method: "keccak256",
        args: {
          value: input,
        },
      });
      if (!response.ok) throw response.error;
      expect(response.value).toEqual(utils.keccak256(input));
    });

    it("should encode meta transaction", async () => {
      let to = "0xb09bCc172050fBd4562da8b229Cf3E45Dc3045A6";
      let value = "1";
      let data =
        "0xa9059cbb000000000000000000000000ffcf8fdee72ac11b5c542428b35eef5769c409f00000000000000000000000000000000000000000000000000f43fc2c04ee0000";
      let operation = "0";
      const response = await client.invoke<string>({
        uri,
        method: "encodeMetaTransaction",
        args: {
          to,
          value,
          data,
          operation,
        },
      });
      if (!response.ok) throw response.error;
      expect(response.value).toEqual(
        "0x00b09bcc172050fbd4562da8b229cf3e45dc3045a600000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000044a9059cbb000000000000000000000000ffcf8fdee72ac11b5c542428b35eef5769c409f00000000000000000000000000000000000000000000000000f43fc2c04ee0000"
      );
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

    it("encodeParams - address array", async () => {
      const response = await client.invoke<string>({
        uri,
        method: "encodeParams",
        args: {
          types: ["address[]"],
          values: ["[0x0000000000000000000000000000000000000001,0x0000000000000000000000000000000000000001]"],
        },
      });

      if (!response.ok) throw response.error;

      const expected = ethers.utils.defaultAbiCoder.encode(
        ["address[]"],
        [["0x0000000000000000000000000000000000000001", "0x0000000000000000000000000000000000000001"]]
      );
      expect(response.value).toBe(expected);
    });

    it.only("encodeParams", async () => {
      const response = await client.invoke<string>({
        uri,
        method: "encodeParams",
        args: {
          types: ["uint256"],
          values: ["0xb1073742015cbcf5a3a4d9d1ae33ecf619439710b89475f92e2abd2117e90f90"],
        },
      });

      console.log(response)
      if (!response.ok) throw response.error;

      const expected = ethers.utils.defaultAbiCoder.encode(
        ["uint256"],
        ["0xb1073742015cbcf5a3a4d9d1ae33ecf619439710b89475f92e2abd2117e90f90"]
      );
      expect(response.value).toBe(expected);
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

    it("encodeFunction - address[]", async () => {
      const method = "function setup(address[] _owners,uint256 _threshold,address to,bytes data,address fallbackHandler,address paymentToken,uint256 payment,address paymentReceiver)";
      const signer = "0x0000000000000000000000000000000000000001";

      const response = await client.invoke<string>({
        uri,
        method: "encodeFunction",
        args: {
          method,
          args: [
            "[\"" + signer + "\"]",
            "1",
            signer,
            "0x",
            signer,
            signer,
            "0",
            signer,
          ],
        },
      });

      if (!response.ok) throw response.error;

      const functionInterface = ethers.Contract.getInterface([method]);
      const expected = functionInterface.encodeFunctionData(
        functionInterface.functions[Object.keys(functionInterface.functions)[0]],
        [
          [signer],
          "1",
          signer,
          "0x",
          signer,
          signer,
          "0",
          signer,
        ]
      );
      expect(response.value).toBe(expected);
    });

    it("encodeFunction - address[] with quotes around address strings", async () => {
      const method = "function setup(address[] _owners,uint256 _threshold,address to,bytes data,address fallbackHandler,address paymentToken,uint256 payment,address paymentReceiver)";
      const signer = "0xd405aebF7b60eD2cb2Ac4497Bddd292DEe534E82";
      const zeroAddr = "0x0000000000000000000000000000000000000000"
      
      const response = await client.invoke<string>({
        uri,
        method: "encodeFunction",
        args: {
          method,
          args: [
            "[\"" + signer + "\"]",
            "1",
            zeroAddr,
            "0x",
            zeroAddr,
            zeroAddr,
            "0",
            zeroAddr,
          ],
        },
      });

      if (!response.ok) throw response.error;

      const functionInterface = ethers.Contract.getInterface([method]);
      const expected = functionInterface.encodeFunctionData(
        functionInterface.functions[Object.keys(functionInterface.functions)[0]],
        [
          [signer],
          "1",
          zeroAddr,
          "0x",
          zeroAddr,
          zeroAddr,
          "0",
          zeroAddr,
        ]
      );
      expect(response.value).toBe(expected);
    });

    describe("Amount formatting", () => {
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

      describe("toEth", () => {
        it("handle integers", async () => {
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

        it("handle decimals", async () => {
          const response = await client.invoke<string>({
            uri,
            method: "toEth",
            args: {
              wei: "200000000000000000",
            },
          });

          if (!response.ok) throw response.error;
          expect(response.value).toBeDefined();
          expect(response.value).toEqual("0.2");
        });
      });
    });

    describe("solidityPack", () => {
      it("should encode packed [int16, uint48]", async () => {
        const response = await client.invoke<string>({
          uri,
          method: "solidityPack",
          args: {
            types: ["int16", "uint48"],
            values: ["-1", "12"],
          },
        });
        if (!response.ok) throw response.error;
        expect(response.value).toEqual("0xffff00000000000c");
      });

      it("should encode packed [uint256, uint256]", async () => {
        const response = await client.invoke<string>({
          uri,
          method: "solidityPack",
          args: {
            types: ["uint256", "uint256"],
            values: ["8", "16"],
          },
        });
        if (!response.ok) throw response.error;
        expect(response.value).toEqual(
          "0x00000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000010"
        );
      });

      it("should encode packed [string, uint8]", async () => {
        const response = await client.invoke<string>({
          uri,
          method: "solidityPack",
          args: {
            types: ["string", "uint8"],
            values: ["Hello", "3"],
          },
        });
        if (!response.ok) throw response.error;
        expect(response.value).toEqual("0x48656c6c6f03");
      });

      it("should encode packed [address, uint]", async () => {
        const response = await client.invoke<string>({
          uri,
          method: "solidityPack",
          args: {
            types: ["address", "uint"],
            values: ["0x8ba1f109551bd432803012645ac136ddd64dba72", "45"],
          },
        });
        if (!response.ok) throw response.error;
        expect(response.value).toEqual(
          "0x8ba1f109551bd432803012645ac136ddd64dba72000000000000000000000000000000000000000000000000000000000000002d"
        );
      });

      it("should encode packed [address[], uint]", async () => {
        const types = ["address[]", "uint"];
        const values = [
          "0x8ba1f109551bd432803012645ac136ddd64dba72,0x8ba1f109551bd432803012645ac136ddd64dba71",
          "45",
        ];

        const response = await client.invoke<string>({
          uri,
          method: "solidityPack",
          args: { types, values },
        });
        if (!response.ok) throw response.error;

        const expected = ethers.utils.solidityPack(types, [
          [
            "0x8ba1f109551bd432803012645ac136ddd64dba72",
            "0x8ba1f109551bd432803012645ac136ddd64dba71",
          ],
          "45",
        ]);
        expect(response.value).toEqual(expected);
      });

      it("should encode packed ethers", async () => {
        const types: string[] = [
          "string",
          "address",
          "bool",
          "uint8",
          "uint16",
          "uint32",
          "uint64",
          "uint128",
          "uint256",
          "uint",
          "int8",
          "int16",
          "int32",
          "int64",
          "int128",
          "int256",
          "int",
          "bytes",
          "bytes8",
          "bytes16",
          "bytes24",
          "bytes32",
        ];
        const exampleValues: Record<string, string> = {
          string: "hello world",
          address: "0x742d35Cc6634C0532925a3b844Bc454e4438f44e",
          bool: "true",
          uint8: "255",
          uint16: "65535",
          uint32: "4294967295",
          uint64: "18446744073709551615",
          uint128: "340282366920938463463374607431768211455",
          uint256:
            "115792089237316195423570985008687907853269984665640564039457584007913129639935",
          uint: "115792089237316195423570985008687907853269984665640564039457584007913129639935",
          int8: "127",
          int16: "32767",
          int32: "2147483647",
          int64: "9223372036854775807",
          int128: "170141183460469231731687303715884105727",
          int256:
            "57896044618658097711785492504343953926634992332820282019728792003956564819967",
          int: "57896044618658097711785492504343953926634992332820282019728792003956564819967",
          bytes: "0x1234abcd",
          bytes8: "0x1234567890abcdef",
          bytes16: "0x1234567890abcdef1234567890abcdef",
          bytes24: "0x1234567890abcdef1234567890abcdef1234567890abcdef",
          bytes32:
            "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
        };
        const values: string[] = Object.values(exampleValues);

        const response = await client.invoke<string>({
          uri,
          method: "solidityPack",
          args: { types, values },
        });
        if (!response.ok) throw response.error;

        const expected = ethers.utils.solidityPack(types, values);
        expect(response.value).toEqual(expected);
      });

      it("should encode packed negative numbers", async () => {
        const types: string[] = [
          "int8",
          "int16",
          "int32",
          "int64",
          "int128",
          "int256",
          "int",
        ];
        const exampleValues: Record<string, string> = {
          int8: "-12",
          int16: "-3276",
          int32: "-214748364",
          int64: "-922337203685477580",
          int128: "-17014118346046923173168730371588410572",
          int256:
            "-5789604461865809771178549250434395392663499233282028201972879200395656481996",
          int: "-5789604461865809771178549250434395392663499233282028201972879200395656481996",
        };
        const values: string[] = Object.values(exampleValues);

        const response = await client.invoke<string>({
          uri,
          method: "solidityPack",
          args: { types, values },
        });
        if (!response.ok) throw response.error;

        const expected = ethers.utils.solidityPack(types, values);
        expect(response.value).toEqual(expected);
      });
    });
  });
});
