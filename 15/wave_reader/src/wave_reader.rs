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
struct PcmWaveFile {
    pub fname: String,
    pub sample_rate: Hertz,
    pub audio_format: WaveFormat,
    pub num_channels: u16,
    pub byte_rate: Hertz,
    pub bits_per_sample: u16,
    pub block_align: u16,
    pub data_size: u32,
    pub file_size: u32,
}

impl PcmWaveFile {
    pub fn default() -> Self {
        PcmWaveFile {
            fname: "Default".to_string(),
            sample_rate: Hertz(8000),
            audio_format: WaveFormat::Float,
            num_channels: 0,
            byte_rate: Hertz(1),
            bits_per_sample: 0,
            block_align: 0,
            data_size: 0,
            file_size: 0,
        }
    }
}

fn check_riff_chunk(f: &mut std::fs::File) -> Result<PcmWaveFile, WavReadError> {
    let mut buf: [u8; 4] = [0; 4];
    f.read_exact(&mut buf)?;

    let header = std::str::from_utf8(&buf)?;
    if header != "RIFF" {
        return Err(WavReadError::Header(WavHeaderError::Riff));
    }

    let mut rc = PcmWaveFile::default();
    f.read_exact(&mut buf)?;
    rc.file_size = u32::from_le_bytes(buf);

    f.read_exact(&mut buf)?;
    let wave = std::str::from_utf8(&buf)?;
    if wave != "WAVE" {
        return Err(WavReadError::Header(WavHeaderError::Wave));
    }

    Ok(rc)
}

fn parse_fmt_chunk(f: &mut std::fs::File, riff: PcmWaveFile) -> Result<PcmWaveFile, WavReadError> {
    let mut buf: [u8; 4] = [0; 4];

    let mut rc = PcmWaveFile::default();
    rc.fname = riff.fname;
    rc.file_size = riff.file_size;

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
impl std::convert::TryFrom<&str> for PcmWaveFile {
    type Error = WavReadError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut f = fs::File::open(value)?;
        let riff_data_only = check_riff_chunk(&mut f)?;
        let riff_and_wav_fmt_data = parse_fmt_chunk(&mut f, riff_data_only)?;
        Ok(riff_and_wav_fmt_data)
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

    fn get_full_path_of_test_resource(rname: &str) -> String {
        std::env::var("CARGO_MANIFEST_DIR")
            .unwrap_or_else(|_| String::from(std::env::current_dir().unwrap().to_str().unwrap()))
            + "/"
            + rname
    }

    fn get_metadata(filename: &str) -> Result<PcmWaveFile, WavReadError> {
        let path = get_full_path_of_test_resource(filename);
        PcmWaveFile::try_from(path.as_str())
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
        let mut expected = PcmWaveFile::default();
        expected.file_size = 636_998;
        expected.audio_format = WaveFormat::Pcm;
        expected.num_channels = 1;
        expected.sample_rate = Hertz(8000);
        expected.byte_rate = Hertz(16000);
        expected.block_align = 2;
        expected.bits_per_sample = 16;

        assert_eq!(metadata.unwrap(), expected);
    }
}
