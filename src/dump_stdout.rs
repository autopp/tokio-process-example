use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, BufReader},
    process::Command,
};

pub async fn dump_stdout() -> Result<String, String> {
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
    let mut buf = String::new();
    loop {
        let mut line = String::new();
        reader.read_line(&mut line).await.unwrap();

        println!("{}", line.trim());

        if line == "3\n" {
            break;
        }
        buf.push_str(&line);
    }

    cmd.wait().await.map(|_| ()).map_err(|e| e.to_string())?;
    reader
        .read_to_string(&mut buf)
        .await
        .map_err(|e| e.to_string())?;

    Ok(buf)
}
