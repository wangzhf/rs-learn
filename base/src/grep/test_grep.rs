use std::env::args;
use std::process::Command;
use std::time::Duration;

fn grep() {
    let mut arg_iter = args();
    arg_iter.next().unwrap();
    let pattern = arg_iter.next().unwrap_or("main".to_string());
    let pt = arg_iter.next().unwrap_or("./".to_string());
    let output = Command::new("/usr/bin/grep")
        .arg("-n")
        .arg("-r")
        .arg(&pattern)
        .arg(&pt)
        .output()
        .unwrap_or_else(|e| panic!("wg panic because: {}", e));
    println!("output: ");
    let st = String::from_utf8_lossy(&output.stdout);
    let lines = st.split("\n");
    for line in lines {
        println!("{}", line);
    }
}

fn grep2() {
    let mut arg_iter = args();
    arg_iter.next();

    let pattern = arg_iter.next().unwrap_or("main".to_string());
    let pt = arg_iter.next().unwrap_or("./".to_string());
    let child = Command::new("grep")
        .arg("-n")
        .arg("-r")
        .arg(&pattern)
        .arg(&pt)
        .spawn().unwrap();

    std::thread::sleep(Duration::from_millis(1000));
    println!("{}", "I am working hard to compute.");
    let out = child.wait_with_output().unwrap();
    let out_str = String::from_utf8_lossy(&out.stdout);
    for line in out_str.split("\n") {
        println!("{}", line);
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_grep() {
        grep();
    }

    #[test]
    fn test_grep2() {
        grep2();
    }

}
