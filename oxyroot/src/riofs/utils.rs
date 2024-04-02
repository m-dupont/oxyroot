use crate::riofs::Error;
use chrono::{DateTime, Datelike, Timelike, Utc};

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

pub(crate) fn now() -> DateTime<Utc> {
    //Utc::now()
    // FIXME: use chrono::Utc::now() when debug is done
    DateTime::from_timestamp(1657861095, 0).unwrap()
}

pub(crate) fn datetime_to_u32(datetime: DateTime<Utc>) -> u32 {
    let year = datetime.year() as u32;
    let month = datetime.month();
    let day = datetime.day();
    let hour = datetime.hour();
    let minute = datetime.minute();
    let second = datetime.second();

    if year < 1995 {
        panic!("year {} is less than 1995", year);
    }

    ((year - 1995) << 26) | (month << 22) | (day << 17) | (hour << 12) | (minute << 6) | second
}
