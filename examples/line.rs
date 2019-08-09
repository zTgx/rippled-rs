//git log  --pretty=tformat: --numstat
extern crate cast_rs;
use std::io::{self, Write};

fn main() {
    use std::process::Command;

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&["/C", "git log  --pretty=tformat: --numstat"])
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg(r#"find . -name "*.py" |xargs cat|wc -l"#)
                .output()
                .expect("failed to execute process")
    };

    io::stdout().write_all(&output.stdout).unwrap();
}
