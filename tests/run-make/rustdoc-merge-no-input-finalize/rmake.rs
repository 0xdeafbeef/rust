// Running --merge=finalize without an input crate root should not trigger ICE.
// Issue: https://github.com/rust-lang/rust/issues/146646

//@ needs-target-std

use run_make_support::{path, rustdoc};

fn main() {
    let out_dir = path("out");
    let merged_dir = path("merged");
    let parts_out_dir = path("parts");
    rustdoc()
        .input("sierra.rs")
        .out_dir(&out_dir)
        .arg("-Zunstable-options")
        .arg(format!("--parts-out-dir={}", parts_out_dir.display()))
        .arg("--merge=none")
        .run();

    let output = rustdoc()
        .arg("-Zunstable-options")
        .out_dir(&out_dir)
        .arg(format!("--include-parts-dir={}", parts_out_dir.display()))
        .arg("--merge=finalize")
        .run();

    output.assert_exit_code(0);
}
