use std::convert::TryInto;
use std::io::Read;
use nom::IResult;
use std::fs::File;
use std::env;

const RIFF_MAGIC: &[u8] = &[0x52, 0x49, 0x46, 0x46];

fn main() -> std::io::Result<()> {
    let args : Vec<String> = env::args().collect();

    if(args.len() < 2) {
        println!("No path to file provided");
        return Ok(());
    }

    let mut f = File::open(&args[1])?;
    let mut buf: Vec<u8> = Vec::new();
    let _ = f.read_to_end(&mut buf);

    let zz = buf.as_slice();
    parse_webp(zz);

    Ok(())
}

fn riff_magic(input: &[u8]) -> nom::IResult<&[u8], &[u8]> {
    let res = nom::bytes::complete::tag(RIFF_MAGIC)(input);
    if(res.is_err()){
        panic!("Not a RIFF file");
    }

    return res;
}

fn take_4(i: &[u8]) -> IResult<&[u8], &[u8]> {
    nom::bytes::complete::take(4u8)(i)
  }

fn parse_webp(input: &[u8]) {
    let mut size_parser = nom::sequence::preceded(
        riff_magic,
        take_4
    );

    let x = size_parser(input).ok().expect("Cannot get size of webp file").1;

    let zz = u32::from_le_bytes(get_four_byte_arr(x));
    println!("{}", zz);
}

fn get_four_byte_arr(arr: &[u8]) -> [u8; 4] {
    return arr.try_into().expect("slice with incorrect length");
}