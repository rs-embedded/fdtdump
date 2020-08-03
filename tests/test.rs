extern crate tempdir;

use std::process::Command;

static DTB_PATH: &str = "./tests/test.dtb";

static DTB_DUMP: &str = "/dts-v1/;
// magic:		0xd00dfeed
// totalsize:		0xaf (175)
// off_dt_struct:	0x38
// off_dt_strings:	0x98
// off_mem_rsvmap:	0x28
// version:		17
// last_comp_version:	16
// boot_cpuid_phys:	0x0
// size_dt_strings:	0x17
// size_dt_struct:	0x60

/ {
    prop = \"hello\";
    other@1 {
        prop2 = [00];
        prop3 = <0x00000000>;
        prop4;
    };
};
";

#[test]
fn get_base_coverage() {
    let target_dir = ::std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| String::from("target"));
    let mut cmd = Command::new(format!("{}/debug/fdtdump", target_dir));
    let cmd = cmd.arg(DTB_PATH);
    assert_eq!(
        DTB_DUMP,
        std::str::from_utf8(cmd.output().unwrap().stdout.as_slice()).unwrap()
    );
}
