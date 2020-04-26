use std::convert::Into;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::error::Error;
use std::fmt;
use std::io::Read;
use std::io::Seek;
use std::str;

use prettytable::{Row, Table};

#[derive(PartialEq, Debug)]
pub struct Config<'a> {
    fname: &'a String,
}

#[derive(PartialEq, Debug)]
pub enum ConfigParseError {
    NotEnoughArgs,
    TooManyArgs,
}

impl<'a> std::convert::TryFrom<&'a [String]> for Config<'a> {
    type Error = ConfigParseError;
    fn try_from(args: &'a [String]) -> Result<Self, Self::Error> {
        if args.len() < 2 {
            return Err(ConfigParseError::NotEnoughArgs);
        }
        if args.len() > 2 {
            return Err(ConfigParseError::TooManyArgs);
        }

        Ok(Config { fname: &args[1] })
    }
}

impl Error for ConfigParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

impl fmt::Display for ConfigParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            ConfigParseError::TooManyArgs => "Too many arguments passed",
            ConfigParseError::NotEnoughArgs => "Not enough args passed",
        };
        write!(f, "{}", s)
    }
}
pub fn run(cfg: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut f = std::fs::File::open(cfg.fname)?;
    let wh = WaveFile::try_from(&mut f)?;
    println!("{}", wh);
    Ok(())
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

impl fmt::Display for WaveFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: &'static str = match self {
            WaveFormat::Alaw => "ALAW",
            WaveFormat::Pcm => "PCM",
            WaveFormat::Float => "Float",
            WaveFormat::Mulaw => "MULAW",
            WaveFormat::Extensible => "Extensible",
        };
        write!(f, "{}", s)
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

impl fmt::Display for Hertz {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Hertz(h) = self;
        write!(f, "{} Hz", h)
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

impl std::convert::From<&WavHeaderError> for &'static str {
    fn from(whe: &WavHeaderError) -> &'static str {
        match whe {
            WavHeaderError::Format => "'fmt ' not present", 
            WavHeaderError::FormatLength => "Format chunk size expected to be fixed at 16, but this wasn't in the file, or it wasn't in the right place",
            WavHeaderError::Riff => "'RIFF' wasn't at the start of the file",
            WavHeaderError::Wave => "'WAVE' wasn't in the right place of the header",
            WavHeaderError::UnrecognisedWaveFormat => "Expected the file to by a PCM format, but it wasn't"
        }
    }
}
#[derive(Debug, PartialEq)]
enum WavReadError {
    FileIO(std::io::ErrorKind),
    DataConvertToString(std::str::Utf8Error),
    Header(WavHeaderError),
}

impl fmt::Display for WavReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let specific: &'static str = match self {
            WavReadError::FileIO(_) => "FileIO error and unable to read from the file. Ensure it has a valid 'data' subchunk",
            WavReadError::DataConvertToString(_) => "Error converting Data to string",
            WavReadError::Header(err) => err.into(),
        };
        write!(f, "Error reading from WAV file: {}", specific)
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
        RiffHeader { file_size: 0 }
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

fn advance_to_data_subchunk(f: &mut std::fs::File) -> Result<(), WavReadError> {
    let mut buf: [u8; 4] = [0; 4];
    loop {
        f.read_exact(&mut buf)?;
        let subchunk2_name = std::str::from_utf8(&buf)?.to_owned();

        f.read_exact(&mut buf)?;
        let subchunk2_len = u32::from_le_bytes(buf);

        if subchunk2_name != "data" {
            // There is a number of optional
            f.seek(std::io::SeekFrom::Current(
                subchunk2_len.try_into().unwrap(),
            ))?;
        } else {
            return Ok(());
        }
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

        advance_to_data_subchunk(f)?;
        Ok(rc)
    }
}
impl fmt::Display for WaveHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut tbl = Table::new();
        tbl.set_titles(Row::from(vec!["Section", "Data"]));
        tbl.add_row(Row::from(vec![
            "File Size",
            &(self.riff.file_size.to_string() + " bytes"),
        ]));
        tbl.add_row(Row::from(vec![
            "Audio Format",
            &self.audio_format.to_string(),
        ]));
        tbl.add_row(Row::from(vec![
            "Number of Channels",
            &self.num_channels.to_string(),
        ]));
        tbl.add_row(Row::from(vec![
            "Sample Rate",
            &self.sample_rate.to_string(),
        ]));
        tbl.add_row(Row::from(vec!["Byte Rate", &self.byte_rate.to_string()]));
        tbl.add_row(Row::from(vec![
            "Block Align",
            &self.block_align.to_string(),
        ]));
        tbl.add_row(Row::from(vec![
            "Bits per sample",
            &self.bits_per_sample.to_string(),
        ]));
        write!(f, "{}", tbl)
    }
}

#[derive(Debug)]
struct WaveFile {
    header: WaveHeader,
    ldata: Vec<u16>,
    rdata: Vec<u16>,
}

impl Default for WaveFile {
    fn default() -> Self {
        WaveFile {
            header: WaveHeader::default(),
            ldata: Vec::new(),
            rdata: Vec::new(),
        }
    }
}

impl std::convert::From<WaveHeader> for WaveFile {
    fn from(h: WaveHeader) -> Self {
        let mut rc = WaveFile::default();
        rc.header = h;
        rc
    }
}

impl std::convert::TryFrom<&mut std::fs::File> for WaveFile {
    type Error = WavReadError;
    fn try_from(f: &mut std::fs::File) -> Result<Self, Self::Error> {
        let header = WaveHeader::try_from(f.by_ref())?;
        let mut rc = WaveFile::from(header);
        let mut samples = Vec::new();
        f.read_to_end(&mut samples)?;

        // This is shit! There must be a rustier way of getting them out
        for (idx, sample) in samples.iter().enumerate() {
            if idx % 2 == 0 {
                let bo = [*sample, samples[idx + 1]];
                rc.ldata.push(u16::from_le_bytes(bo));
            }
        }
        Ok(rc)
    }
}

impl fmt::Display for WaveFile{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", self.header)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_args_test_not_enough() {
        let args = vec!["Progname".to_string()];
        let cfg = Config::try_from(args.as_slice());

        assert_eq!(cfg.err().unwrap(), ConfigParseError::NotEnoughArgs);
    }

    #[test]
    fn parse_args_test_just_right() {
        let args = vec!["Progname".to_string(), "arg".to_string()];
        let cfg = Config::try_from(args.as_slice()).unwrap();
        assert_eq!(&args[1], cfg.fname);
        assert_eq!(args[1], *cfg.fname);
    }

    #[test]
    fn parse_args_test_too_many() {
        let args = vec!["one".to_string(), "arg".to_string(), "extra".to_string()];
        let cfg = Config::try_from(args.as_slice());
        assert_eq!(cfg.err().unwrap(), ConfigParseError::TooManyArgs);
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
    fn read_wave_file_test_file_not_found() {
        let metadata = get_metadata("doesn't exist");
        assert_eq!(
            metadata.err().unwrap(),
            WavReadError::FileIO(std::io::ErrorKind::NotFound)
        );
    }
    #[test]
    fn read_wave_file_test_file_no_riff() {
        let metadata = get_metadata("test_input_no_riff.wav");
        assert_eq!(
            metadata.err().unwrap(),
            WavReadError::Header(WavHeaderError::Riff)
        );
    }

    #[test]
    fn read_wave_file_test_file_no_wave() {
        let metadata = get_metadata("test_input_no_wave.wav");
        assert_eq!(
            metadata.err().unwrap(),
            WavReadError::Header(WavHeaderError::Wave)
        );
    }

    #[test]
    fn read_wave_file_test_file_no_fmt() {
        let metadata = get_metadata("test_input_no_fmt.wav");
        assert_eq!(
            metadata.err().unwrap(),
            WavReadError::Header(WavHeaderError::Format)
        );
    }

    #[test]
    fn read_wave_file_test_file_no_fmt_len() {
        let metadata = get_metadata("test_input_no_fmt_len.wav");
        assert_eq!(
            metadata.err().unwrap(),
            WavReadError::Header(WavHeaderError::FormatLength)
        );
    }

    #[test]
    fn read_wave_file_test_file_wrong_fmt() {
        let metadata = get_metadata("test_input_wrong_fmt.wav");
        assert_eq!(
            metadata.err().unwrap(),
            WavReadError::Header(WavHeaderError::UnrecognisedWaveFormat)
        );
    }

    #[test]
    fn read_wave_file_test_file_no_data() {
        let metadata = get_metadata("test_input_no_data.wav");
        assert_eq!(
            metadata.err().unwrap(),
            WavReadError::FileIO(std::io::ErrorKind::UnexpectedEof)
        );
    }

    #[test]
    fn read_wave_file_test_file_success() {
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

    #[test]
    fn read_data_from_wave_file() {
        let file = get_full_path_of_test_resource("test_input_short.wav");
        let mut file = std::fs::File::open(file).unwrap();
        let wave = WaveFile::try_from(&mut file);
        assert!(wave.is_ok());
        let wave = wave.unwrap();
        assert_eq!(wave.ldata, [0, 1, 2, 3, 4, 5]);
    }
}
