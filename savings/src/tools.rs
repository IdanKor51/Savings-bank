pub fn read_inp() -> Result<String, i32>{
    let mut buffer = String::new();
    let stdin = std::io::stdin(); // We get `Stdin` here.
    let _ = stdin.read_line(&mut buffer);
    Ok(buffer)
}