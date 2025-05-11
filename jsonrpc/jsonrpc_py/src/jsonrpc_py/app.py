import json
import requests

def hello() -> str:
    return "Hello 你好 from jsonrpc_py!"


def jsonrpc(url: str, body: str) -> None:
    print("post", url, body)
    resp = requests.post(url, data=body, headers={"Content-Type":"application/json"})
    print('=>', resp.status_code)
    
    obj = json.loads(str(resp.content, 'utf8'))
    print(json.dumps(obj, indent=2))
