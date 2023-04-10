from polywrap_plugin import PluginModule, PluginPackage
from polywrap_result import Ok, Result

class EthereumProvider(PluginModule[None]):
    def __init__(self):
        super().__init__(None)

    async def request(self) -> Result[str]:
        return Ok("works")