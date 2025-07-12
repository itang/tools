import argparse
import sys

from prename.app import batch_rename_suffix
from prename.decorators import debug_log, measure_time


def _get_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="批量文件后缀修改工具")
    parser.add_argument("path", help="目标目录路径")
    parser.add_argument("old_suffix", help="原后缀（空字符串表示无后缀文件）")
    parser.add_argument("new_suffix", help="新后缀（必须以点开头，如 .png）")
    parser.add_argument("--no-recursive", action="store_false", dest="recursive",
                        help="禁用递归处理子目录")
    parser.add_argument("--dry-run", action="store_true",
                        help="预览模式（不实际修改）")
    args = parser.parse_args()

    return args


@debug_log
@measure_time
def _main(args: argparse.Namespace) -> None:
    batch_rename_suffix(
        path=args.path,
        old_suffix=args.old_suffix.strip('\"'),
        new_suffix=args.new_suffix.strip('\"'),
        recursive=args.recursive,
        dry_run=args.dry_run
    )


def main() -> None:
    _main(_get_args())


if __name__ == "__main__":
    print(f"argv: {sys.argv}")
    main()
