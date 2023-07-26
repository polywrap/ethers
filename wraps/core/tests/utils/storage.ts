import { Connection } from "@polywrap/ethereum-wallet-js";
import { ContractFactory, Contract, Signer, ethers } from "ethers";
import {ETH_ENS_IPFS_MODULE_CONSTANTS} from "polywrap";

export async function deployStorage(
  abi: string[],
  bytecode: string
): Promise<string> {
  const signer = getSigner();
  const factory = new ContractFactory(abi, bytecode, signer);
  const contract = await factory.deploy();
  await contract.deployed();
  return contract.address;
}

export async function addStructToStorage(
  abi: string[],
  address: string,
  args: string[]
): Promise<void> {
  const signer = getSigner();
  const contract = new Contract(address, abi);
  const calldata = ethers.utils.defaultAbiCoder.encode(
    ["address", "uint256"],
    args
  );
  await contract.connect(signer).addJob(calldata);
}

export async function addPrimitiveToArrayStorage(
  abi: string[],
  address: string,
  data: string
): Promise<void> {
  const signer = getSigner();
  const contract = new Contract(address, abi);
  await contract.connect(signer).addSimple(data);
}

export async function setPrimitiveToStorage(
  abi: string[],
  address: string,
  data: string
): Promise<void> {
  const signer = getSigner();
  const contract = new Contract(address, abi);
  await contract.connect(signer).set(data);
}

function getSigner(): Signer {
  const connection = new Connection({ provider: ETH_ENS_IPFS_MODULE_CONSTANTS.ethereumProvider });
  return connection.getSigner();
}
