extern crate tempdir;

use std::process::Command;

static DTS: &str = "/dts-v1/;

/ {
    prop = \"hello\";
    other@1 {
        prop2 = \"\";
        prop3 = <0x0>;
        prop4;
    };
};
";

static DTB: &str = "/dts-v1/;
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

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[test]
fn get_base_coverage() {
    let temp_dir = tempdir::TempDir::new("fdtdump-test").unwrap();
    let dts_path = temp_dir.path().join(Path::new("example.dts"));
    let dtb_path = temp_dir.path().join(Path::new("example.dtb"));

    let mut file = File::create(&dts_path).unwrap();
    file.write_all(DTS.as_bytes()).unwrap();

    let output = Command::new("dtc")
        .arg(dts_path.to_str().unwrap())
        .output()
        .unwrap();

    let mut file = File::create(&dtb_path).unwrap();
    file.write_all(output.stdout.as_slice()).unwrap();

    let target_dir = ::std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| String::from("target"));
    let mut cmd = Command::new(format!("{}/debug/fdtdump", target_dir));
    let cmd = cmd.arg(dtb_path);
    assert_eq!(
        DTB,
        std::str::from_utf8(cmd.output().unwrap().stdout.as_slice()).unwrap()
    );
}
