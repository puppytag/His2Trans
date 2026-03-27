from __future__ import annotations

import argparse
import json
import re
from bisect import bisect_right
from collections import Counter, defaultdict
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Dict, Iterable, Iterator, Mapping, Optional, Sequence


THIS_FILE = Path(__file__).resolve()
HIS2TRANS_ROOT = THIS_FILE.parents[2]
WORKSPACE_ROOT = HIS2TRANS_ROOT.parent
ASE_ROOT = WORKSPACE_ROOT / "ASE"
C2R_LATEST_ROOT = WORKSPACE_ROOT / "c2-rust_framework" / "paper_tables_postprocess" / "latest"
STRUCTURED_JSON_ROOT = HIS2TRANS_ROOT / "data" / "paper_metric_exports" / "generated_structured_json"
DEFAULT_REPORT_PATH = ASE_ROOT / "reviewer_warning_unsafe_taxonomy.md"

RQ2_INCLUDED_PROJECTS = (
    "ht",
    "qsort",
    "quadtree",
    "buffer",
    "rgba",
    "urlparser",
    "genann",
    "avl",
    "zopfli",
)

WARNING_CATEGORY_ORDER = (
    "ffi_unsafe_idiom",
    "suspicious_cast_pointer",
    "unused_dead_code",
    "naming_style",
    "other",
)

WARNING_CATEGORY_LABELS = {
    "ffi_unsafe_idiom": "FFI / Unsafe / Idiom",
    "suspicious_cast_pointer": "Suspicious Cast / Pointer",
    "unused_dead_code": "Unused / Dead Code",
    "naming_style": "Naming / Style",
    "other": "Other",
}

UNSAFE_CATEGORY_ORDER = (
    "ffi_boundary",
    "raw_pointer_traversal",
    "layout_preserving_casts",
    "manual_memory_ops",
    "other",
)

UNSAFE_CATEGORY_LABELS = {
    "ffi_boundary": "FFI boundary",
    "raw_pointer_traversal": "Raw pointer traversal",
    "layout_preserving_casts": "Layout-preserving casts",
    "manual_memory_ops": "Manual memory ops",
    "other": "Other",
}

WARNING_EXACT_CATEGORY = {
    "unused_assignments": "unused_dead_code",
    "unused_variables": "unused_dead_code",
    "unused_imports": "unused_dead_code",
    "unused_mut": "unused_dead_code",
    "unused_unit": "unused_dead_code",
    "dead_code": "unused_dead_code",
    "unreachable_code": "unused_dead_code",
    "clippy::needless_return": "naming_style",
    "clippy::let_and_return": "naming_style",
    "clippy::collapsible_if": "naming_style",
    "clippy::manual_range_contains": "naming_style",
    "clippy::upper_case_acronyms": "naming_style",
    "clippy::single_match": "naming_style",
    "clippy::assign_op_pattern": "naming_style",
    "clippy::bool_comparison": "naming_style",
    "clippy::needless_range_loop": "naming_style",
    "clippy::needless_late_init": "naming_style",
    "clippy::precedence": "naming_style",
    "clippy::nonminimal_bool": "naming_style",
    "clippy::explicit_auto_deref": "naming_style",
    "clippy::too_many_arguments": "naming_style",
    "clippy::unnecessary_mut_passed": "naming_style",
    "clippy::ptr_eq": "ffi_unsafe_idiom",
    "clippy::cmp_null": "ffi_unsafe_idiom",
    "clippy::manual_c_str_literals": "ffi_unsafe_idiom",
    "clippy::not_unsafe_ptr_arg_deref": "ffi_unsafe_idiom",
    "clippy::missing_safety_doc": "ffi_unsafe_idiom",
    "clippy::toplevel_ref_arg": "ffi_unsafe_idiom",
    "clippy::zero_ptr": "suspicious_cast_pointer",
    "clippy::ptr_offset_with_cast": "suspicious_cast_pointer",
    "clippy::unnecessary_cast": "suspicious_cast_pointer",
    "clippy::useless_transmute": "suspicious_cast_pointer",
    "clippy::missing_transmute_annotations": "suspicious_cast_pointer",
    "clippy::unnecessary_operation": "suspicious_cast_pointer",
}

UNSAFE_PRIMARY_ORDER = (
    "manual_memory_ops",
    "raw_pointer_traversal",
    "layout_preserving_casts",
    "ffi_boundary",
    "other",
)

RAW_POINTER_PATTERNS = (
    re.compile(r"\(\s*\*"),
    re.compile(r"\.offset\s*\("),
    re.compile(r"\.add\s*\("),
    re.compile(r"\.sub\s*\("),
    re.compile(r"\.wrapping_offset\s*\("),
    re.compile(r"\.read(_unaligned)?\s*\("),
    re.compile(r"\.write(_unaligned)?\s*\("),
)

LAYOUT_CAST_PATTERNS = (
    re.compile(r"\bas\s+\*mut\b"),
    re.compile(r"\bas\s+\*const\b"),
    re.compile(r"\btransmute\b"),
    re.compile(r"\bfrom_raw_parts(_mut)?\b"),
    re.compile(r"\bcast::<"),
)

MANUAL_MEMORY_PATTERNS = (
    re.compile(r"\bmalloc\b"),
    re.compile(r"\bcalloc\b"),
    re.compile(r"\brealloc\b"),
    re.compile(r"\bfree\b"),
    re.compile(r"\bmemcpy\b"),
    re.compile(r"\bmemmove\b"),
    re.compile(r"\bmemset\b"),
    re.compile(r"\bcopy_nonoverlapping\b"),
    re.compile(r"\bwrite_bytes\b"),
    re.compile(r"\bstd::alloc::"),
    re.compile(r"\balloc::alloc\b"),
    re.compile(r"\balloc::dealloc\b"),
    re.compile(r"\bBox::from_raw\b"),
    re.compile(r"\bVec::from_raw_parts\b"),
)

FFI_PATTERNS = (
    re.compile(r'extern\s+"C"'),
    re.compile(r"\blibc::"),
    re.compile(r"\bcore::ffi::"),
    re.compile(r"\bstd::ffi::"),
    re.compile(r"\bCStr\b"),
    re.compile(r"\bCString\b"),
    re.compile(r"\bc_char\b"),
    re.compile(r"\bc_void\b"),
    re.compile(r"\bfrom_raw\b"),
    re.compile(r"\binto_raw\b"),
)


@dataclass(frozen=True)
class DatasetSpec:
    method: str
    rq: str
    kind: str
    path: Path


@dataclass(frozen=True)
class UnsafeSourceSpec:
    method: str
    rq: str
    project_dirs: tuple[Path, ...]


@dataclass
class WarningSummary:
    method: str
    rq: str
    total_warnings: int
    category_counts: Dict[str, int]
    top_codes: list[tuple[str, int]]


@dataclass
class UnsafeSite:
    file_path: Path
    line_no: int
    kind: str
    text: str
    tags: tuple[str, ...]


@dataclass
class UnsafeSummary:
    method: str
    rq: str
    total_sites: int
    tagged_counts: Dict[str, int]
    primary_counts: Dict[str, int]
    examples: Dict[str, list[UnsafeSite]]


def warning_dataset_specs() -> tuple[DatasetSpec, ...]:
    return (
        DatasetSpec(
            method="His2Trans (Claude, RQ1)",
            rq="rq1",
            kind="structured",
            path=STRUCTURED_JSON_ROOT / "rq1_claude.json",
        ),
        DatasetSpec(
            method="His2Trans (DeepSeek K=5, RQ1)",
            rq="rq1",
            kind="structured",
            path=STRUCTURED_JSON_ROOT / "rq1_k5.json",
        ),
        DatasetSpec(
            method="C2Rust",
            rq="rq2",
            kind="project_json_dir",
            path=C2R_LATEST_ROOT / "rerun_unified" / "c2rust_projects",
        ),
        DatasetSpec(
            method="C2SaferRust",
            rq="rq2",
            kind="baseline",
            path=C2R_LATEST_ROOT / "rerun_unified" / "raw" / "c2saferrust_rq2_compilation_analysis.json",
        ),
        DatasetSpec(
            method="His2Trans (DeepSeek, RQ2)",
            rq="rq2",
            kind="structured",
            path=STRUCTURED_JSON_ROOT / "rq2_deepseek.json",
        ),
        DatasetSpec(
            method="His2Trans (Claude, RQ2)",
            rq="rq2",
            kind="structured",
            path=STRUCTURED_JSON_ROOT / "rq2_claude.json",
        ),
    )


def unsafe_dataset_specs() -> tuple[UnsafeSourceSpec, ...]:
    rq1_claude = json.loads((STRUCTURED_JSON_ROOT / "rq1_claude.json").read_text(encoding="utf-8"))
    rq2_deepseek = json.loads((STRUCTURED_JSON_ROOT / "rq2_deepseek.json").read_text(encoding="utf-8"))
    rq2_claude = json.loads((STRUCTURED_JSON_ROOT / "rq2_claude.json").read_text(encoding="utf-8"))

    rq1_project_dirs = tuple(
        Path(project["project_dir"])
        for project in rq1_claude["projects"].values()
        if isinstance(project, Mapping) and project.get("project_dir")
    )

    def his2trans_rq2_dirs(report: Mapping[str, Any]) -> tuple[Path, ...]:
        run_dir = Path(str(report["run_dir"]))
        project_dirs = []
        for name in RQ2_INCLUDED_PROJECTS:
            project_dir = (
                run_dir / "intermediate" / name / "workspace" / "final_projects" / name / "translate_by_qwen3_coder"
            )
            project_dirs.append(project_dir)
        return tuple(project_dirs)

    return (
        UnsafeSourceSpec(
            method="His2Trans (Claude, RQ1)",
            rq="rq1",
            project_dirs=rq1_project_dirs,
        ),
        UnsafeSourceSpec(
            method="C2Rust",
            rq="rq2",
            project_dirs=tuple(
                C2R_LATEST_ROOT / "inputs" / "source_rq2_tests" / name / "c2rust"
                for name in RQ2_INCLUDED_PROJECTS
            ),
        ),
        UnsafeSourceSpec(
            method="His2Trans (DeepSeek, RQ2)",
            rq="rq2",
            project_dirs=his2trans_rq2_dirs(rq2_deepseek),
        ),
        UnsafeSourceSpec(
            method="His2Trans (Claude, RQ2)",
            rq="rq2",
            project_dirs=his2trans_rq2_dirs(rq2_claude),
        ),
    )


def _project_items(report: Mapping[str, Any]) -> Iterable[tuple[str, Mapping[str, Any]]]:
    projects = report.get("projects")
    if isinstance(projects, Mapping):
        return projects.items()
    return ()


def _iter_project_warning_objs(project: Mapping[str, Any]) -> Iterator[Mapping[str, Any]]:
    for key in ("cargo_clippy_result", "clippy", "clippy_results"):
        obj = project.get(key)
        if isinstance(obj, Mapping):
            yield obj


def _parse_warning_counter_from_output(output: str) -> Counter[str]:
    counter: Counter[str] = Counter()
    for raw_line in output.splitlines():
        line = raw_line.strip()
        if not line or not line.startswith("{"):
            continue
        try:
            payload = json.loads(line)
        except json.JSONDecodeError:
            continue
        if payload.get("reason") != "compiler-message":
            continue
        message = payload.get("message")
        if not isinstance(message, Mapping):
            continue
        if message.get("level") != "warning":
            continue
        code_obj = message.get("code")
        code = None
        if isinstance(code_obj, Mapping):
            code = code_obj.get("code")
        if not code:
            code = "rustc::uncoded_warning"
        counter[str(code)] += 1
    return counter


def extract_warning_codes(project: Mapping[str, Any]) -> Counter[str]:
    merged: Counter[str] = Counter()
    recorded_total = 0
    for warning_obj in _iter_project_warning_objs(project):
        if warning_obj.get("warning_count_total") is not None:
            recorded_total += int(warning_obj.get("warning_count_total") or 0)
        else:
            recorded_total += int(warning_obj.get("warning_count") or 0) + int(
                warning_obj.get("rustc_warning_count") or 0
            )
        codes = warning_obj.get("warnings_by_type")
        saw_structured_clippy = False
        if isinstance(codes, Mapping):
            merged.update({str(k): int(v) for k, v in codes.items()})
            saw_structured_clippy = True
        rustc_codes = warning_obj.get("rustc_warning_codes")
        if isinstance(rustc_codes, Mapping):
            merged.update({str(k): int(v) for k, v in rustc_codes.items()})
        output = warning_obj.get("output")
        if isinstance(output, str) and output.strip():
            parsed = _parse_warning_counter_from_output(output)
            for code, count in parsed.items():
                if not saw_structured_clippy or not str(code).startswith("clippy::"):
                    merged[str(code)] += int(count)
    current_total = sum(merged.values())
    if recorded_total > current_total:
        merged["rustc::uncoded_warning"] += recorded_total - current_total
    return merged


def categorize_warning_code(code: str) -> str:
    if code in WARNING_EXACT_CATEGORY:
        return WARNING_EXACT_CATEGORY[code]
    normalized = code.removeprefix("clippy::")
    if normalized.startswith("unused_") or normalized == "dead_code":
        return "unused_dead_code"
    if normalized.startswith("improper_ctypes"):
        return "ffi_unsafe_idiom"
    if normalized.startswith("missing_safety_doc"):
        return "ffi_unsafe_idiom"
    if normalized.startswith("not_unsafe_ptr_arg_deref"):
        return "ffi_unsafe_idiom"
    if normalized.endswith("transmute") or "cast" in normalized or "ptr" in normalized:
        return "suspicious_cast_pointer"
    if normalized.startswith("needless_") or normalized in {
        "collapsible_if",
        "single_match",
        "assign_op_pattern",
        "manual_range_contains",
        "precedence",
        "nonminimal_bool",
        "upper_case_acronyms",
        "too_many_arguments",
        "explicit_auto_deref",
    }:
        return "naming_style"
    return "other"


def summarize_warning_taxonomy(specs: Sequence[DatasetSpec]) -> list[WarningSummary]:
    summaries: list[WarningSummary] = []
    for spec in specs:
        code_counter: Counter[str] = Counter()
        if spec.kind == "project_json_dir":
            for json_path in sorted(spec.path.glob("*.json")):
                report = json.loads(json_path.read_text(encoding="utf-8"))
                for name, project in _project_items(report):
                    if spec.rq == "rq2" and name not in RQ2_INCLUDED_PROJECTS:
                        continue
                    code_counter.update(extract_warning_codes(project))
        else:
            report = json.loads(spec.path.read_text(encoding="utf-8"))
            for name, project in _project_items(report):
                if spec.rq == "rq2" and name not in RQ2_INCLUDED_PROJECTS:
                    continue
                code_counter.update(extract_warning_codes(project))
        category_counts = {name: 0 for name in WARNING_CATEGORY_ORDER}
        for code, count in code_counter.items():
            category_counts[categorize_warning_code(code)] += count
        summaries.append(
            WarningSummary(
                method=spec.method,
                rq=spec.rq,
                total_warnings=sum(code_counter.values()),
                category_counts=category_counts,
                top_codes=code_counter.most_common(10),
            )
        )
    return summaries


def _is_ident_char(ch: str) -> bool:
    return ch.isalnum() or ch == "_"


def _skip_line_comment(s: str, i: int) -> int:
    j = s.find("\n", i)
    return len(s) if j < 0 else j + 1


def _skip_block_comment(s: str, i: int) -> int:
    depth = 1
    j = i + 2
    while j < len(s):
        if s.startswith("/*", j):
            depth += 1
            j += 2
            continue
        if s.startswith("*/", j):
            depth -= 1
            j += 2
            if depth == 0:
                return j
            continue
        j += 1
    return len(s)


def _skip_normal_string(s: str, i: int) -> int:
    quote = s[i]
    j = i + 1
    while j < len(s):
        if s[j] == "\\":
            j += 2
            continue
        if s[j] == quote:
            return j + 1
        j += 1
    return len(s)


def _try_skip_raw_string(s: str, i: int) -> Optional[int]:
    if s[i] != "r":
        return None
    j = i + 1
    hash_count = 0
    while j < len(s) and s[j] == "#":
        hash_count += 1
        j += 1
    if j >= len(s) or s[j] != '"':
        return None
    terminator = '"' + ("#" * hash_count)
    end = s.find(terminator, j + 1)
    if end < 0:
        return len(s)
    return end + len(terminator)


def _skip_ws_and_comments(s: str, i: int) -> int:
    n = len(s)
    while i < n:
        if s.startswith("//", i):
            i = _skip_line_comment(s, i)
            continue
        if s.startswith("/*", i):
            i = _skip_block_comment(s, i)
            continue
        if i < n and s[i].isspace():
            i += 1
            continue
        return i
    return i


def _find_matching_brace(s: str, brace_idx: int) -> Optional[int]:
    if brace_idx >= len(s) or s[brace_idx] != "{":
        return None
    depth = 1
    i = brace_idx + 1
    while i < len(s):
        if s.startswith("//", i):
            i = _skip_line_comment(s, i)
            continue
        if s.startswith("/*", i):
            i = _skip_block_comment(s, i)
            continue
        raw_end = _try_skip_raw_string(s, i)
        if raw_end is not None:
            i = raw_end
            continue
        if s[i] in {'"', "'"}:
            i = _skip_normal_string(s, i)
            continue
        if s[i] == "{":
            depth += 1
        elif s[i] == "}":
            depth -= 1
            if depth == 0:
                return i + 1
        i += 1
    return None


def _find_body_brace_or_decl_end(s: str, i: int) -> Optional[int]:
    while i < len(s):
        if s.startswith("//", i):
            i = _skip_line_comment(s, i)
            continue
        if s.startswith("/*", i):
            i = _skip_block_comment(s, i)
            continue
        raw_end = _try_skip_raw_string(s, i)
        if raw_end is not None:
            i = raw_end
            continue
        if s[i] in {'"', "'"}:
            i = _skip_normal_string(s, i)
            continue
        if s[i] == "{":
            return i
        if s[i] == ";":
            return None
        i += 1
    return None


def _index_to_line(line_starts: Sequence[int], idx: int) -> int:
    return bisect_right(line_starts, idx) - 1


def _line_starts(content: str) -> list[int]:
    starts = [0]
    for idx, ch in enumerate(content):
        if ch == "\n":
            starts.append(idx + 1)
    return starts


def _list_rust_files(project_dir: Path) -> list[Path]:
    exclude_parts = {"target", ".git", ".c2r_c2rust_fallback", "c2rust_fallback"}
    files = []
    for path in project_dir.rglob("*.rs"):
        if exclude_parts.intersection(path.parts):
            continue
        if any(part.startswith(".c2r_bindgen_") for part in path.parts):
            continue
        if "tests" in path.parts or path.name in {"test.rs", "test_c2r.rs"} or path.stem.startswith("test_"):
            continue
        files.append(path)
    return sorted(files)


def find_unsafe_sites_in_text(content: str, file_path: Path) -> list[UnsafeSite]:
    starts = _line_starts(content)
    sites: list[UnsafeSite] = []
    i = 0
    n = len(content)

    while i < n:
        if content.startswith("//", i):
            i = _skip_line_comment(content, i)
            continue
        if content.startswith("/*", i):
            i = _skip_block_comment(content, i)
            continue
        raw_end = _try_skip_raw_string(content, i)
        if raw_end is not None:
            i = raw_end
            continue
        if content[i] in {'"', "'"}:
            i = _skip_normal_string(content, i)
            continue
        if content.startswith("unsafe", i):
            before_ok = i == 0 or not _is_ident_char(content[i - 1])
            after = i + 6
            after_ok = after >= n or not _is_ident_char(content[after])
            if before_ok and after_ok:
                j = _skip_ws_and_comments(content, after)
                kind = "unsafe_keyword"
                body_start = None
                if j < n and content[j] == "{":
                    kind = "unsafe_block"
                    body_start = j
                elif content.startswith("fn", j) and (j + 2 >= n or not _is_ident_char(content[j + 2])):
                    kind = "unsafe_fn"
                    body_start = _find_body_brace_or_decl_end(content, j + 2)
                elif content.startswith("extern", j) and (j + 6 >= n or not _is_ident_char(content[j + 6])):
                    kind = "unsafe_extern"
                    body_start = _find_body_brace_or_decl_end(content, j + 6)
                elif content.startswith("impl", j) and (j + 4 >= n or not _is_ident_char(content[j + 4])):
                    kind = "unsafe_impl"
                    body_start = _find_body_brace_or_decl_end(content, j + 4)
                elif content.startswith("trait", j) and (j + 5 >= n or not _is_ident_char(content[j + 5])):
                    kind = "unsafe_trait"
                    body_start = _find_body_brace_or_decl_end(content, j + 5)
                if body_start is not None:
                    body_end = _find_matching_brace(content, body_start)
                    if body_end is not None:
                        line_no = _index_to_line(starts, i) + 1
                        raw_site_text = content[i:body_end]
                        sites.append(
                            UnsafeSite(
                                file_path=file_path,
                                line_no=line_no,
                                kind=kind,
                                text=raw_site_text,
                                tags=(),
                            )
                        )
                        i = body_end
                        continue
        i += 1
    return sites


def classify_unsafe_tags(site: UnsafeSite) -> tuple[str, ...]:
    text = site.text
    tags: list[str] = []
    if site.kind in {"unsafe_extern", "unsafe_fn"} and any(marker in text for marker in ('extern "C"', "core::ffi::", "std::ffi::")):
        tags.append("ffi_boundary")
    if any(pattern.search(text) for pattern in FFI_PATTERNS):
        tags.append("ffi_boundary")
    if any(pattern.search(text) for pattern in RAW_POINTER_PATTERNS):
        tags.append("raw_pointer_traversal")
    if any(pattern.search(text) for pattern in LAYOUT_CAST_PATTERNS):
        tags.append("layout_preserving_casts")
    if any(pattern.search(text) for pattern in MANUAL_MEMORY_PATTERNS):
        tags.append("manual_memory_ops")
    if not tags:
        tags.append("other")
    ordered = [name for name in UNSAFE_CATEGORY_ORDER if name in tags]
    return tuple(ordered)


def pick_primary_unsafe_category(tags: Sequence[str]) -> str:
    for name in UNSAFE_PRIMARY_ORDER:
        if name in tags:
            return name
    return "other"


def summarize_unsafe_taxonomy(specs: Sequence[UnsafeSourceSpec]) -> list[UnsafeSummary]:
    summaries: list[UnsafeSummary] = []
    for spec in specs:
        tagged_counts = {name: 0 for name in UNSAFE_CATEGORY_ORDER}
        primary_counts = {name: 0 for name in UNSAFE_CATEGORY_ORDER}
        examples: Dict[str, list[UnsafeSite]] = defaultdict(list)
        total_sites = 0
        for project_dir in spec.project_dirs:
            for rs_file in _list_rust_files(project_dir):
                content = rs_file.read_text(encoding="utf-8", errors="ignore")
                for site in find_unsafe_sites_in_text(content, rs_file):
                    tags = classify_unsafe_tags(site)
                    site.tags = tags
                    total_sites += 1
                    for tag in tags:
                        tagged_counts[tag] += 1
                        if len(examples[tag]) < 3:
                            examples[tag].append(site)
                    primary_counts[pick_primary_unsafe_category(tags)] += 1
        summaries.append(
            UnsafeSummary(
                method=spec.method,
                rq=spec.rq,
                total_sites=total_sites,
                tagged_counts=tagged_counts,
                primary_counts=primary_counts,
                examples=dict(examples),
            )
        )
    return summaries


def _pct(count: int, total: int) -> str:
    if total <= 0:
        return "0.00"
    return f"{(count / total) * 100.0:.2f}"


def _render_warning_table(summaries: Sequence[WarningSummary]) -> str:
    lines = [
        "| Method | Total | FFI / Unsafe / Idiom | Suspicious Cast / Pointer | Unused / Dead Code | Naming / Style | Other |",
        "| --- | ---: | ---: | ---: | ---: | ---: | ---: |",
    ]
    for summary in summaries:
        row = [summary.method, str(summary.total_warnings)]
        for category in WARNING_CATEGORY_ORDER:
            count = summary.category_counts.get(category, 0)
            row.append(f"{count} ({_pct(count, summary.total_warnings)}%)")
        lines.append("| " + " | ".join(row) + " |")
    return "\n".join(lines)


def _render_warning_top_codes(summaries: Sequence[WarningSummary]) -> str:
    lines: list[str] = []
    for summary in summaries:
        lines.append(f"### {summary.method}")
        for code, count in summary.top_codes[:8]:
            lines.append(f"- `{code}`: {count}")
        lines.append("")
    return "\n".join(lines).rstrip()


def _render_unsafe_table(summaries: Sequence[UnsafeSummary]) -> str:
    lines = [
        "| Method | Unsafe Sites | FFI boundary | Raw pointer traversal | Layout-preserving casts | Manual memory ops | Other |",
        "| --- | ---: | ---: | ---: | ---: | ---: | ---: |",
    ]
    for summary in summaries:
        row = [summary.method, str(summary.total_sites)]
        for category in UNSAFE_CATEGORY_ORDER:
            count = summary.tagged_counts.get(category, 0)
            row.append(f"{count} ({_pct(count, summary.total_sites)}%)")
        lines.append("| " + " | ".join(row) + " |")
    return "\n".join(lines)


def _render_unsafe_examples(summary: UnsafeSummary) -> str:
    lines = [f"### {summary.method}"]
    for category in UNSAFE_CATEGORY_ORDER:
        sites = summary.examples.get(category) or []
        if not sites:
            continue
        lines.append(f"- `{UNSAFE_CATEGORY_LABELS[category]}` examples:")
        for site in sites[:2]:
            first_line = next((line.strip() for line in site.text.splitlines() if line.strip()), "").strip()
            lines.append(
                f"  - `{site.file_path.name}:{site.line_no}` `{site.kind}` -> `{first_line[:100]}`"
            )
    return "\n".join(lines)


def build_markdown_report(
    warning_summaries: Sequence[WarningSummary],
    unsafe_summaries: Sequence[UnsafeSummary],
) -> str:
    rq1_warning = [summary for summary in warning_summaries if summary.rq == "rq1"]
    rq2_warning = [summary for summary in warning_summaries if summary.rq == "rq2"]
    rq1_unsafe = [summary for summary in unsafe_summaries if summary.rq == "rq1"]
    rq2_unsafe = [summary for summary in unsafe_summaries if summary.rq == "rq2"]

    lines = [
        "# Reviewer Notes: Warning / Unsafe Taxonomy",
        "",
        "口径说明：",
        "- Warning taxonomy 按当前 Clippy / rustc warning code 聚合，不重跑实验。",
        "- Unsafe taxonomy 按最终 Rust crate 中的 `unsafe block` / `unsafe fn` 做静态标注统计。",
        "- Unsafe taxonomy 为 tagged-site 统计，同一个 unsafe site 可以同时命中多个类别，因此各列百分比之和可能超过 100%。",
        "- 当前 artifact bundle 未保留 C2SaferRust 的最终源码，只保留了评测 `test.rs`，因此 unsafe taxonomy 中不纳入 C2SaferRust。",
        "",
        "## RQ1 Warning Taxonomy",
        "",
        _render_warning_table(rq1_warning),
        "",
        "## RQ1 Warning Top Codes",
        "",
        _render_warning_top_codes(rq1_warning),
        "",
        "## RQ2 Warning Taxonomy",
        "",
        _render_warning_table(rq2_warning),
        "",
        "## RQ2 Warning Top Codes",
        "",
        _render_warning_top_codes(rq2_warning),
        "",
        "## RQ1 Unsafe Taxonomy",
        "",
        _render_unsafe_table(rq1_unsafe),
        "",
        "## RQ2 Unsafe Taxonomy",
        "",
        _render_unsafe_table(rq2_unsafe),
        "",
        "## Unsafe Examples",
        "",
    ]
    for summary in rq1_unsafe + rq2_unsafe:
        lines.append(_render_unsafe_examples(summary))
        lines.append("")
    return "\n".join(lines).rstrip() + "\n"


def generate_default_report(report_path: Path = DEFAULT_REPORT_PATH) -> Path:
    warning_summaries = summarize_warning_taxonomy(warning_dataset_specs())
    unsafe_summaries = summarize_unsafe_taxonomy(unsafe_dataset_specs())
    report_path.parent.mkdir(parents=True, exist_ok=True)
    report_path.write_text(build_markdown_report(warning_summaries, unsafe_summaries), encoding="utf-8")
    return report_path


def main() -> None:
    parser = argparse.ArgumentParser(description="Generate warning/unsafe taxonomy report for RQ1/RQ2.")
    parser.add_argument(
        "--output",
        type=Path,
        default=DEFAULT_REPORT_PATH,
        help="Markdown report output path.",
    )
    args = parser.parse_args()
    path = generate_default_report(args.output)
    print(path)


if __name__ == "__main__":
    main()
