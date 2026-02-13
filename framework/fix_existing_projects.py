#!/usr/bin/env python3
"""
修复已生成项目中的编译级联错误

使用方法:
    python fix_existing_projects.py <run_dir>

例如:
    python fix_existing_projects.py translation_outputs/Our/OurSmoke
"""

import re
import sys
import logging
from pathlib import Path
from typing import Set, List, Tuple

logging.basicConfig(level=logging.INFO, format='%(levelname)s: %(message)s')
logger = logging.getLogger(__name__)


def extract_globals_names(globals_path: Path) -> Set[str]:
    """从 globals.rs 中提取所有 pub static (mut) 变量名"""
    if not globals_path.exists():
        return set()
    try:
        text = globals_path.read_text(encoding="utf-8", errors="ignore")
        names = set(re.findall(r"(?m)^\s*pub\s+static(?:\s+mut)?\s+([A-Za-z_]\w*)\s*:", text))
        return names
    except Exception:
        return set()


def fix_fallback_visibility(project_dir: Path) -> int:
    """修复 fallback 模块中函数的可见性问题 (E0603)"""
    fallback_dir = project_dir / "src" / "__c2r_generated" / "c2rust_fallback"
    if not fallback_dir.exists():
        return 0

    fixed_count = 0
    for rs_file in fallback_dir.glob("*.rs"):
        if rs_file.name == "mod.rs":
            continue
        try:
            content = rs_file.read_text(encoding="utf-8", errors="ignore")
            original = content

            # 修复函数可见性
            content = re.sub(r'(?m)^(\s*)unsafe\s+extern\s+"C"\s+fn\s+', r'\1pub unsafe extern "C" fn ', content)
            content = re.sub(r'(?m)^(\s*)extern\s+"C"\s+fn\s+', r'\1pub extern "C" fn ', content)
            content = re.sub(r'(?m)^(\s*)unsafe\s+fn\s+([A-Za-z_])', r'\1pub unsafe fn \2', content)
            content = re.sub(r'(?m)^fn\s+([A-Za-z_])', r'pub fn \1', content)
            content = re.sub(r'pub\s+pub\s+', 'pub ', content)

            if content != original:
                rs_file.write_text(content, encoding="utf-8")
                fixed_count += 1
                logger.info(f"  修复函数可见性: {rs_file.name}")
        except Exception as e:
            logger.warning(f"  处理 {rs_file} 失败: {e}")

    return fixed_count


def remove_duplicate_statics(rs_path: Path, globals_names: Set[str]) -> int:
    """从 rs 文件中移除与 globals.rs 重复的 extern static 声明"""
    if not rs_path.exists() or not globals_names:
        return 0

    try:
        text = rs_path.read_text(encoding="utf-8", errors="ignore")
        original = text
        removed = 0

        lines = text.splitlines()
        out_lines: List[str] = []
        in_extern_block = False
        extern_block_depth = 0
        skip_until_semicolon = False

        for line in lines:
            stripped = line.strip()

            if 'extern "C"' in stripped and '{' in stripped:
                in_extern_block = True
                extern_block_depth = stripped.count('{') - stripped.count('}')

                # 检查是否是单行 extern "C" 块
                var_match = re.search(r'pub\s+static(?:\s+mut)?\s+([A-Za-z_]\w*)\s*:', stripped)
                if var_match and var_match.group(1) in globals_names and '}' in stripped:
                    removed += 1
                    in_extern_block = False
                    continue

                out_lines.append(line)
                continue

            if in_extern_block:
                extern_block_depth += stripped.count('{') - stripped.count('}')

                if extern_block_depth <= 0:
                    in_extern_block = False
                    out_lines.append(line)
                    continue

                var_match = re.match(r'\s*(?:pub\s+)?static(?:\s+mut)?\s+([A-Za-z_]\w*)\s*:', stripped)
                if var_match and var_match.group(1) in globals_names:
                    removed += 1
                    if ';' not in line:
                        skip_until_semicolon = True
                    continue

                if skip_until_semicolon:
                    if ';' in line:
                        skip_until_semicolon = False
                    continue

            out_lines.append(line)

        new_text = '\n'.join(out_lines)
        if original.endswith('\n') and not new_text.endswith('\n'):
            new_text += '\n'

        if new_text != original and removed > 0:
            rs_path.write_text(new_text, encoding="utf-8")

        return removed
    except Exception as e:
        logger.warning(f"处理 {rs_path} 失败: {e}")
        return 0


def fix_project(project_dir: Path) -> dict:
    """对单个项目执行所有修复"""
    results = {"fallback_fixed": 0, "types_dedup": 0, "compat_dedup": 0}

    src_dir = project_dir / "src"
    if not src_dir.exists():
        return results

    logger.info(f"\n修复项目: {project_dir.name}")

    # 修复 1: fallback 函数可见性
    results["fallback_fixed"] = fix_fallback_visibility(project_dir)
    if results["fallback_fixed"]:
        logger.info(f"  修复了 {results['fallback_fixed']} 个 fallback 文件")

    # 修复 2: 移除重复静态变量
    globals_names = extract_globals_names(src_dir / "globals.rs")
    if globals_names:
        results["types_dedup"] = remove_duplicate_statics(src_dir / "types.rs", globals_names)
        results["compat_dedup"] = remove_duplicate_statics(src_dir / "compat.rs", globals_names)
        if results["types_dedup"]:
            logger.info(f"  types.rs 移除 {results['types_dedup']} 个重复定义")
        if results["compat_dedup"]:
            logger.info(f"  compat.rs 移除 {results['compat_dedup']} 个重复定义")

    return results


def fix_run_dir(run_dir: Path) -> None:
    """修复运行目录下的所有项目"""
    intermediate_dir = run_dir / "intermediate"
    if not intermediate_dir.exists():
        intermediate_dir = run_dir
        if not (intermediate_dir / "workspace").exists() and not list(intermediate_dir.glob("*/workspace")):
            logger.error(f"找不到项目目录: {run_dir}")
            return

    total_fixed = {"fallback": 0, "types": 0, "compat": 0}

    for project_dir in intermediate_dir.iterdir():
        if not project_dir.is_dir():
            continue

        final_projects = project_dir / "workspace" / "final_projects"
        if not final_projects.exists():
            continue

        for proj in final_projects.iterdir():
            if not proj.is_dir():
                continue

            for llm_dir in proj.iterdir():
                if llm_dir.is_dir() and llm_dir.name.startswith("translate_by_"):
                    results = fix_project(llm_dir)
                    total_fixed["fallback"] += results["fallback_fixed"]
                    total_fixed["types"] += results["types_dedup"]
                    total_fixed["compat"] += results["compat_dedup"]

    logger.info(f"\n{'='*60}")
    logger.info(f"修复完成!")
    logger.info(f"  fallback 文件: {total_fixed['fallback']}")
    logger.info(f"  types.rs 去重: {total_fixed['types']}")
    logger.info(f"  compat.rs 去重: {total_fixed['compat']}")
    logger.info(f"{'='*60}")


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print(__doc__)
        sys.exit(1)

    target = Path(sys.argv[1]).resolve()

    if (target / "src").exists():
        fix_project(target)
    else:
        fix_run_dir(target)
