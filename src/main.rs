use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();

    let mut buffer = [0; 4096];
    let mut bom_checked = false;
    let mut bytes_read;

    loop {
        bytes_read = stdin.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        if !bom_checked {
            if buffer[..bytes_read].starts_with(&[0xEF, 0xBB, 0xBF]) {
                stdout.write_all(&buffer[3..bytes_read])?;
            } else {
                stdout.write_all(&buffer[..bytes_read])?;
            }
            bom_checked = true;
        } else {
            stdout.write_all(&buffer[..bytes_read])?;
        }
    }

    Ok(())
}
