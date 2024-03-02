use crate::riofs::Error;

pub fn decode_name_cycle(namecycle: &str) -> Result<(&str, u16), Error> {
    let toks: Vec<_> = namecycle.split(';').collect();

    match toks.len() {
        1 => Ok((toks[0], 9999)),
        2 => {
            unimplemented!()
        }

        _ => Err(Error::CantDecodeNameCycle(namecycle.to_string())),
    }
}
