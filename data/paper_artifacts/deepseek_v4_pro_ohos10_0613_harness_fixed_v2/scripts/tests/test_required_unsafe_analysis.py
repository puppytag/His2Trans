"""测试必须 unsafe 行统计脚本的保守分类口径。"""

from pathlib import Path

import analyze_required_unsafe as aru


def _write_crate(tmp_path: Path, source: str) -> Path:
    """创建最小 Rust crate 目录。"""
    crate = tmp_path / "demo"
    src = crate / "src"
    src.mkdir(parents=True)
    (crate / "Cargo.toml").write_text(
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
        encoding="utf-8",
    )
    (src / "lib.rs").write_text(source, encoding="utf-8")
    return crate


def test_unused_unsafe_block_is_not_required(tmp_path: Path) -> None:
    """只包住安全表达式的 unsafe block 不计入必须 unsafe。"""
    crate = _write_crate(
        tmp_path,
        """
pub extern "C" fn demo() -> i32 {
    unsafe {
        let value = 1 + 2;
        value
    }
}
""",
    )

    result = aru.analyze_crate(crate)

    assert result["required_unsafe_lines"] == 0
    assert result["required_reasons"] == {}


def test_raw_pointer_deref_is_required(tmp_path: Path) -> None:
    """raw pointer 解引用计入必须 unsafe，纯 unsafe block 结构行不计。"""
    crate = _write_crate(
        tmp_path,
        """
pub extern "C" fn read_value(ptr: *const i32) -> i32 {
    unsafe {
        (*ptr)
    }
}
""",
    )

    result = aru.analyze_crate(crate)

    assert result["required_unsafe_lines"] == 1
    assert result["required_reasons"]["raw_pointer_deref"] == 1
    assert "unsafe_block_required" not in result["required_reasons"]


def test_multiline_ffi_call_counts_call_anchor(tmp_path: Path) -> None:
    """多行 FFI 调用只计调用锚点行。"""
    crate = _write_crate(
        tmp_path,
        """
extern "C" {
    fn HiLogPrint(level: i32, tag: *const i8, fmt: *const i8) -> i32;
}

pub extern "C" fn log_it(tag: *const i8) -> i32 {
    unsafe {
        HiLogPrint(
            1,
            tag,
            b"hello\\0".as_ptr() as *const i8,
        )
    }
}
""",
    )

    result = aru.analyze_crate(crate)

    assert result["required_unsafe_lines"] == 1
    assert result["required_reasons"]["ffi_call"] == 1
    assert "unsafe_block_required" not in result["required_reasons"]


def test_static_mut_access_and_assume_init_are_required(tmp_path: Path) -> None:
    """static mut 访问和 FFI layout 初始化计入必须 unsafe。"""
    crate = _write_crate(
        tmp_path,
        """
use std::mem::MaybeUninit;

pub static mut G_VALUE: i32 = unsafe { MaybeUninit::<i32>::zeroed().assume_init() };

pub extern "C" fn get_value() -> i32 {
    unsafe {
        G_VALUE
    }
}
""",
    )

    result = aru.analyze_crate(crate)

    assert result["required_unsafe_lines"] == 2
    assert result["required_reasons"]["ffi_layout_init"] == 1
    assert result["required_reasons"]["static_mut_access"] == 1
    assert "unsafe_block_required" not in result["required_reasons"]


def test_non_ffi_transmute_is_not_counted(tmp_path: Path) -> None:
    """缺少 FFI/layout 证据的 transmute 不归入必须 unsafe。"""
    crate = _write_crate(
        tmp_path,
        """
pub fn convert(value: u32) -> i32 {
    unsafe {
        std::mem::transmute(value)
    }
}
""",
    )

    result = aru.analyze_crate(crate)

    assert result["required_unsafe_lines"] == 0


def test_comment_ffi_name_does_not_make_block_required(tmp_path: Path) -> None:
    """注释里的 FFI 函数名不能让安全 unsafe block 变成必须 unsafe。"""
    crate = _write_crate(
        tmp_path,
        """
extern "C" {
    fn cJSON_GetObjectItem(obj: *mut i32) -> *mut i32;
}

pub extern "C" fn demo() -> i32 {
    unsafe {
        // Placeholder: cJSON_GetObjectItem would be called by the original C.
        1 + 2
    }
}
""",
    )

    result = aru.analyze_crate(crate)

    assert result["required_unsafe_lines"] == 0
    assert result["required_reasons"] == {}


def test_string_static_mut_name_is_not_counted(tmp_path: Path) -> None:
    """字符串里的 static mut 名称不能被当成全局访问。"""
    crate = _write_crate(
        tmp_path,
        """
pub static mut G_VALUE: i32 = 0;

pub extern "C" fn demo() -> *const u8 {
    unsafe {
        b"G_VALUE\\0".as_ptr()
    }
}
""",
    )

    result = aru.analyze_crate(crate)

    assert result["required_unsafe_lines"] == 0
    assert result["required_reasons"] == {}


def test_unsafe_abi_contract_is_required(tmp_path: Path) -> None:
    """unsafe extern C ABI 合约本身计入必须 unsafe。"""
    crate = _write_crate(
        tmp_path,
        """
pub extern "C" fn call_cb(cb: unsafe extern "C" fn(*mut core::ffi::c_void), arg: *mut core::ffi::c_void) {
    unsafe {
        cb(arg);
    }
}
""",
    )

    result = aru.analyze_crate(crate)

    assert result["required_unsafe_lines"] == 2
    assert result["required_reasons"]["unsafe_abi_contract"] == 1
    assert result["required_reasons"]["unsafe_callback_call"] == 1
    assert "unsafe_block_required" not in result["required_reasons"]


def test_callback_type_does_not_expand_raw_unsafe_scope(tmp_path: Path) -> None:
    """函数指针类型里的 unsafe extern fn 不能吞掉后续普通代码。"""
    crate = _write_crate(
        tmp_path,
        """
pub struct Holder {
    pub cb: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    pub raw: *mut core::ffi::c_void,
}

pub fn safe_after() -> i32 {
    let value = 42;
    value
}
""",
    )

    result = aru.analyze_crate(crate)

    assert result["raw_unsafe_lines"] == 1
    assert result["required_unsafe_lines"] == 1
    assert result["required_reasons"]["unsafe_abi_contract"] == 1
    assert result["findings_sample"][0]["line"] == 3


def test_unsafe_extern_block_contract_is_required(tmp_path: Path) -> None:
    """unsafe extern block 是 ABI 合约，但纯结构括号行不计。"""
    crate = _write_crate(
        tmp_path,
        """
unsafe extern "C" {
    pub fn ffi_fn(p: *mut i32);
}
""",
    )

    result = aru.analyze_crate(crate)

    assert result["raw_unsafe_lines"] == 2
    assert result["required_unsafe_lines"] == 1
    assert result["required_reasons"]["unsafe_extern_block_contract"] == 1


def test_pointer_unsafe_fn_contract_is_required(tmp_path: Path) -> None:
    """带 raw pointer 合约的 unsafe fn 定义行计入必须 unsafe。"""
    crate = _write_crate(
        tmp_path,
        """
unsafe fn read_raw(ptr: *const i32) -> i32 {
    *ptr
}
""",
    )

    result = aru.analyze_crate(crate)

    assert result["required_unsafe_lines"] == 2
    assert result["required_reasons"]["unsafe_fn_contract"] == 1
    assert result["required_reasons"]["raw_pointer_deref"] == 1


def test_raw_unsafe_rate_excludes_structural_lines_but_uses_total_lines(tmp_path: Path) -> None:
    """原始 unsafe 分子不计纯结构行，分母使用物理总行数。"""
    crate = _write_crate(
        tmp_path,
        """

pub extern "C" fn read_value(ptr: *const i32) -> i32 {
    unsafe {
        (*ptr)
    }
}
""",
    )

    result = aru.analyze_crate(crate)

    assert result["total_lines"] == 7
    assert result["raw_unsafe_lines"] == 1
    assert result["raw_unsafe_keyword_lines"] == 0
    assert result["raw_unsafe_ratio"] == 1 / 7
    assert result["required_unsafe_ratio"] == 1 / 7
