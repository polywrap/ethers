from typing import Callable
from polywrap_client import PolywrapClient
from polywrap_core import InvokerOptions, UriPackageOrWrapper, Uri
from gzip import decompress
from base64 import b64decode

WithSigner = bool
provider_uri = Uri.from_str("plugin/ethereum-provider")

RLP = "H4sIAPtRQWQC/31UXWhcRRSeuT8lzYubmF1JDGZZfROtGLehtNBAreYhKZMluVcZLXNMjNYatASsJIWdmXtva7HCvbspPlRLq0KJBaVCkUKxRVsoiKX+kpYIrQ/+9EWlRelDE8/cu5ukKM7D3XPn+873nTlz7lq31rRpd0Nr+Mml7SScv7CmFrin56Q85d4SUvRXHpN5IDlfEHmbM3AvSyBFQTYIcmPx37hw1ueADOJbWVztlmP3zW/+rgPIsC/HxOKTJzCcxHBy8OdNGO7H8OveznMYHvN5KgBkp3DkelvRGMhEEoP1l4eb5z1O8GffCqYRszciRgsZlgB92uOifxjIYcSc59MXmdjxDUQ/BvpcAz1t0C2rUKUExT3yQw2f9KbHWXU3vXn41IefvRGP5Vvm1j58/GTb+1snvrjrodmlx4tXur+8uzcIjBZtrZsc+9dltaOMeVwKouJRVt0zOzcw8H10qOuRC/sW/P4zb3X/vX/3od9afzx66duhb1ipV6ci24yG07VKwzPNGIlZjIEQVI4A3RWD22Wke6goysSxRJEiP1b4rEhUUKqCW9pw321w80Cv+5iRA3rGB0pkMOJYQSVBHnbrusc1nlsQzBOkGHM1oiqxoDRGXAZtQL/ydWyLngLSODMrrchYWCSzWMlO64odRWsarBLefzQK1g5kG3troy/LYqmzEEh6TxOQFL2iUZ3HVz/RlGvVhvGLvi7r0bTGtBiwhjzO0Rqs18w8TOP9YBVcasOuo5MSBLuNXLBmMhC7F4iekLaDNZcNpi7H7IWlxkp9Ps8AUQxowgJBtRVQs38l26+xWpzegyob6z8UuAc8XktYdjmYFtntebDXNmRIQMt3GNj3ZwjY6yK9F2t72zOijC0ri6IOU42hpgbYPNQRcq95HJ0yN7AnFTjvYMcqmTnY1SAC502P1xnYB4IoBGfe8A4G4GIHQhqzOho1yMc1ONLAJ9VK1nkViSJW5fxikIuqkbjK9Cc0vbhiKvo1yoJ925x0L9jjHn43tuleBZycCnFrCueBzdYZuyPDWWcyojQDnE0mIwT7MpbRLBH7gESjRlK1ZxUSjhixZrszBjjThpFlNyE1HLMk66fWlYzYxMA5hmf/yCg1KFNL/7F0YRlXjUiroA+9sJxGjTxQuTy4ri81HVYhRSeVloRfUWCu2+3ASwwR4Mvnt7QT41jqggT3UV9UdSFhOG86pzqQ/4QP7lZwB81wp57VbQMzu8j/LPOtlYRTEQ8IYqZF9HSKnoA2qwf3YHZlqgPcI43/Y2/xvfEdr05MPVVqL577c8/i0Eu1zdWrr3/6QV8fL9l8xv99+sGFlp0L9157pjw+9crLY1tIi016z/4DcyP5khUGAAA="


async def test_sign_transaction(client_factory: Callable[[WithSigner], PolywrapClient]):
    rlp = decompress(b64decode(RLP))
    options: InvokerOptions[UriPackageOrWrapper] = InvokerOptions(
        uri=provider_uri,
        method="signTransaction",
        args={"rlp": rlp},
        encode_result=False,
    )
    client = client_factory(True)
    result = await client.invoke(options)

    assert (
        result
        == "0xeb91a997a865e2e4a48c098ea519666ed7fa5d9922f4e7e9b6838dc18ecfdab03a568682c3f0a4cb6b78ef0f601117a0de9848c089c94c01f782f067404c1eae1b"
    )
