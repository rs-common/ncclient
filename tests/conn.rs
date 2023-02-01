use ncclient::{error::Error, ssh::SSH};

#[test]
fn new() -> Result<(), Error> {
    let _ = SSH::new("172.22.100.1:830", "lyonsdpy", "Dpy870717", None)?;
    Ok(())
}
