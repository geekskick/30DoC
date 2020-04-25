use std::convert::TryFrom;
use std::convert::TryInto;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io::Read;
use std::str;

pub struct Config<'a> {
    fname: &'a String,
}

pub fn parse_args(args: &[String]) -> Result<Config, &'static str> {
    if args.len() != 2 {
        return Err("Incorrect number of arguments");
    }

    Ok(Config { fname: &args[1] })
}

pub fn run(cfg: &Config) -> Result<(), &'static str> {
    Err("misc error")
}

#[derive(PartialEq, Debug)]
enum WaveFormat {
    Pcm,
    Float,
    Alaw,
    Mulaw,
    Extensible,
}

impl Default for WaveFormat {
    fn default() -> Self {
        WaveFormat::Float
    }
}

impl std::convert::TryFrom<u16> for WaveFormat {
    type Error = WavReadError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(WaveFormat::Pcm),
            _ => Err(WavReadError::Header(WavHeaderError::UnrecognisedWaveFormat)),
        }
    }
}

#[derive(PartialEq, Debug)]
struct Hertz(u32);
impl Default for Hertz {
    fn default() -> Self {
        Hertz(0)
    }
}

#[derive(PartialEq, Debug)]
enum WavHeaderError {
    Riff,
    Wave,
    Format,
    FormatLength,
    UnrecognisedWaveFormat,
}
#[derive(Debug, PartialEq)]
enum WavReadError {
    FileIO(std::io::ErrorKind),
    DataConvertToString(std::str::Utf8Error),
    Header(WavHeaderError),
}

impl fmt::Display for WavReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error reading from WAV file")
    }
}
impl Error for WavReadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

impl std::convert::From<std::io::Error> for WavReadError {
    fn from(value: std::io::Error) -> Self {
        WavReadError::FileIO(value.kind())
    }
}

impl std::convert::From<std::str::Utf8Error> for WavReadError {
    fn from(value: std::str::Utf8Error) -> Self {
        WavReadError::DataConvertToString(value)
    }
}

#[derive(PartialEq, Debug)]
struct RiffHeader {
    file_size: u32,
}

impl Default for RiffHeader {
    fn default() -> Self {
        RiffHeader {
            file_size: 0,
        }
    }
}

#[derive(PartialEq, Debug)]
struct WaveHeader {
    riff: RiffHeader,
    sample_rate: Hertz,
    audio_format: WaveFormat,
    num_channels: u16,
    byte_rate: Hertz,
    bits_per_sample: u16,
    block_align: u16,
    data_size: u16,
}

impl Default for WaveHeader {
    fn default() -> Self {
        WaveHeader {
            riff: RiffHeader::default(),
            sample_rate: Hertz::default(),
            audio_format: WaveFormat::default(),
            bits_per_sample: 0,
            byte_rate: Hertz::default(),
            block_align: 0,
            data_size: 0,
            num_channels: 0,
        }
    }
}

impl std::convert::TryFrom<&mut std::fs::File> for RiffHeader {
    type Error = WavReadError;
    fn try_from(f: &mut std::fs::File) -> Result<Self, Self::Error> {
        let mut buf: [u8; 4] = [0; 4];
        f.read_exact(&mut buf)?;

        let header = std::str::from_utf8(&buf)?;
        if header != "RIFF" {
            return Err(WavReadError::Header(WavHeaderError::Riff));
        }

        let mut rc = RiffHeader::default();
        f.read_exact(&mut buf)?;
        rc.file_size = u32::from_le_bytes(buf);

        f.read_exact(&mut buf)?;
        let wave = std::str::from_utf8(&buf)?;
        if wave != "WAVE" {
            return Err(WavReadError::Header(WavHeaderError::Wave));
        }

        Ok(rc)
    }
}

impl std::convert::From<RiffHeader> for WaveHeader {
    fn from(riff: RiffHeader) -> Self {
        let mut rc = WaveHeader::default();
        rc.riff = riff;
        rc
    }
}

impl std::convert::TryFrom<&mut std::fs::File> for WaveHeader {
    type Error = WavReadError;
    fn try_from(f: &mut std::fs::File) -> Result<Self, Self::Error> {
        let riff = RiffHeader::try_from(f.by_ref())?;
        let mut rc = WaveHeader::from(riff);
        let mut buf: [u8; 4] = [0; 4];

        f.read_exact(&mut buf)?;
        let fmt = std::str::from_utf8(&buf)?;
        if fmt != "fmt " {
            return Err(WavReadError::Header(WavHeaderError::Format));
        }

        f.read_exact(&mut buf)?;
        let fmt_len = u32::from_le_bytes(buf);
        if fmt_len != 16 {
            return Err(WavReadError::Header(WavHeaderError::FormatLength));
        }

        f.read_exact(&mut buf)?;
        // Fallible conversion from the u16::from_le_bytes page
        let (af_bytes, num_ch_bytes) = buf.split_at(std::mem::size_of::<u16>());
        rc.audio_format = WaveFormat::try_from(u16::from_le_bytes(af_bytes.try_into().unwrap()))?;

        rc.num_channels = u16::from_le_bytes(num_ch_bytes.try_into().unwrap());

        f.read_exact(&mut buf)?;
        rc.sample_rate = Hertz(u32::from_le_bytes(buf));

        f.read_exact(&mut buf)?;
        rc.byte_rate = Hertz(u32::from_le_bytes(buf));

        f.read_exact(&mut buf)?;
        let (block_bytes, bit_bytes) = buf.split_at(std::mem::size_of::<u16>());
        rc.block_align = u16::from_le_bytes(block_bytes.try_into().unwrap());
        rc.bits_per_sample = u16::from_le_bytes(bit_bytes.try_into().unwrap());

        Ok(rc)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_args_test() {
        let args = ["Progname".to_string()];
        let cfg = parse_args(&args);
        assert!(cfg.is_err());

        let args = ["Progname".to_string(), "arg".to_string()];
        let cfg = parse_args(&args).unwrap();
        assert_eq!(&args[1], cfg.fname);
        assert_eq!(args[1], *cfg.fname);

        let args = ["one".to_string(), "arg".to_string(), "extra".to_string()];
        let cfg = parse_args(&args);
        assert!(cfg.is_err());
    }

    fn get_full_path_of_test_resource(rname: &str) -> std::path::PathBuf {
        let s = std::env::var("CARGO_MANIFEST_DIR")
            .unwrap_or_else(|_| String::from(std::env::current_dir().unwrap().to_str().unwrap()))
            + "/"
            + rname;
        std::path::PathBuf::from(&s)
    }

    fn get_metadata(filename: &str) -> Result<WaveHeader, WavReadError> {
        let path = get_full_path_of_test_resource(filename);
        let mut f = std::fs::File::open(path)?;
        WaveHeader::try_from(&mut f)
    }

    #[test]
    fn read_wave_file_test() {
        let metadata = get_metadata("doesn't exist");
        assert_eq!(
            metadata.err().unwrap(),
            WavReadError::FileIO(std::io::ErrorKind::NotFound)
        );

        let metadata = get_metadata("test_input_no_riff.wav");
        assert_eq!(
            metadata.err().unwrap(),
            WavReadError::Header(WavHeaderError::Riff)
        );

        let metadata = get_metadata("test_input_no_wave.wav");
        assert_eq!(
            metadata.err().unwrap(),
            WavReadError::Header(WavHeaderError::Wave)
        );

        let metadata = get_metadata("test_input_no_fmt.wav");
        assert_eq!(
            metadata.err().unwrap(),
            WavReadError::Header(WavHeaderError::Format)
        );

        let metadata = get_metadata("test_input_no_fmt_len.wav");
        assert_eq!(
            metadata.err().unwrap(),
            WavReadError::Header(WavHeaderError::FormatLength)
        );

        let metadata = get_metadata("test_input_wrong_fmt.wav");
        assert_eq!(
            metadata.err().unwrap(),
            WavReadError::Header(WavHeaderError::UnrecognisedWaveFormat)
        );

        let metadata = get_metadata("test_input.wav");
        let mut expected = WaveHeader::default();
        expected.riff.file_size = 636_998;
        expected.audio_format = WaveFormat::Pcm;
        expected.num_channels = 1;
        expected.sample_rate = Hertz(8000);
        expected.byte_rate = Hertz(16000);
        expected.block_align = 2;
        expected.bits_per_sample = 16;

        assert_eq!(metadata.unwrap(), expected);
    }
}
