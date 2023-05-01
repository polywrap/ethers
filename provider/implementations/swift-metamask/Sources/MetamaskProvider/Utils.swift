import Foundation

func buildHttpRequest(body: Data?) -> URLRequest {
    let endpoint = URL(string: "https://eth-goerli.g.alchemy.com/v2/zzLPStDTNNQdylflTNS_JN9Pc6y_u8_u")!
    var request = URLRequest(url: endpoint)
    request.httpMethod = "post"
    request.httpBody = body
    request.addValue("application/json", forHTTPHeaderField: "accept")
    request.addValue("application/json", forHTTPHeaderField: "content-type")

    return request
}

func handleHttpRequest(req: URLRequest, completion: @escaping (Result<String, Error>) -> Void) {
            let task = URLSession.shared.dataTask(with: req) { (data, response, error) in
            if let error = error {
                return completion(.failure(error))
            } else if let data = data {
                let json = try! JSONSerialization.jsonObject(with: data, options: []) as! [String : Any]
                let stringJson = try! JSONSerialization.data(withJSONObject: json["result"])
                return completion(.success(String(data:stringJson, encoding: .utf8)!))
            } else {
                print("unexpected error")
            }
        }
           
       task.resume()
}
