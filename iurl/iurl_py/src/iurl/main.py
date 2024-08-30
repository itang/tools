import json
import argparse
from urllib.parse import ParseResult, urlparse, parse_qsl, parse_qs
from termcolor import colored


DEFAULT_URL = "http://39.104.227.186:8089/api/api/runtime/organization/department/tree?deptIds=&filterType=root_display&sourceType=portal&corpId=&excludeCorpId=&selectUserIds=&workflowInstanceId=&activityCode=&formRootDeptIds=&queryType=#/apps/main"


def _get_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="My url pretty parser")
    parser.add_argument("--url", type=str, help="输入url", default=DEFAULT_URL)
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
        url_obj: ParseResult = urlparse(url)

        parse_query_fn = parse_qsl if args.query_model == "qsl" else parse_qs

        url_dict = {
            "scheme": url_obj.scheme,
            "hostname": url_obj.hostname,
            "port": url_obj.port,
            "path": url_obj.path,
            "query": dict(parse_query_fn(url_obj.query, keep_blank_values=True)),
            "fragment": url_obj.fragment,
        }

        print(colored(f"{index+1}: {url}", "blue"))
        print(colored("\n>>>>\n", "light_grey"))
        print(colored(json.dumps(url_dict, indent=4, ensure_ascii=False), "green"))
        print()


def main() -> None:
    _main(_get_args())


if __name__ == "__main__":
    main()
