// based on free infura networks
export enum KnownNetworkId {
  mainnet = 1,
  goerli = 5,
  sepolia = 11155111,
  "celo-mainnet" = 42220,
  "celo-alfajores" = 44787,
  "avalanche-mainnet"= 43114,
  "avalanche-fuji" = 43113,
  "palm-mainnet" = 11297108109,
  "palm-testnet" = 11297108099,
  "aurora-mainnet" = 1313161554,
  "aurora-testnet" = 1313161555,
}

export type KnownNetwork = KnownNetworkId | (keyof typeof KnownNetworkId);
