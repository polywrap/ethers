# import json

# class EthEncodePacked:
#     @staticmethod
#     def deserialize_parameters(params_str: str) -> dict:
#         params = json.loads(params_str)

#         if (isinstance(params, dict) and
#                 "types" in params and "values" in params and
#                 isinstance(params["types"], list) and isinstance(params["values"], list) and
#                 isinstance(params["types"][0], str)):

#             return {
#                 "types": params["types"],
#                 "values": EthEncodePacked.parse_values(params["values"])
#             }

#         raise ValueError(
#             "Invalid JSON-RPC parameters provided for eth_encodePacked method. "
#             "Expected JSON of the form: { types: string[], values: string[] }"
#         )

#     @staticmethod
#     def parse_values(values=None) -> list:
#         if values is None:
#             return []

#         parsed_values = []
#         for arg in values:
#             if (arg.startswith("[") and arg.endswith("]")) or (arg.startswith("{") and arg.endswith("}")):
#                 parsed_values.append(json.loads(arg))
#             else:
#                 parsed_values.append(arg)

#         return parsed_values


# params_str = '{"types": ["uint256", "address"], "values": ["42", "0x742d35Cc6634C0532925a3b844Bc454e4438f44e"]}'
# deserialized_params = EthEncodePacked.deserialize_parameters(params_str)
# print(deserialized_params)
