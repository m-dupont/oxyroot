use flate2::read::ZlibDecoder;
use lz4::block::decompress_to_buffer as LZ4_decompress_to_buffer;
use std::io::Read;
use xz2::read::XzDecoder;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

// Note: this contains ZL[src][dst] where src and dst are 3 bytes each.
const HEADER_SIZE: usize = 9;

// because each zipped block contains:
// - the size of the input data
// - the size of the compressed data
// where each size is saved on 3 bytes, the maximal size
// of each block can not be bigger than 16Mb.
#[allow(dead_code)]
const K_MAX_COMPRESSED_BLOCK_SIZE: usize = 0xffffff;

#[allow(dead_code)]
#[derive(PartialEq)]
enum Kind {
    #[allow(dead_code)]
    Inherit = -1,
    UseGlobal = 0,
    Zlib = 1,
    Lzma = 2,
    OldCompression = 3,
    LZ4 = 4,
    Zstd = 5,
    UndefinedCompression = 6,
}

// kindOf returns the kind of compression algorithm.
fn kind_of(buf: &[u8]) -> Kind {
    match (buf[0] as char, buf[1] as char) {
        ('Z', 'L') => Kind::Zlib,
        ('X', 'Z') => Kind::Lzma,
        ('L', '4') => Kind::LZ4,
        ('Z', 'S') => Kind::Zstd,
        ('C', 'S') => Kind::OldCompression,

        _ => Kind::UndefinedCompression,
    }
}

pub fn decompress(dst: &mut [u8], mut src: &[u8]) -> Result<usize> {
    let _beg = 0;
    let mut end = 0;
    let buflen = dst.len() as i64;
    let mut hdr = [0_u8; HEADER_SIZE];

    // let mut v =

    while end < buflen {
        // let src = src.as_mut();
        src.read_exact(&mut hdr)?;
        // let _ = src.read_exact(dst)?;
        // let _ = src.read_exact(dst)?;

        let _srcsz = hdr[3] as i64 | (hdr[4] as i64) << 8 | (hdr[5] as i64) << 16;
        let tgtsz = hdr[6] as i64 | (hdr[7] as i64) << 8 | (hdr[8] as i64) << 16;
        // let tgtsz = hdr[6]) | int64(hdr[7])<<8 | int64(hdr[8])<<16
        end += tgtsz;

        match kind_of(hdr.as_ref()) {
            Kind::Inherit => {
                unimplemented!()
            }
            Kind::UseGlobal => {
                unimplemented!()
            }

            Kind::Zlib => {
                let mut d = ZlibDecoder::new(src);
                d.read_exact(dst.as_mut())?;
                return Ok(0);
            }
            Kind::Lzma => {
                let mut d = XzDecoder::new(src);
                d.read_exact(dst.as_mut())?;
            }
            Kind::OldCompression => {
                unimplemented!()
            }
            Kind::LZ4 => {
                LZ4_decompress_to_buffer(&src[8..], Some(dst.len() as i32), dst)?;
                return Ok(0);
                // let mut d = LZ4Decoder::new(src)?;
                // d.read_exact(dst.as_mut())?;
            }
            Kind::Zstd => {
                unimplemented!()
            }
            Kind::UndefinedCompression => {
                unimplemented!()
            }
        }
    }

    Ok(0)
}

fn root_compress_algo_level(algo: i32) -> (Kind, i32) {
    let kind = algo / 100;
    let level = algo % 100;
    let kind = match kind {
        0 => Kind::UseGlobal,
        1 => Kind::Zlib,
        2 => Kind::Lzma,
        3 => Kind::OldCompression,
        4 => Kind::LZ4,
        5 => Kind::Zstd,
        _ => Kind::UndefinedCompression,
    };
    (kind, level)
}

pub fn compress(src: Vec<u8>, compression: i32) -> Result<Vec<u8>> {
    assert_eq!(compression, 1);

    let (kind, level) = root_compress_algo_level(compression);

    if kind == Kind::UseGlobal || level == 0 || src.len() < 512 {
        // no compression
        // let mut dst = Vec::new();
        //std::io::copy(src, &mut dst)?;
        return Ok(src);
    }

    unimplemented!();

    // let mut dst = Vec::new();
    // {
    //     let mut w = flate2::write::ZlibEncoder::new(&mut dst, flate2::Compression::default());
    //     std::io::copy(&mut src, &mut w)?;
    // }
    //
    // Ok(dst)
}
