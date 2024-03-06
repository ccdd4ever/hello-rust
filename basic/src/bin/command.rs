use std::process::Command;

fn main() {
    let output = Command::new("cmd").arg("dir").output().expect("failed to execute command");

    println!("status： {}", output.status);
    if output.status.success(){
        println!("stdout： {}", String::from_utf8_lossy(&output.stdout));
    }else {
            println!("stderr： {}", String::from_utf8_lossy(&output.stderr));
    }
}