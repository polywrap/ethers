import { PolywrapClient, ClientConfigBuilder } from "@polywrap/client-js";

import { BigNumber, Wallet } from "ethers";

import { ethereumProviderPlugin, Connection, Connections } from "../src";

jest.setTimeout(360000);

describe("Ethereum Plugin", () => {
  let client: PolywrapClient;
  let clientNoSigner: PolywrapClient;
  const uri = "wrap://plugin/ethereum-provider";

  beforeAll(async () => {
    client = new PolywrapClient(
      new ClientConfigBuilder().addPackage(
        uri,
        ethereumProviderPlugin({
          connections: new Connections({
            networks: {
              binance: new Connection({
                provider: "https://bsc-dataseed1.binance.org/",
                signer: new Wallet("0x4f3edf983ac636a65a842ce7c78d9aa706d3b113bce9c46f30d7d21715b23b1d"),
              })
            },
            defaultNetwork: "binance",
          })
        }),
      ).build()
    );

    clientNoSigner = new PolywrapClient(
      new ClientConfigBuilder().addPackage(
        uri,
        ethereumProviderPlugin({
          connections: new Connections({
            networks: {
              binance: new Connection({
                provider: "https://bsc-dataseed1.binance.org/",
              })
            },
            defaultNetwork: "binance",
          })
        }),
      ).build()
    );
  });

  describe("EthereumProviderPlugin", () => {
    it("eth_chainId", async () => {
      const response = await client.invoke<string>({
        uri,
        method: "request",
        args: { method: "eth_chainId" }
      });

      if (response.ok === false) throw response.error;
      expect(response.value).toBeDefined();

      const res = BigNumber.from(JSON.parse(response.value)).toString();
      expect(res).toBe("56");
    });

    it("eth_getTransactionCount", async () => {
      const response = await client.invoke<string>({
        uri,
        method: "request",
        args: {
          method: "eth_getTransactionCount",
          params: `["0x3f349bBaFEc1551819B8be1EfEA2fC46cA749aA1","latest"]`
        }
      });

      if (response.ok === false) throw response.error;
      expect(response.value).toBeDefined();
      expect(BigNumber.from(JSON.parse(response.value)).gt(0)).toBe(true);
    });

    it("signerAddress", async () => {
      const response = await client.invoke<string | undefined>({
        uri,
        method: "signerAddress",
      });

      if (response.ok === false) throw response.error;
      expect(response.value).toBeDefined();

      expect(response.value?.startsWith("0x")).toBe(true);
    });

    it("signerAddress - no signer", async () => {
      const response = await clientNoSigner.invoke<string | undefined>({
        uri,
        method: "signerAddress",
      });

      if (response.ok === false) throw response.error;
      expect(response.value).toBeDefined();
      expect(response.value).toBe(null);
    });

    it("signMessage", async () => {
      const message = new TextEncoder().encode("Hello World");
      const response = await client.invoke<string | undefined>({
        uri,
        method: "signMessage",
        args: { message },
      });

      if (response.ok === false) throw response.error;

      expect(response.value).toBeDefined();
      expect(response.value).toBe("0xa4708243bf782c6769ed04d83e7192dbcf4fc131aa54fde9d889d8633ae39dab03d7babd2392982dff6bc20177f7d887e27e50848c851320ee89c6c63d18ca761c");
    });

    it("signTransaction", async () => {
      const rlp = [2,249,6,17,130,5,57,10,132,178,208,94,0,132,216,200,6,146,131,5,188,169,128,128,185,5,249,96,128,96,64,82,52,128,21,97,0,16,87,96,0,128,253,91,80,97,5,217,128,97,0,32,96,0,57,96,0,243,254,96,128,96,64,82,52,128,21,97,0,16,87,96,0,128,253,91,80,96,4,54,16,97,0,76,87,96,0,53,96,224,28,128,99,30,216,63,212,20,97,0,81,87,128,99,96,254,71,177,20,97,0,109,87,128,99,109,76,230,60,20,97,0,137,87,128,99,209,51,25,196,20,97,0,167,87,91,96,0,128,253,91,97,0,107,96,4,128,54,3,129,1,144,97,0,102,145,144,97,2,246,86,91,97,0,197,86,91,0,91,97,0,135,96,4,128,54,3,129,1,144,97,0,130,145,144,97,3,59,86,91,97,1,22,86,91,0,91,97,0,145,97,1,89,86,91,96,64,81,97,0,158,145,144,97,4,101,86,91,96,64,81,128,145,3,144,243,91,97,0,175,97,1,98,86,91,96,64,81,97,0,188,145,144,97,4,67,86,91,96,64,81,128,145,3,144,243,91,129,129,96,1,145,144,97,0,214,146,145,144,97,1,244,86,91,80,127,119,1,244,158,185,170,190,136,144,99,21,8,169,9,46,171,181,17,163,69,102,195,15,45,148,255,68,32,218,28,203,19,51,131,131,96,64,81,97,1,10,147,146,145,144,97,3,232,86,91,96,64,81,128,145,3,144,161,80,80,86,91,128,96,0,129,144,85,80,127,124,148,169,72,72,213,133,155,26,48,200,135,220,87,64,191,141,28,247,137,119,155,233,10,221,161,208,211,77,210,80,34,51,130,96,64,81,97,1,78,146,145,144,97,4,26,86,91,96,64,81,128,145,3,144,161,80,86,91,96,0,128,84,144,80,144,86,91,96,96,96,1,128,84,97,1,113,144,97,5,26,86,91,128,96,31,1,96,32,128,145,4,2,96,32,1,96,64,81,144,129,1,96,64,82,128,146,145,144,129,129,82,96,32,1,130,128,84,97,1,157,144,97,5,26,86,91,128,21,97,1,234,87,128,96,31,16,97,1,191,87,97,1,0,128,131,84,4,2,131,82,145,96,32,1,145,97,1,234,86,91,130,1,145,144,96,0,82,96,32,96,0,32,144,91,129,84,129,82,144,96,1,1,144,96,32,1,128,131,17,97,1,205,87,130,144,3,96,31,22,130,1,145,91,80,80,80,80,80,144,80,144,86,91,130,128,84,97,2,0,144,97,5,26,86,91,144,96,0,82,96,32,96,0,32,144,96,31,1,96,32,144,4,129,1,146,130,97,2,34,87,96,0,133,85,97,2,105,86,91,130,96,31,16,97,2,59,87,128,53,96,255,25,22,131,128,1,23,133,85,97,2,105,86,91,130,128,1,96,1,1,133,85,130,21,97,2,105,87,145,130,1,91,130,129,17,21,97,2,104,87,130,53,130,85,145,96,32,1,145,144,96,1,1,144,97,2,77,86,91,91,80,144,80,97,2,118,145,144,97,2,122,86,91,80,144,86,91,91,128,130,17,21,97,2,147,87,96,0,129,96,0,144,85,80,96,1,1,97,2,123,86,91,80,144,86,91,96,0,128,131,96,31,132,1,18,97,2,169,87,96,0,128,253,91,130,53,144,80,103,255,255,255,255,255,255,255,255,129,17,21,97,2,194,87,96,0,128,253,91,96,32,131,1,145,80,131,96,1,130,2,131,1,17,21,97,2,218,87,96,0,128,253,91,146,80,146,144,80,86,91,96,0,129,53,144,80,97,2,240,129,97,5,140,86,91,146,145,80,80,86,91,96,0,128,96,32,131,133,3,18,21,97,3,9,87,96,0,128,253,91,96,0,131,1,53,103,255,255,255,255,255,255,255,255,129,17,21,97,3,35,87,96,0,128,253,91,97,3,47,133,130,134,1,97,2,151,86,91,146,80,146,80,80,146,80,146,144,80,86,91,96,0,96,32,130,132,3,18,21,97,3,77,87,96,0,128,253,91,96,0,97,3,91,132,130,133,1,97,2,225,86,91,145,80,80,146,145,80,80,86,91,97,3,109,129,97,4,156,86,91,130,82,80,80,86,91,96,0,97,3,127,131,133,97,4,139,86,91,147,80,97,3,140,131,133,132,97,4,216,86,91,97,3,149,131,97,5,123,86,91,132,1,144,80,147,146,80,80,80,86,91,96,0,97,3,171,130,97,4,128,86,91,97,3,181,129,133,97,4,139,86,91,147,80,97,3,197,129,133,96,32,134,1,97,4,231,86,91,97,3,206,129,97,5,123,86,91,132,1,145,80,80,146,145,80,80,86,91,97,3,226,129,97,4,206,86,91,130,82,80,80,86,91,96,0,96,64,130,1,144,80,97,3,253,96,0,131,1,134,97,3,100,86,91,129,129,3,96,32,131,1,82,97,4,16,129,132,134,97,3,115,86,91,144,80,148,147,80,80,80,80,86,91,96,0,96,64,130,1,144,80,97,4,47,96,0,131,1,133,97,3,100,86,91,97,4,60,96,32,131,1,132,97,3,217,86,91,147,146,80,80,80,86,91,96,0,96,32,130,1,144,80,129,129,3,96,0,131,1,82,97,4,93,129,132,97,3,160,86,91,144,80,146,145,80,80,86,91,96,0,96,32,130,1,144,80,97,4,122,96,0,131,1,132,97,3,217,86,91,146,145,80,80,86,91,96,0,129,81,144,80,145,144,80,86,91,96,0,130,130,82,96,32,130,1,144,80,146,145,80,80,86,91,96,0,97,4,167,130,97,4,174,86,91,144,80,145,144,80,86,91,96,0,115,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,130,22,144,80,145,144,80,86,91,96,0,129,144,80,145,144,80,86,91,130,129,131,55,96,0,131,131,1,82,80,80,80,86,91,96,0,91,131,129,16,21,97,5,5,87,128,130,1,81,129,132,1,82,96,32,129,1,144,80,97,4,234,86,91,131,129,17,21,97,5,20,87,96,0,132,132,1,82,91,80,80,80,80,86,91,96,0,96,2,130,4,144,80,96,1,130,22,128,97,5,50,87,96,127,130,22,145,80,91,96,32,130,16,129,20,21,97,5,70,87,97,5,69,97,5,76,86,91,91,80,145,144,80,86,91,127,78,72,123,113,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,96,0,82,96,34,96,4,82,96,36,96,0,253,91,96,0,96,31,25,96,31,131,1,22,144,80,145,144,80,86,91,97,5,149,129,97,4,206,86,91,129,20,97,5,160,87,96,0,128,253,91,80,86,254,162,100,105,112,102,115,88,34,18,32,196,241,124,254,77,106,146,63,127,224,120,183,164,55,55,91,34,3,91,123,87,239,122,43,220,8,107,220,27,225,92,53,100,115,111,108,99,67,0,8,3,0,51,192];
      const response = await client.invoke<string | undefined>({
        uri,
        method: "signTransaction",
        args: { rlp },
      });

      if (response.ok === false) throw response.error;

      expect(response.value).toBeDefined();
      expect(response.value).toBe("0xeb91a997a865e2e4a48c098ea519666ed7fa5d9922f4e7e9b6838dc18ecfdab03a568682c3f0a4cb6b78ef0f601117a0de9848c089c94c01f782f067404c1eae1b");
    });

    it("signTypedData", async () => {
      const domain = {
        name: 'Ether Mail',
        version: '1',
        chainId: 1,
        verifyingContract: '0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC'
    };
    
    // The named list of all type definitions
    const types = {
      EIP712Domain: [
        {
          type: "string",
          name: "name"
        },
        {
          type: "string",
          name: "version"
        },
        {
          type: "uint256",
          name: "chainId",
        },
        {
          type: "address",
          name: "verifyingContract",
        },
      ],
      Person: [
          { name: 'name', type: 'string' },
          { name: 'wallet', type: 'address' }
      ],
      Mail: [
          { name: 'from', type: 'Person' },
          { name: 'to', type: 'Person' },
          { name: 'contents', type: 'string' }
      ]
    };
    
    // The data to sign
    const message = {
        from: {
            name: 'Cow',
            wallet: '0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826'
        },
        to: {
            name: 'Bob',
            wallet: '0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB'
        },
        contents: 'Hello, Bob!'
    };
  
      const response = await client.invoke<string>({
        uri,
        method: "request",
        args: {
          method: "eth_signTypedData_v4",
          params: JSON.stringify(["0x90F8bf6A479f320ead074411a4B0e7944Ea8c9C1", { domain, primaryType: 'Mail', types, message }])
        }
      });
      if (response.ok === false) throw response.error;
      expect(response.value).toBe(
        "\"0x12bdd486cb42c3b3c414bb04253acfe7d402559e7637562987af6bd78508f38623c1cc09880613762cc913d49fd7d3c091be974c0dee83fb233300b6b58727311c\""
      );
    });

    describe("eth_encodePacked", () => {
      it("should encode packed [int16, uint48]", async () => {
        const response = await client.invoke<string>({
          uri,
          method: "request",
          args: {
            method: "eth_encodePacked",
            params: JSON.stringify({
              types: [ "int16", "uint48" ],
              values: [ "-1", "12" ]
            })
          }
        });
        if (response.ok === false) throw response.error;
        expect(response.value).toEqual(`"0xffff00000000000c"`);
      });

      it("should encode packed [uint256, uint256]", async () => {
        const response = await client.invoke<string>({
          uri,
          method: "request",
          args: {
            method: "eth_encodePacked",
            params: JSON.stringify({
              types: [ "uint256", "uint256" ],
              values: [ "8", "16" ]
            })
          }
        });
        if (response.ok === false) throw response.error;
        expect(response.value).toEqual(`"0x00000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000010"`);
      });

      it("should encode packed [string, uint8]", async () => {
        const response = await client.invoke<string>({
          uri,
          method: "request",
          args: {
            method: "eth_encodePacked",
            params: JSON.stringify({
              types: [ "string", "uint8" ],
              values: [ "Hello", "3" ]
            })
          }
        });
        if (response.ok === false) throw response.error;
        expect(response.value).toEqual(`"0x48656c6c6f03"`);
      });
    })
  });
});
