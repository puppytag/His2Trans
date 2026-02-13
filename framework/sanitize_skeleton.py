import re
import sys
from pathlib import Path

"""
对骨架源码进行轻量级语法净化，尽量减少明显的语法噪声导致的全仓构建失败。
处理项（保守修改）：
1) 删除 Markdown 围栏行: 仅包含 ``` 的行
2) 将文件中部出现的 #![...] 改为 #[...]
   - 若出现在首行且紧随模块层级，保留为 #![...]（不过度改动）
3) 去除多余的右尖括号模式（常见 '>>' 末尾一枚多余），仅在行尾多出单独一个 '>' 时移除
"""

FENCE_LINE_RE = re.compile(r'^\s*```\s*$')
INNER_ATTR_RE = re.compile(r'^\s*#!\[(.+)\]\s*$')
OUTER_ATTR_FMT = '#[{}]'

def process_file(path: Path):
    try:
        src = path.read_text(encoding='utf-8', errors='ignore').splitlines()
    except Exception:
        return False, "read_error"

    changed = False
    out_lines = []

    for idx, line in enumerate(src):
        # 1) 去掉 Markdown 围栏
        if FENCE_LINE_RE.match(line):
            changed = True
            continue

        # 2) 将中部 #![...] 转换为 #[...]
        m = INNER_ATTR_RE.match(line)
        if m:
            # 若是文件前两行，尽量保留（减少过度改动）
            if idx <= 1:
                out_lines.append(line)
            else:
                changed = True
                out_lines.append(OUTER_ATTR_FMT.format(m.group(1)))
            continue

        # 3) 行尾多余 '>'（非常保守的修正，仅当行以单独一个 '>' 结尾）
        if line.rstrip().endswith('>') and not line.rstrip().endswith('>>'):
            # 仅在发现形如 '...DataSyncManager>>' 的上一行修正，此处不盲改
            out_lines.append(line)
        else:
            out_lines.append(line)

    if changed:
        try:
            path.write_text('\n'.join(out_lines) + '\n', encoding='utf-8')
        except Exception:
            return False, "write_error"
        return True, "changed"
    return True, "unchanged"

def main():
    if len(sys.argv) < 2:
        print("用法: python sanitize_skeleton.py <skeleton_project_dir>")
        sys.exit(1)
    root = Path(sys.argv[1])
    src_dir = root / "src"
    if not src_dir.exists():
        print(f"未找到 src 目录: {src_dir}")
        sys.exit(0)

    total = 0
    changed = 0
    errors = 0
    for p in src_dir.rglob("*.rs"):
        ok, status = process_file(p)
        total += 1
        if not ok:
            errors += 1
        elif status == "changed":
            changed += 1
    print(f"Sanitize 完成: 总计 {total}, 修改 {changed}, 错误 {errors}")

if __name__ == "__main__":
    main()


