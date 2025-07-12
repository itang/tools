import os


def batch_rename_suffix(path: str,
                        old_suffix: str,
                        new_suffix: str,
                        recursive=True,
                        dry_run=False) -> None:
    """
    批量修改文件后缀（支持递归和预览模式）

    :param path: 目标目录路径
    :param old_suffix: 原后缀（如 ".jpg"，空字符串表示无后缀文件）
    :param new_suffix: 新后缀（如 ".png"，必须以点开头）
    :param recursive: 是否递归子目录（默认True）
    :param dry_run: 预览模式（仅显示变更，不实际执行）
    """
    counter = 0
    # 自动补充后缀名的点符号[3](@ref)
    if old_suffix and not old_suffix.startswith('.'):
        old_suffix = '.' + old_suffix
    if new_suffix and not new_suffix.startswith('.'):
        new_suffix = '.' + new_suffix

    # 遍历文件系统[1,5](@ref)
    for root, dirs, files in os.walk(path):
        if not recursive and root != path:
            continue  # 跳过子目录

        for file in files:
            old_path = os.path.join(root, file)

            # 情况1：修改指定后缀的文件[1,2](@ref)
            if old_suffix:
                if file.endswith(old_suffix):
                    # 构建新文件名（保留主名）[4](@ref)
                    new_file = file[:-len(old_suffix)] + new_suffix
                    new_path = os.path.join(root, new_file)

                    # 检查文件冲突[7](@ref)
                    if os.path.exists(new_path):
                        print(f"⚠️ 冲突跳过: {file} → 目标已存在")
                        continue

                    # 执行重命名[8](@ref)
                    if not dry_run:
                        try:
                            os.replace(old_path, new_path)  # 跨平台安全替换[8](@ref)
                            counter += 1
                            print(f"✅ 重命名: {file} → {new_file}")
                        except Exception as e:
                            print(f"❌ 错误: {file} → {str(e)}")
                    else:
                        print(f"🚧 预览: {file} → {new_file}")
                        counter += 1

            # 情况2：给无后缀文件添加后缀[1](@ref)
            elif not os.path.splitext(file)[1]:
                new_path = os.path.join(root, file + new_suffix)

                if os.path.exists(new_path):
                    print(f"⚠️ 冲突跳过: {file} → 目标已存在")
                    continue

                if not dry_run:
                    try:
                        os.replace(old_path, new_path)
                        counter += 1
                        print(f"✅ 添加后缀: {file} → {file}{new_suffix}")
                    except Exception as e:
                        print(f"❌ 错误: {file} → {str(e)}")
                else:
                    print(f"🚧 预览: {file} → {file}{new_suffix}")
                    counter += 1

    print(f"\n操作完成！共处理 {counter} 个文件")
