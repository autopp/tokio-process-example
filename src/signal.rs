use std::os::unix::process::ExitStatusExt;

use nix::sys::signal::{kill, Signal};
use tokio::{io::AsyncBufReadExt, io::BufReader, process::Command};

pub async fn signal() -> Result<String, String> {
    let script = r#"
set -eu
set -o pipefail

for i in {1..10}; do
    echo "$i"
    sleep 1
done
"#;
    let mut cmd = Command::new("bash")
        .args(["-c", script])
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();

    let stdout = cmd.stdout.take().unwrap();
    let mut reader = BufReader::new(stdout);
    loop {
        let mut line = String::new();
        reader.read_line(&mut line).await.unwrap();

        println!("{}", line.trim());

        if line == "3\n" {
            break;
        }
    }

    let pid = nix::unistd::Pid::from_raw(cmd.id().unwrap() as i32);
    kill(pid, Signal::SIGTERM).unwrap();

    let status = cmd.wait().await.map_err(|e| e.to_string()).unwrap();
    status
        .signal()
        .map(|s| format!("signal: {:?}", s))
        .ok_or("not signaled".to_string())
}
