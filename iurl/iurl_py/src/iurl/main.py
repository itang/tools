from urllib.parse import urlparse, parse_qsl
import json
import argparse


def _get_args():
    link = "http://39.104.227.186:8089/api/api/runtime/organization/department/tree?deptIds=&filterType=root_display&sourceType=portal&corpId=&excludeCorpId=&selectUserIds=&workflowInstanceId=&activityCode=&formRootDeptIds=&queryType="

    parser = argparse.ArgumentParser(description="My url pretty parser")
    parser.add_argument("--url", type=str, help="输入url", default=link)
    parser.add_argument(
        "urls_args",
        type=str,
        nargs="*",
        help="urls",
    )
    args = parser.parse_args()
    return args


def _main(args: argparse.Namespace) -> None:
    urls = args.urls_args or [args.url]
    for index, url in enumerate(urls):
        url_obj = urlparse(url)

        m = {
            "scheme": url_obj.scheme,
            "hostname": url_obj.hostname,
            "path": url_obj.path,
            "query": dict(parse_qsl(url_obj.query, keep_blank_values=True)),
            "fragment": url_obj.fragment,
        }

        print(f"{index+1}: {url}")
        print(">>>>")
        print(json.dumps(m, indent=4, ensure_ascii=False))


def main() -> None:
    _main(_get_args())


if __name__ == "__main__":
    main()
