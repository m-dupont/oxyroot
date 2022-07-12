use anyhow::{bail, Result};

pub fn decode_name_cycle(namecycle: &str) -> Result<(&str, u16)> {
    let toks: Vec<_> = namecycle.split(';').collect();

    match toks.len() {
        1 => Ok((toks[0], 9999)),
        2 => {
            unimplemented!()
        }

        _ => {
            bail!("Invalid namecycle")
        }
    }
}
