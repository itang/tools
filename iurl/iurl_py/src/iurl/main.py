import json
import argparse
from urllib.parse import ParseResult, urlparse, parse_qsl, parse_qs
from termcolor import colored
from typing import TypeVar


DEFAULT_URL = "http://39.104.227.186:8089/api/api/runtime/organization/department/tree?deptIds=&filterType=root_display&sourceType=portal&corpId=&excludeCorpId=&selectUserIds=&workflowInstanceId=&activityCode=&formRootDeptIds=&queryType=#/apps/main"


U = TypeVar("U")


def _identity(x: U) -> U:
    return x


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
        choices=["qsl", "qs", "raw"],
        default="qsl",
        help="query model",
    )

    return parser.parse_args()


def _format_port(port: int | None, scheme: str) -> str:
    if port:
        return f"{port}"
    elif scheme == "http":
        return "80"
    elif scheme == "https":
        return "443"
    else:
        return "0"


def _w(fn):  # type: ignore
    def wrapper(x: str | None):  # type: ignore
        return dict(fn(x, keep_blank_values=True))  # type: ignore

    return wrapper  # type: ignore


def _main(args: argparse.Namespace) -> None:
    urls = args.urls_args or [args.url]
    for index, url in enumerate(urls):
        url_obj: ParseResult = urlparse(url)  # type: ignore

        parse_query_fn = parse_qsl
        match args.query_model:
            case "qsl":
                parse_query_fn = _w(parse_qsl)  # type: ignore
            case "qs":
                parse_query_fn = _w(parse_qs)  # type: ignore
            case "raw":
                parse_query_fn = _identity
            case _:
                raise ValueError(f"Invalid query model: {args.query_model}")

        url_dict = {  # type: ignore
            "scheme": url_obj.scheme,  # type: ignore
            "hostname": url_obj.hostname,  # type: ignore
            "port": _format_port(url_obj.port, url_obj.scheme),  # type: ignore
            "path": url_obj.path,  # type: ignore
            "query": parse_query_fn(url_obj.query),  # type: ignore
            "fragment": url_obj.fragment,  # type: ignore
        }

        print(colored(f"{index+1}: {url}", "blue"))
        print(colored("\n>>>>\n", "light_grey"))
        print(colored(json.dumps(url_dict, indent=4, ensure_ascii=False), "green"))
        print()


def main() -> None:
    _main(_get_args())


if __name__ == "__main__":
    main()
