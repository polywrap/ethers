# @polywrap/ethereum-provider-js
The Ethereum Provider plugin implements the `ethereum-provider-interface` @ [ens/wrappers.polywrap.eth:ethereum-provider@1.0.0](https://app.ens.domains/name/wrappers.polywrap.eth/details) (see [../../interface/src/schema.graphql](../../interface/src/schema.graphql)). It handles Ethereum wallet transaction signatures and sends JSON RPC requests for the Ethereum wrapper.

## Usage
### 1. Configure Client
When creating your Polywrap JS client, add the ethereum provider plugin:
```typescript
import { PolywrapClient } from "@polywrap/client-js";
import { ethereumProviderPlugin } from "@polywrap/ethereum-provider-js";

const client = new PolywrapClient({
  // 1. Add the plugin package @ an arbitrary URI
  packages: [{
    uri: "plugin/ethereum-provider-js",
    package: ethereumProviderPlugin({ })
  }],
  // 2. Register this plugin as an implementation of the interface
  interfaces: [{
    interface: "ens/wrappers.polywrap.eth:ethereum-provider@1.0.0",
    implementations: ["plugin/ethereum-provider-js"]
  }],
});
```

### 2. Invoke The Ethereum Wrapper
Invocations to the Ethereum wrapper may trigger sub-invocations to the Ethereum Provider plugin:
```typescript
await client.invoke({
  uri: "ens/wrappers.polywrap.eth:ethereum@1.0.0",
  method: "getSignerAddress",
});
```
