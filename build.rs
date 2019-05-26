use std::process;

fn main() {
    // Just run `make` to generate the chip definitions
    if !process::Command::new("make").status().unwrap().success() {
        eprintln!("
avr-device: Running `make` resulted in a failure.

    Please make sure, you have all dependencies
    installed.  They are:
        - atdf2svd
        - svd2rust
        - form
        - rustfmt (for the toolchain you are using!)
        - PyYAML

    For more info, look at README file:
        https://github.com/Rahix/avr-device/blob/master/README.md
");
        panic!("make failed");
    }
}
