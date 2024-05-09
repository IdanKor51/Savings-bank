pub fn read_inp() -> Result<String, i32>{
    let mut buffer = String::new();
    let stdin = std::io::stdin(); // We get `Stdin` here.
    let _ = stdin.read_line(&mut buffer);

    // get rid of the \n\r at the end of the string
    let buffer = buffer
        .chars()
        .filter(|&x| x != '\n' && x != '\r')
        .collect();
    
    println!("-{}-", buffer);
    Ok(buffer)
}
