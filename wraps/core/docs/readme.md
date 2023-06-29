# The Ethers wrap

The Ethers wrap provides you with methods for easy interaction with the Ethereum Blockchain.

## Requirements

To run the Ethers wrap you'll need a Polywrap client in your application. See here for installation information: [https://docs.polywrap.io/clients](https://docs.polywrap.io/clients)

### Configuration

The Ethers wrap requires an [ethereum-provider plugin](https://github.com/polywrap/ethereum-wallet). Plugins are added directly to the client using its config.

[Here's an example](https://github.com/polywrap/ethers/blob/36e6f3331264732e73f3e236004416e82930ed64/provider/implementations/js/tests/index.spec.ts#L15-L30) of setting up a JavaScript / TypeScript client with the ethereum-provider plugin.

You can learn more about Polywrap clients & configs in the docs [here](https://docs.polywrap.io/tutorials/use-wraps/configure-client).

## Run!

With your client successfully configured, you can now run any function on the Ethers wrap with ease.

You can check out the Ethers wrap's schema for a list of methods, or [check out its tests](https://github.com/polywrap/ethers/tree/main/wraps/core/tests) for detailed usage examples.

## Support

For any questions or problems related to the Ethers wrap or Polywrap at large, please visit our [Discord](https://discord.polywrap.io).
