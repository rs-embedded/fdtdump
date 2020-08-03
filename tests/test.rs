extern crate tempdir;

use std::env;
use std::path::PathBuf;
use std::process::Command;

static DTB_PATH: &str = "test.dtb";

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
    // To find the directory where the built binary is, we walk up the directory tree of the test binary until the
    // parent is "target/".
    let mut binary_path =
        env::current_exe().expect("need current binary path to find binary to test");
    loop {
        {
            let parent = binary_path.parent();
            if parent.is_none() {
                panic!(
                    "Failed to locate binary path from original path: {:?}",
                    env::current_exe()
                );
            }
            let parent = parent.unwrap();
            if parent.is_dir() && parent.file_name().unwrap() == "target" {
                break;
            }
        }
        binary_path.pop();
    }

    binary_path.push(if cfg!(target_os = "windows") {
        format!("{}.exe", env!("CARGO_PKG_NAME"))
    } else {
        env!("CARGO_PKG_NAME").to_string()
    });

    let mut work_dir = PathBuf::new();
    work_dir.push(env!("CARGO_MANIFEST_DIR"));
    work_dir.push("tests");

    let mut cmd = Command::new(binary_path);
    let cmd = cmd.arg(DTB_PATH).current_dir(work_dir);

    assert_eq!(
        DTB_DUMP,
        std::str::from_utf8(cmd.output().unwrap().stdout.as_slice()).unwrap()
    );
}
