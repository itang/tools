from urllib.parse import urlparse, parse_qsl
import json

# get args


import argparse


def _get_args():
    link = "http://39.104.227.186:8089/api/api/runtime/organization/department/tree?deptIds=&filterType=root_display&sourceType=portal&corpId=&excludeCorpId=&selectUserIds=&workflowInstanceId=&activityCode=&formRootDeptIds=&queryType="

    parser = argparse.ArgumentParser(description="My url pretty parser")
    parser.add_argument("--url", type=str, help="输入url", default=link)
    args = parser.parse_args()
    return args


def _main(args: argparse.Namespace) -> None:
    url = urlparse(args.url)

    m = {
        "scheme": url.scheme,
        "hostname": url.hostname,
        "path": url.path,
        "query": dict(parse_qsl(url.query, keep_blank_values=True)),
        "fragment": url.fragment,
    }

    print(args.url)
    print(">>>>")
    print(json.dumps(m, indent=4))


def main() -> None:
    _main(_get_args())


if __name__ == "__main__":
    main()
