use std::process::{ Command, Stdio };

fn main() {
    let mut find = Command::new("fd");
    find.args([
        ".*",
        "/home/alucherdi/hj/",
        "-d", "2",
        "-t", "d",
        "--min-depth", "2"
    ]);

    find.stdout(Stdio::piped());

    let result = find.output().expect("Error executing fd");
    println!("status: {}", result.status);
    println!("stdout: \n{}", String::from_utf8_lossy(&result.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&result.stderr));
}
