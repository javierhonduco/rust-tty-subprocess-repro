fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::process::{Command, Stdio};
    use std::{thread, time::Duration};
    use rand;

    #[test]
    fn test_issue() {
        let num: u64 = rand::random();       
        let mut ruby_process = Command::new("docker")
            .args([
                "run",
                "--rm",
                "-it", // :)
                "--name",
                format!("random-name-{}", num).as_str(),
                "-v",
                "/home/vagrant/git/:/usr/src/myapp",
                "-w",
                "/usr/src/myapp",
                "ruby:3.0",
                "ruby",
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("Failed to start echo process");
   
   
            thread::sleep(Duration::from_millis(200));
    
            ruby_process.kill().expect("kill");
           
    } 
}
