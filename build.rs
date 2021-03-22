use std::path::PathBuf;
use std::env;

fn main() {
    let mut cc = cc::Build::new();
    cc.warnings(false);

    let out_dir = PathBuf::from(&env::var_os("OUT_DIR").expect("OUT_DIR"));

    let target_pointer_width = env::var("CARGO_CFG_TARGET_POINTER_WIDTH").expect("target");
    let target_os = env::var("CARGO_CFG_TARGET_OS").expect("target");
    let is_windows = target_os == "windows";

    std::fs::write(out_dir.join("config.h"), format!(r#"
        {}
        #define HAVE_INT64_T 1
        #define HAVE_INTTYPES_H 1
        #define HAVE_POW 1
        #define HAVE_STRERROR 1
        #define HAVE_STRTOUL 1
        #define HAVE_SYS_TYPES_H 1
        #define HAVE_SYS_STAT_H 1
        #define HAVE_UINT64_T 1
        #define HAVE_UINTPTR_T 1
        #define PATHNAME_SEPARATOR '/'
        #define RANDOM rand
        #define OUTPUT_GIF_TO_TERMINAL 1
        #define GIF_ALLOCATOR_DEFINED 1
        #define SIZEOF_UNSIGNED_INT 4
    "#, if is_windows {""} else {"#define HAVE_MKSTEMP 1"})).expect("OUT_DIR/config.h");

    cc.define("HAVE_CONFIG_H", Some("1"));
    cc.define("SIZEOF_VOID_P", Some(if target_pointer_width == "32" {"4"} else {"8"}));
    cc.define("SIZEOF_UNSIGNED_LONG", Some(if target_pointer_width == "32" || is_windows {"4"} else {"8"}));
    cc.define("VERSION", Some(concat!("\"", env!("CARGO_PKG_VERSION"), "\"")));
    cc.define("main", Some("gifsicle_main"));

    cc.include("vendor/include");
    cc.include("vendor/src");
    cc.include(out_dir);

    cc.file("vendor/src/clp.c");
    cc.file("vendor/src/fmalloc.c");
    cc.file("vendor/src/giffunc.c");
    cc.file("vendor/src/gifread.c");
    cc.file("vendor/src/gifunopt.c");
    cc.file("vendor/src/merge.c");
    cc.file("vendor/src/optimize.c");
    cc.file("vendor/src/quantize.c");
    cc.file("vendor/src/support.c");
    cc.file("vendor/src/xform.c");
    cc.file("vendor/src/gifsicle.c");
    cc.file("vendor/src/gifwrite.c");

    cc.compile("gifsicle");
}
