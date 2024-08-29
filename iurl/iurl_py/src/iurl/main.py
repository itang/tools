from urllib.parse import urlparse, parse_qsl, parse_qs
import json
import argparse


def _get_args() -> argparse.Namespace:
    link = "http://39.104.227.186:8089/api/api/runtime/organization/department/tree?deptIds=&filterType=root_display&sourceType=portal&corpId=&excludeCorpId=&selectUserIds=&workflowInstanceId=&activityCode=&formRootDeptIds=&queryType="

    parser = argparse.ArgumentParser(description="My url pretty parser")
    parser.add_argument("--url", type=str, help="输入url", default=link)
    parser.add_argument(
        "urls_args",
        type=str,
        nargs="*",
        help="urls",
    )
    parser.add_argument(
        "--query-model",
        type=str,
        choices=["qsl", "qs"],
        default="qsl",
        help="query model",
    )

    return parser.parse_args()


def _main(args: argparse.Namespace) -> None:
    urls = args.urls_args or [args.url]
    for index, url in enumerate(urls):
        url_obj = urlparse(url)

        parse_query_fn = parse_qsl if args.query_model == "qsl" else parse_qs

        m = {
            "scheme": url_obj.scheme,
            "hostname": url_obj.hostname,
            "path": url_obj.path,
            "query": dict(parse_query_fn(url_obj.query, keep_blank_values=True)),
            "fragment": url_obj.fragment,
        }

        print(f"{index+1}: {url}")
        print(">>>>")
        print(json.dumps(m, indent=4, ensure_ascii=False))


def main() -> None:
    _main(_get_args())


if __name__ == "__main__":
    main()
