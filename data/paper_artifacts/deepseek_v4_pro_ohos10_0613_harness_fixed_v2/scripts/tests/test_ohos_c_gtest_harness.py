"""测试 OHOS C gtest harness 的解析和统计口径。"""

from pathlib import Path

import ohos_c_gtest_harness as harness
from ohos_c_gtest_harness import (
    RustFunction,
    _audio_entry_source,
    _audio_test_helper_sources,
    _build_appverify_helpers,
    _build_securec,
    _compile_helper_c_source,
    _collect_existing_no_mangle_names,
    _extra_link_flags_for_project,
    _generate_exports_source,
    _host_shim_source,
    _manager_semantic_fixture_source,
    _patch_host_test_source,
    _prepare_test_source_for_host,
    _patch_cpp_vdi_init,
    build_staticlib,
    extract_unresolved_symbols,
    include_dirs_for,
    parse_gtest_cases,
    parse_failed_tests,
    parse_gtest_counts,
    parse_gtest_list,
    parse_rust_functions,
    split_top_level_commas,
    summarize_isolated_gtest_runs,
    summarize_results,
)


def test_build_staticlib_only_builds_library(monkeypatch, tmp_path: Path) -> None:
    """构建 staticlib 时只构建 lib 目标，避免无关 bin 链接污染测试。"""
    calls = []

    def fake_run_command(cmd, cwd, timeout, env):
        calls.append((list(cmd), cwd, timeout, env))
        lib_dir = tmp_path / "target/release"
        lib_dir.mkdir(parents=True)
        (lib_dir / "libdemo.a").write_bytes(b"")
        return {"ok": True, "cmd": list(cmd)}

    monkeypatch.setattr(harness, "run_command", fake_run_command)
    result, staticlib = build_staticlib(tmp_path, 30)

    assert result["ok"] is True
    assert staticlib == tmp_path / "target/release/libdemo.a"
    assert calls[0][0] == ["cargo", "build", "--release", "--offline", "--lib"]
    assert calls[0][3]["CARGO_TARGET_DIR"] == str(tmp_path / "target")


def test_split_top_level_commas_keeps_nested_types() -> None:
    """顶层逗号切分不会拆开泛型、数组和函数指针类型。"""
    text = "a: *mut Foo, b: Option<unsafe extern \"C\" fn(x: i32, y: i32)>, c: [u8; 4]"
    assert split_top_level_commas(text) == [
        "a: *mut Foo",
        "b: Option<unsafe extern \"C\" fn(x: i32, y: i32)>",
        "c: [u8; 4]",
    ]


def test_parse_gtest_list_counts_cases() -> None:
    """gtest list 输出按 suite/case 统计用例数。"""
    output = """
FooTest.
  CaseA
  CaseB
BarTest.
  CaseC
"""
    assert parse_gtest_list(output) == 3


def test_parse_gtest_cases_returns_full_names() -> None:
    """gtest list 输出能保留 suite.case 完整用例名。"""
    output = """
FooTest.
  CaseA
  CaseB
BarTest.
  CaseC
"""
    assert parse_gtest_cases(output) == ["FooTest.CaseA", "FooTest.CaseB", "BarTest.CaseC"]


def test_parse_gtest_counts_and_failures() -> None:
    """gtest run 输出能得到总数、通过数和失败用例名。"""
    output = """
[==========] 3 tests from 2 test suites ran.
[  PASSED  ] 2 tests.
[  FAILED  ] 1 test, listed below:
[  FAILED  ] FooTest.CaseB
"""
    assert parse_gtest_counts(output) == {
        "tests_total": 3,
        "tests_passed": 2,
        "tests_failed": 1,
        "pass_rate": 2 / 3,
    }
    assert parse_failed_tests(output) == ["FooTest.CaseB"]


def test_extract_unresolved_symbols() -> None:
    """链接错误中的 undefined reference 能被去重提取。"""
    stderr = """
/usr/bin/ld: a.o: undefined reference to `HdfSbufObtain'
/usr/bin/ld: b.o: undefined reference to 'APPVERI_AppVerify'
/usr/bin/ld: c.o: undefined reference to `HdfSbufObtain'
"""
    assert extract_unresolved_symbols(stderr) == ["APPVERI_AppVerify", "HdfSbufObtain"]


def test_patch_host_test_source_fixes_size_t_decode_len() -> None:
    """64 位 host 下 base64 helper 不能把 size_t 输出写入 int32_t 栈变量。"""
    source = """
int32_t len = 0;
mbedtls_base64_decode(reinterpret_cast<unsigned char *>(buffer.get()), static_cast<size_t>(wholeLen),
    reinterpret_cast<size_t *>(&len), reinterpret_cast<const unsigned char *>(org), static_cast<size_t>(wholeLen));
int32_t num = 0;
"""

    patched = _patch_host_test_source(source)

    assert "size_t decodedLen = 0;" in patched
    assert "&decodedLen" in patched
    assert "reinterpret_cast<size_t *>(&len)" not in patched
    assert "len = static_cast<int32_t>(decodedLen);" in patched


def test_prepare_test_source_for_host_writes_patched_copy(tmp_path: Path) -> None:
    """host 测试源码修正只写入临时 build 目录，不修改原始测试文件。"""
    src = tmp_path / "write_file.cpp"
    original = """
int32_t len = 0;
mbedtls_base64_decode(out, out_len,
    reinterpret_cast<size_t *>(&len), in, in_len);
int32_t num = 0;
"""
    src.write_text(original, encoding="utf-8")

    patched_path, notes = _prepare_test_source_for_host(src, tmp_path / "build")

    assert notes == ["host_size_t_decode_len"]
    assert patched_path != src
    assert src.read_text(encoding="utf-8") == original
    assert "&decodedLen" in patched_path.read_text(encoding="utf-8")


def test_patch_host_test_source_zero_initializes_audio_struct_locals() -> None:
    """audio 官方 helper 的局部结构体在 host 测试中需要确定性零初始化。"""
    source = """
int32_t CodecGetConfigInfoTest(void)
{
    struct HdfDeviceObject device;
    struct CodecData codecData;
    struct DaiData daiData;
}
"""

    patched = _patch_host_test_source(source)

    assert "struct HdfDeviceObject device = {0};" in patched
    assert "struct CodecData codecData = {0};" in patched
    assert "struct DaiData daiData = {0};" in patched
    assert "struct CodecData codecData;" not in patched


def test_summarize_isolated_gtest_runs_counts_abort_as_failure() -> None:
    """单用例隔离统计把 abort 用例计为失败而不是吞掉分母。"""
    summary = summarize_isolated_gtest_runs(
        [
            {"case": "Foo.A", "result": {"ok": True}},
            {"case": "Foo.B", "result": {"ok": False, "returncode": -6}},
        ]
    )
    assert summary == {
        "tests_total": 2,
        "tests_passed": 1,
        "tests_failed": 1,
        "pass_rate": 0.5,
        "failed_tests": ["Foo.B"],
    }


def test_parse_rust_functions_from_public_sources(tmp_path: Path) -> None:
    """Rust extern C 函数解析使用 crate 目录作为公开入口。"""
    src = tmp_path / "src"
    src.mkdir()
    (src / "types.rs").write_text("", encoding="utf-8")
    (src / "src_demo.rs").write_text(
        """
pub extern "C" fn Foo(a: i32, b: *mut crate::types::Bar) -> i32 { 0 }
pub unsafe extern "C" fn Bar(x: u32) { let _ = x; }
""",
        encoding="utf-8",
    )
    functions, diagnostics = parse_rust_functions(tmp_path)
    assert diagnostics == []
    assert [(fn.name, fn.module, fn.arg_names, fn.return_type, fn.is_unsafe) for fn in functions] == [
        ("Foo", "src_demo", ("a", "b"), "i32", False),
        ("Bar", "src_demo", ("x",), None, True),
    ]


def test_parse_rust_functions_ignores_commented_failed_translations(tmp_path: Path) -> None:
    """注释中的失败译文签名不能污染真实导出函数扫描。"""
    src = tmp_path / "src"
    src.mkdir()
    (src / "types.rs").write_text("", encoding="utf-8")
    (src / "src_demo.rs").write_text(
        """
pub extern "C" fn Real(a: i32) -> i32 { a }
// pub extern "C" fn Commented(a: i32) -> i32 { a }
/*
pub extern "C" fn BlockCommented(a: i32) -> i32 { a }
*/
// pub extern "C" fn Real(a: i32) -> i32 { a + 1 }
""",
        encoding="utf-8",
    )

    functions, diagnostics = parse_rust_functions(tmp_path)

    assert diagnostics == []
    assert [(fn.name, fn.module) for fn in functions] == [("Real", "src_demo")]


def test_generate_exports_only_wraps_requested_rust_functions() -> None:
    """按测试对象引用过滤导出，避免无关函数拖入链接依赖。"""
    functions = [
        RustFunction("Foo", "src_demo", "a: i32", "i32", ("a",), False),
        RustFunction("Bar", "src_demo", "", None, (), True),
    ]
    text, meta = _generate_exports_source(functions, ["Foo", "Missing", "_ZSt4cerr"])
    assert "fn Foo" in text
    assert "fn Bar" not in text
    assert meta["requested_exports"] == ["Foo"]
    assert meta["unmatched_requested_export_count"] == 2
    assert "use crate::*;" in text
    assert "use core::ffi::" in text


def test_generate_exports_skips_functions_already_no_mangle_exported() -> None:
    """Claude 产物已 no_mangle 导出的函数不能再生成同名 wrapper。"""
    functions = [
        RustFunction("HdfCloseVdi", "load_vdi", "vdi_obj: *mut HdfVdiObject", None, ("vdi_obj",), True),
        RustFunction("HdfLoadVdi", "load_vdi", "lib_name: *const c_char", "*mut HdfVdiObject", ("lib_name",), True),
    ]

    text, meta = _generate_exports_source(functions, ["HdfCloseVdi", "HdfLoadVdi"], ["HdfCloseVdi"])

    assert "fn HdfCloseVdi" not in text
    assert "fn HdfLoadVdi" in text
    assert meta["already_exported_requested"] == ["HdfCloseVdi"]


def test_collect_existing_no_mangle_names_accepts_unsafe_attr(tmp_path: Path) -> None:
    """Rust 2024 的 unsafe(no_mangle) 也应识别为已有 C ABI 导出。"""
    src = tmp_path / "src"
    src.mkdir()
    (src / "lib.rs").write_text(
        """
#[no_mangle]
pub unsafe extern "C" fn Plain() {}
#[unsafe(no_mangle)]
pub extern "C" fn UnsafeAttr() {}
""",
        encoding="utf-8",
    )

    assert _collect_existing_no_mangle_names(tmp_path) == ["Plain", "UnsafeAttr"]


def test_compile_helper_weakens_symbols_already_defined_by_rust_staticlib(tmp_path: Path) -> None:
    """官方 helper 只补缺失符号，和 Rust staticlib 重叠时退成弱定义。"""
    src = tmp_path / "helper.c"
    obj = tmp_path / "helper.o"
    src.write_text("int HdfDeviceInfoNewInstance(void) { return 7; }\n", encoding="utf-8")

    result = _compile_helper_c_source(
        src,
        obj,
        includes=[],
        build_dir=tmp_path,
        timeout=30,
        static_symbols=["HdfDeviceInfoNewInstance"],
    )

    assert result["ok"] is True
    assert result["weakened_symbols"] == ["HdfDeviceInfoNewInstance"]
    assert "#pragma weak HdfDeviceInfoNewInstance" in Path(result["patched_source"]).read_text(encoding="utf-8")
    assert any(line.endswith(" W HdfDeviceInfoNewInstance") for line in harness.run_command(
        ["nm", "-g", "--defined-only", str(obj)],
        tmp_path,
        30,
    )["stdout"].splitlines())


def test_securec_weakens_symbols_already_defined_by_rust_staticlib(monkeypatch, tmp_path: Path) -> None:
    """securec helper 与 Claude 产物同名时退成弱定义，避免链接阶段适配阻塞。"""
    src_dir = tmp_path / "ohos/third_party/bounds_checking_function/src"
    src_dir.mkdir(parents=True)
    (src_dir / "memcpy_s.c").write_text("int memcpy_s(void) { return 0; }\n", encoding="utf-8")

    objs, reports = _build_securec(tmp_path / "build", tmp_path / "ohos", [], 30, ["memcpy_s"])

    assert [obj.name for obj in objs] == ["memcpy_s.o"]
    assert reports[0]["weakened_symbols"] == ["memcpy_s"]
    assert any(line.endswith(" W memcpy_s") for line in harness.run_command(
        ["nm", "-g", "--defined-only", str(objs[0])],
        tmp_path,
        30,
    )["stdout"].splitlines())


def test_weak_helper_compile_keeps_original_source_include_dir(tmp_path: Path) -> None:
    """弱化 helper 副本放到 build 目录后仍能 include 原始源旁边的私有头。"""
    src_dir = tmp_path / "securec"
    build_dir = tmp_path / "build"
    src_dir.mkdir()
    build_dir.mkdir()
    src = src_dir / "helper.c"
    obj = tmp_path / "helper.o"
    (src_dir / "private.h").write_text("#define C2R_HELPER_VALUE 7\n", encoding="utf-8")
    src.write_text('#include "private.h"\nint Helper(void) { return C2R_HELPER_VALUE; }\n', encoding="utf-8")

    result = _compile_helper_c_source(
        src,
        obj,
        includes=[],
        build_dir=build_dir,
        timeout=30,
        static_symbols=["Helper"],
    )

    assert result["ok"] is True
    assert result["weakened_symbols"] == ["Helper"]


def test_appverify_link_globals_skips_rust_defined_globals(monkeypatch, tmp_path: Path) -> None:
    """appverify host 全局只补 Rust staticlib 没有导出的变量。"""
    calls = []

    def fake_compile(src, obj, includes, cwd, timeout, extra=None):
        calls.append(src)
        obj.parent.mkdir(parents=True, exist_ok=True)
        obj.write_bytes(b"")
        return {"ok": True, "cmd": ["gcc", str(src)]}

    monkeypatch.setattr(harness, "_compile_c_source", fake_compile)
    ohos_root = tmp_path / "ohos"
    includes = tmp_path / "includes"
    (ohos_root / "third_party/mbedtls/library").mkdir(parents=True)
    (ohos_root / "third_party/cJSON").mkdir(parents=True)
    (ohos_root / "third_party/cJSON/cJSON.c").write_text("", encoding="utf-8")

    _objs, report = _build_appverify_helpers(
        tmp_path / "build",
        ohos_root,
        [includes],
        tmp_path / "project",
        ["g_productDiffFunc", "g_rootCaG2Cert"],
        30,
    )

    link_source = (tmp_path / "build/appverify_link_globals.c").read_text(encoding="utf-8")
    assert "ProductDiff g_productDiffFunc" not in link_source
    assert "mbedtls_x509_crt g_rootCaG2Cert" not in link_source
    assert "bool g_rootCertLoaded = false;" in link_source
    assert report["link_globals_source"]["skipped_existing_globals"] == ["g_productDiffFunc", "g_rootCaG2Cert"]
    assert calls


def test_audio_helper_sources_follow_test_file_stems(tmp_path: Path) -> None:
    """audio helper 只按当前 gtest 文件名选择官方 C helper。"""
    ohos_root = tmp_path / "ohos"
    helper_dir = ohos_root / "drivers/hdf_core/framework/test/unittest/model/audio/src"
    helper_dir.mkdir(parents=True)
    (helper_dir / "audio_core_test.c").write_text("", encoding="utf-8")
    (helper_dir / "hdf_audio_test.c").write_text("", encoding="utf-8")

    test_srcs = [tmp_path / "project/test/audio_core_test.cpp", tmp_path / "project/test/other_test.cpp"]

    assert _audio_test_helper_sources(test_srcs, ohos_root) == [helper_dir / "audio_core_test.c"]


def test_include_dirs_for_adds_linux_osal_adapter(tmp_path: Path) -> None:
    """host 编译 include 覆盖 Linux OSAL adapter 和 utils 头。"""
    source_dir = tmp_path / "project"
    build_dir = tmp_path / "build"
    ohos_root = tmp_path / "ohos"
    for path in (
        source_dir,
        build_dir,
        ohos_root / "drivers/hdf_core/adapter/khdf/linux/osal/include",
        ohos_root / "drivers/hdf_core/framework/include/utils",
        ohos_root / "drivers/hdf_core/framework/model/audio/dispatch/include",
    ):
        path.mkdir(parents=True)

    dirs = include_dirs_for(source_dir, ohos_root, build_dir)

    assert ohos_root / "drivers/hdf_core/adapter/khdf/linux/osal/include" in dirs
    assert ohos_root / "drivers/hdf_core/framework/include/utils" in dirs
    assert ohos_root / "drivers/hdf_core/framework/model/audio/dispatch/include" in dirs


def test_posix_uses_host_file_shim_instead_of_kernel_osal_file(tmp_path: Path) -> None:
    """posix 官方 helper 不直接编译内核态 osal_file.c，而由 host 文件语义 shim 适配。"""
    ohos_root = tmp_path / "ohos"
    osal_dir = ohos_root / "drivers/hdf_core/framework/test/unittest/osal"
    osal_dir.mkdir(parents=True)
    for name in ("osal_all_test.c", "osal_get_case_test.c", "osal_list_test.c", "osal_work_test.c", "osal_test_entry.c"):
        (osal_dir / name).write_text("", encoding="utf-8")

    helpers = harness._pre_staticlib_helper_sources("posix__demo", ohos_root)
    shim_sources = dict(harness._environment_shim_sources("posix__demo"))
    file_shim = shim_sources["c2r_osal_file_host_shim"]

    assert [path.name for _, path, _ in helpers] == [
        "osal_all_test.c",
        "osal_get_case_test.c",
        "osal_list_test.c",
        "osal_work_test.c",
        "osal_test_entry.c",
    ]
    assert "osal_file.c" not in [path.name for _, path, _ in helpers]
    assert "int fd = open(path, flags, (mode_t)rights);" in file_shim
    assert "ssize_t ret = read(fd, buf, (size_t)length);" in file_shim
    assert "ssize_t ret = write(fd, string, (size_t)length);" in file_shim
    assert "off_t ret = lseek(fd, offset, whence);" in file_shim


def test_posix_does_not_fake_pthread_scheduler_state() -> None:
    """posix host runner 不伪造 pthread 调度状态，避免污染 OSAL 行为对照。"""
    shim_sources = dict(harness._environment_shim_sources("posix__demo"))
    flags = harness._link_wrap_flags_for_project("posix__demo")

    assert "c2r_posix_pthread_sched_shim" not in shim_sources
    assert not any("pthread_" in flag for flag in flags)


def test_posix_excludes_host_infeasible_realtime_scheduler_cases() -> None:
    """普通 Linux host 不统计无法真实提供 realtime pthread 调度语义的 OSAL 用例。"""
    exclusions = harness._host_infeasible_gtest_exclusions("posix__demo")
    filter_args = harness._gtest_filter_for_exclusions(exclusions)

    assert [item["case"] for item in exclusions] == [
        "OsalTest.OsalGetThread001",
        "OsalTest.OsalGetThread003",
        "OsalTest.OsalGetAll001",
        "OsalTestPosix.OsalGetThread001",
        "OsalTestPosix.OsalGetThread003",
        "OsalTestPosix.OsalGetAll001",
    ]
    assert filter_args == [
        "--gtest_filter=-OsalTest.OsalGetThread001:OsalTest.OsalGetThread003:OsalTest.OsalGetAll001:"
        "OsalTestPosix.OsalGetThread001:OsalTestPosix.OsalGetThread003:OsalTestPosix.OsalGetAll001"
    ]
    assert harness._host_infeasible_gtest_exclusions("host__demo") == []
    assert harness._gtest_filter_for_exclusions([]) == []


def test_common_links_official_neighbor_audio_sources(tmp_path: Path) -> None:
    """common audio 测试用官方相邻模块补跨模块依赖，不覆盖 common 被测函数。"""
    ohos_root = tmp_path / "ohos"
    audio_root = ohos_root / "drivers/hdf_core/framework/model/audio"
    for rel in ("core/src/audio_core.c", "core/src/audio_parse.c", "sapm/src/audio_sapm.c"):
        path = audio_root / rel
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text("", encoding="utf-8")

    sources = harness._supplemental_audio_product_c_sources("common__demo", ohos_root)
    shim_sources = dict(harness._environment_shim_sources("common__demo"))

    assert [path.name for _, path in sources] == ["audio_core.c", "audio_parse.c", "audio_sapm.c"]
    assert "c2r_core_audio_fixture" in shim_sources
    assert "DeviceResourceGetIfaceInstance" in shim_sources["c2r_core_audio_fixture"]


def test_common_audio_fixture_rejects_unknown_resource_nodes(tmp_path: Path) -> None:
    """audio fixture 不能对随机 DeviceResourceNode 伪造配置读取成功。"""
    source = dict(harness._environment_shim_sources("common__demo"))["c2r_core_audio_fixture"]

    assert "C2rIsKnownAudioNode" in source
    assert "if (!C2rIsKnownAudioNode(node))" in source
    assert "*value = def;" in source
    assert "return HDF_FAILURE;" in source


def test_host_shim_provides_osal_io_unmap_environment_symbol() -> None:
    """host shim 补齐内联 OSAL IO 环境符号，避免 audio core 链接失败。"""
    source = _host_shim_source()

    assert "void *OsalIoRemap(unsigned long phys_addr, unsigned long size)" in source
    assert "void OsalIoUnmap(void *addr)" in source
    assert "(void)addr;" in source


def test_host_shim_provides_user_copy_environment_symbols() -> None:
    """common audio mmap 路径需要 host 侧 CopyToUser/CopyFromUser 环境符号。"""
    source = _host_shim_source()

    assert "int32_t CopyFromUser(void *dest, const void *src, uint32_t count)" in source
    assert "int32_t CopyToUser(void *dest, const void *src, uint32_t count)" in source
    assert "memcpy(dest, src, (size_t)count);" in source


def test_host_shim_provides_common_audio_platform_environment_symbols() -> None:
    """common audio 测试需要 host 侧平台符号才能进入被测 Rust 逻辑。"""
    source = _host_shim_source()

    for name in (
        "HdfDeviceObjectSetServInfo",
        "HdfDeviceObjectUpdate",
        "I2cOpen",
        "I2cClose",
        "I2cTransfer",
    ):
        assert name in source


def test_appverify_adds_openssl_link_flags() -> None:
    """appverify Rust staticlib 依赖 openssl-sys，最终 C++ 链接必须带系统库。"""
    assert _extra_link_flags_for_project("appverify_lite__demo") == ["-lssl", "-lcrypto"]
    assert _extra_link_flags_for_project("common__demo") == []


def test_sapm_supplemental_shim_exports_codec_reg_update_dependency() -> None:
    """sapm 相邻 audio core 依赖必须导出官方符号名。"""
    source = harness._supplemental_audio_product_shim_source("sapm__demo")

    assert source is not None
    assert "int32_t AudioUpdateCodecRegBits(struct CodecDevice *codec" in source
    assert "return C2rAudioUpdateCodecRegBits(codec, reg, mask, shift, value);" in source


def test_audio_entry_source_keeps_only_selected_helper_cases(tmp_path: Path) -> None:
    """filtered audio entry 只转发当前 helper 声明的官方测试函数。"""
    ohos_root = tmp_path / "ohos"
    src_dir = ohos_root / "drivers/hdf_core/framework/test/unittest/model/audio/src"
    inc_dir = ohos_root / "drivers/hdf_core/framework/test/unittest/model/audio/include"
    src_dir.mkdir(parents=True)
    inc_dir.mkdir(parents=True)
    (src_dir / "hdf_audio_test.c").write_text(
        """
static HdfTestCaseList g_hdfAudioTestCaseList[] = {
    {AUDIO_ADM_TEST_A, AudioSelectedTest},
    {AUDIO_ADM_TEST_B, AudioUnselectedTest},
};
""",
        encoding="utf-8",
    )
    helper = src_dir / "audio_core_test.c"
    helper.write_text("", encoding="utf-8")
    (inc_dir / "audio_core_test.h").write_text(
        "int32_t AudioSelectedTest(void);\n",
        encoding="utf-8",
    )

    source, meta = _audio_entry_source([helper], ohos_root)

    assert "case AUDIO_ADM_TEST_A" in source
    assert "AudioSelectedTest()" in source
    assert "AUDIO_ADM_TEST_B" not in source
    assert "AudioUnselectedTest" not in source
    assert meta["selected_case_count"] == 1


def test_audio_entry_source_uses_project_test_cmd_values(tmp_path: Path) -> None:
    """audio entry 按项目测试 header 的 TEST* 实际整数值转发。"""
    ohos_root = tmp_path / "ohos"
    src_dir = ohos_root / "drivers/hdf_core/framework/test/unittest/model/audio/src"
    inc_dir = ohos_root / "drivers/hdf_core/framework/test/unittest/model/audio/include"
    test_dir = tmp_path / "project/test/unittest/common"
    src_dir.mkdir(parents=True)
    inc_dir.mkdir(parents=True)
    test_dir.mkdir(parents=True)
    (src_dir / "hdf_audio_test.c").write_text(
        """
static HdfTestCaseList g_hdfAudioTestCaseList[] = {
    {AUDIO_ADM_TEST_GETSERVICENAME, CodecGetServiceNameTest},
    {AUDIO_ADM_TEST_GETDAINAME, CodecGetDaiNameTest},
};
""",
        encoding="utf-8",
    )
    helper = src_dir / "audio_codec_base_test.c"
    helper.write_text("", encoding="utf-8")
    (inc_dir / "audio_codec_base_test.h").write_text(
        "int32_t CodecGetServiceNameTest(void);\nint32_t CodecGetDaiNameTest(void);\n",
        encoding="utf-8",
    )
    (test_dir / "audio_common_test.h").write_text(
        """
enum HDFAudioTestCommonCmd {
    TESTI2CREADWRITE = 48,
    TESTREGBITSREAD,
    TESTREGBITSUPDATE,
    TESTDEVICEFREQUENCYPARSE,
    TESTDAIPARAMSUPDATE,
    TESTDEVICECFGGET,
    TESTDEVICECTRLREGINIT,
    TESTDEVICEREGREAD,
    TESTDEVICEREGWRITE,
    TESTAGETCONFIGINFO,
    TESTGETSERVICENAME,
};
""",
        encoding="utf-8",
    )
    test_src = test_dir / "audio_codec_base_test.cpp"
    test_src.write_text(
        '#include "audio_common_test.h"\n'
        "void f() { struct HdfTestMsg msg = {701, TESTGETSERVICENAME, -1}; }\n",
        encoding="utf-8",
    )

    source, meta = _audio_entry_source([helper], ohos_root, [test_src])

    assert "case 58:" in source
    assert "CodecGetServiceNameTest()" in source
    assert "case AUDIO_ADM_TEST_GETSERVICENAME" not in source
    assert meta["switch_cases"][0] == {
        "case": "58",
        "function": "CodecGetServiceNameTest",
        "source_cmd": "TESTGETSERVICENAME",
    }


def test_manager_filters_to_semantic_lite_manager_test(tmp_path: Path) -> None:
    """manager 只编译能触达 DevSvcManager 翻译产物的原始测试源。"""
    selected_path = tmp_path / "hdf_lite_manager_test.cpp"
    remote_path = tmp_path / "hdf_remote_adapter_test.cpp"
    sbuf_path = tmp_path / "hdf_sbuf_test.cpp"

    selected, excluded = harness._filter_project_test_sources(
        "manager__demo",
        [remote_path, selected_path, sbuf_path],
    )

    assert selected == [selected_path]
    assert excluded == [remote_path, sbuf_path]


def test_manager_fixture_directly_refs_rust_manager_exports() -> None:
    """manager fixture 必须通过 Rust DevSvcManager 导出形成语义覆盖。"""
    source = _manager_semantic_fixture_source()

    for name in (
        "DevSvcManagerCreate",
        "DevSvcManagerAddService",
        "DevSvcManagerGetService",
        "DevSvcManagerRemoveService",
        "DevSvcManagerListService",
        "DevSvcManagerListAllService",
    ):
        assert f"extern " in source
        assert name in source

    assert "DevSvcManagerGetInstance" not in source
    assert "HdfIoServiceBind" in source
    assert "HdfGetServiceNameByDeviceClass" in source
    assert "return &g_c2r_bound_service;" in source


def test_manager_pre_staticlib_helper_compiles_before_export_selection() -> None:
    """manager fixture 放在 pre-staticlib 阶段，确保 undefined 符号驱动 Rust 导出。"""
    generated = harness._pre_staticlib_generated_sources("manager__demo")

    assert [name for name, _ in generated] == ["c2r_manager_semantic_fixture"]
    assert "DevSvcManagerGetService" in generated[0][1]


def test_manager_fixture_clones_input_sbuf_before_dispatch_read() -> None:
    """manager dispatch 不能直接消费原始 data，必须先复制出本地 sbuf。"""
    source = _manager_semantic_fixture_source()

    assert "localData = HdfSbufCopy(data);" in source
    assert "struct HdfSBuf *localData" in source
    assert "C2rReadString(localData)" in source
    assert "HdfSbufRecycle(localData);" in source


def test_host_shim_keeps_ipc_null_string_failure_semantics() -> None:
    """IPC sbuf 不能继续沿用 raw 的 NULL string 写成功语义。"""
    source = _host_shim_source()

    assert "C2rIpcSbufWriteString" in source
    assert "value == NULL" in source
    assert "return false;" in source
    assert "sbuf->writeString = C2rIpcSbufWriteString;" in source
    assert "return C2rWrapIpcSbuf(SbufObtainRaw(capacity));" in source
    assert "return C2rWrapIpcSbuf(SbufBindRaw(base, size));" in source


def test_host_shim_forwards_audio_and_fails_unknown_messages() -> None:
    """HdfTestSendMsgToService 只能转发 audio entry，未知消息不能伪通过。"""
    source = _host_shim_source()

    assert "HdfAudioEntry(msg)" in source
    assert "if (msg->cmd == 701)" in source
    assert "return msg->result;" in source
    assert source.rstrip().endswith("'''") is False
    assert "return -1;" in source


def test_patch_cpp_vdi_init_uses_plain_extern_c() -> None:
    """C++ fixture 宏展开不能写出带反斜杠的 extern C。"""
    patched = _patch_cpp_vdi_init("HDF_VDI_INIT(OHOS::VDI::Sample::V1_0::g_vdiB);\n")
    assert 'extern "C" struct HdfVdiBase *hdfVdiDesc' in patched
    assert r"extern \"C\"" not in patched


def test_summarize_results_only_counts_entered_gtest() -> None:
    """汇总通过率只统计进入 gtest runner 且有测试数的项目。"""
    summary = summarize_results(
        [
            {
                "compiled": True,
                "executed": True,
                "tests_total": 4,
                "tests_passed": 3,
                "tests_failed": 1,
                "test_symbols_from_rust_staticlib": ["TranslatedFoo"],
            },
            {"compiled": True, "executed": False, "tests_total": None, "tests_passed": None, "tests_failed": None},
        ]
    )
    assert summary["projects_total"] == 2
    assert summary["projects_compiled"] == 2
    assert summary["projects_executed"] == 1
    assert summary["projects_with_gtest_counts"] == 1
    assert summary["tests_total"] == 4
    assert summary["tests_passed"] == 3
    assert summary["tests_failed"] == 1
    assert summary["pass_rate"] == 0.75
    assert summary["projects_with_rust_symbol_coverage"] == 1
    assert summary["semantic_projects_executed"] == 1
    assert summary["semantic_tests_total"] == 4
    assert summary["semantic_tests_passed"] == 3
    assert summary["semantic_tests_failed"] == 1
    assert summary["semantic_pass_rate"] == 0.75


def test_summarize_results_separates_environment_only_passes() -> None:
    """没有触达 Rust 翻译符号的 gtest 通过不能计入语义通过率。"""
    summary = summarize_results(
        [
            {
                "compiled": True,
                "executed": True,
                "tests_total": 4,
                "tests_passed": 4,
                "tests_failed": 0,
                "test_symbols_from_rust_staticlib": [],
            },
            {
                "compiled": True,
                "executed": True,
                "tests_total": 2,
                "tests_passed": 1,
                "tests_failed": 1,
                "test_symbols_from_rust_staticlib": ["TranslatedBar"],
            },
        ]
    )

    assert summary["projects_with_gtest_counts"] == 2
    assert summary["tests_total"] == 6
    assert summary["tests_passed"] == 5
    assert summary["pass_rate"] == 5 / 6
    assert summary["projects_with_rust_symbol_coverage"] == 1
    assert summary["semantic_projects_executed"] == 1
    assert summary["semantic_tests_total"] == 2
    assert summary["semantic_tests_passed"] == 1
    assert summary["semantic_tests_failed"] == 1
    assert summary["semantic_pass_rate"] == 0.5
