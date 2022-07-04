use anyhow::Result;
use flate2::read::ZlibDecoder;
use log::trace;
use lz4::block::decompress_to_buffer as LZ4_decompress_to_buffer;
use lz4::Decoder as LZ4Decoder;
use std::io::Read;
use xz2::read::XzDecoder;

// Note: this contains ZL[src][dst] where src and dst are 3 bytes each.
const HEADER_SIZE: usize = 9;

// because each zipped block contains:
// - the size of the input data
// - the size of the compressed data
// where each size is saved on 3 bytes, the maximal size
// of each block can not be bigger than 16Mb.
const K_MAX_COMPRESSED_BLOCK_SIZE: usize = 0xffffff;

enum Kind {
    Inherit = -1,
    UseGlobal = 0,
    ZLIB = 1,
    LZMA = 2,
    OldCompression = 3,
    LZ4 = 4,
    ZSTD = 5,
    UndefinedCompression = 6,
}

// kindOf returns the kind of compression algorithm.
fn kind_of(buf: &[u8]) -> Kind {
    match (buf[0] as char, buf[1] as char) {
        ('Z', 'L') => Kind::ZLIB,
        ('X', 'Z') => Kind::LZMA,
        ('L', '4') => Kind::LZ4,
        ('Z', 'S') => Kind::ZSTD,
        ('C', 'S') => Kind::OldCompression,

        _ => Kind::UndefinedCompression,
    }
}

pub fn decompress(dst: &mut [u8], mut src: &[u8]) -> Result<usize> {
    let _beg = 0;
    let mut end = 0;
    let buflen = dst.len() as i64;
    let mut hdr = [0 as u8; HEADER_SIZE];

    // let mut v =

    while end < buflen {
        // let src = src.as_mut();
        let _ = src.read_exact(&mut hdr)?;
        // let _ = src.read_exact(dst)?;
        // let _ = src.read_exact(dst)?;
        trace!("decompress: hdr = {:?}", hdr);

        let _srcsz = hdr[3] as i64 | (hdr[4] as i64) << 8 | (hdr[5] as i64) << 16;
        let tgtsz = hdr[6] as i64 | (hdr[7] as i64) << 8 | (hdr[8] as i64) << 16;
        // let tgtsz = hdr[6]) | int64(hdr[7])<<8 | int64(hdr[8])<<16
        end += tgtsz;
        trace!("end = {}", end);

        match kind_of(hdr.as_ref()) {
            Kind::Inherit => {
                unimplemented!()
            }
            Kind::UseGlobal => {
                unimplemented!()
            }

            Kind::ZLIB => {
                trace!("case zlib");
                let mut d = ZlibDecoder::new(src);
                d.read_exact(dst.as_mut())?;
                return Ok(0);
            }
            Kind::LZMA => {
                let mut d = XzDecoder::new(src);
                d.read_exact(dst.as_mut())?;
            }
            Kind::OldCompression => {
                unimplemented!()
            }
            Kind::LZ4 => {
                LZ4_decompress_to_buffer(&src[8..], Some(dst.len() as i32), dst.as_mut())?;
                return Ok(0);
                // let mut d = LZ4Decoder::new(src)?;
                // d.read_exact(dst.as_mut())?;
            }
            Kind::ZSTD => {
                unimplemented!()
            }
            Kind::UndefinedCompression => {
                unimplemented!()
            }
        }
    }

    Ok(0)
}
