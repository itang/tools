import sys
import argparse

from .app import jsonrpc 


def _get_args():
    parser = argparse.ArgumentParser(description="JsonRpc parser")
    parser.add_argument("--url", type=str, help="输入url", default="http://127.0.0.1:8080/api/table/query")
    parser.add_argument(
        "body",
        type=str,
        help="body",
        nargs='?',
        default='{"data": {"columns":["id", "name"], "limit": 10, "table":"b_user", "count": false}}'
    )
    args = parser.parse_args()
    return args


def _main(args: argparse.Namespace) -> None:
    jsonrpc(args.url, args.body)


def main() -> None:
    _main(_get_args())


if __name__ == "__main__":
    print(f"argv: {sys.argv}")
    main()
