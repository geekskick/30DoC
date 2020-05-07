use std::convert::Into;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::error::Error;
use std::fmt;
use std::io::Read;
use std::io::Seek;
use std::str;

use byteorder::{LittleEndian, ReadBytesExt};
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
    println!("{}", parse(&wh.ldata, 3000));
    Ok(())
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Symbol {
    Short,
    Long,
    Space,
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
    ldata: Vec<i16>,
    rdata: Vec<i16>,
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

        rc.ldata = samples
            .chunks_exact(2)
            .step_by(rc.header.num_channels as usize)
            .map(|mut byte_pair| byte_pair.read_i16::<LittleEndian>().unwrap_or(0))
            .collect();

        if rc.header.num_channels == 2 {
            rc.rdata = samples
                .chunks_exact(2)
                .skip(1)
                .step_by(2)
                .map(|mut byte_pair| byte_pair.read_i16::<LittleEndian>().unwrap_or(0))
                .collect()
        }

        Ok(rc)
    }
}

impl fmt::Display for WaveFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.header)
    }
}

#[derive(Debug)]
struct ParserStatus {
    over_threshold: u64,
    under_threshold: u64,
}

impl Default for ParserStatus {
    fn default() -> ParserStatus {
        ParserStatus {
            over_threshold: 0,
            under_threshold: 0,
        }
    }
}

impl ParserStatus {
    // End of a long/short
    fn is_end_of_symbol(&self) -> bool {
        // Small gap immediately after some stuff over the threshold
        // This was measured in audacity
        self.under_threshold > 3 && self.under_threshold <= 1000 && self.over_threshold != 0
    }

    fn over(&mut self) {
        self.over_threshold += 1;
        self.under_threshold = 0;
    }

    fn under(&mut self) {
        self.under_threshold += 1;
    }

    fn reset_under(&mut self) {
        self.under_threshold = 0;
    }

    fn reset_over(&mut self) {
        self.over_threshold = 0;
    }

    fn is_end_of_letter(&self) -> bool {
        // Big gap at the end of the
        self.under_threshold > 1000 && self.under_threshold <= 2000 && self.over_threshold == 0
    }

    fn is_space(&self) -> bool {
        // Bigger gap
        self.under_threshold > 2000 && self.over_threshold == 0
    }
}

fn parse(samples: &[i16], threshold: i16) -> String {
    let mut status = ParserStatus::default();
    let mut symbols: Vec<Symbol> = Vec::new();
    let mut rc = String::new();
    for sample in samples {
        let abs = sample.checked_abs().unwrap_or(threshold + 1);

        if abs > threshold {
            status.over();
        } else if abs < threshold {
            status.under();
        }

        if status.is_end_of_symbol() {
            // If the symbol doesn't successfully parse then it's less than a
            // unit (short). In which case assume it's an erroneous blip in the sinwave
            // so go to pretend it never happened by just going to the next sample
            let symbol = Symbol::try_from(status.over_threshold);

            status.reset_over();
            if symbol.is_err() {
                continue;
            }

            symbols.push(symbol.unwrap());
        } else if status.is_end_of_letter() && !symbols.is_empty() {
            // If it's the end of the word I can try to decode the symbols gathered
            rc += decode(&symbols);
            symbols.clear();
        } else if status.is_space() {
            // If it's a really long gap I can add a space to the output string
            // Make sure to reset the under threshold count as there has been a big gap, so I'm not really bothered
            // about anything until the end of the next big gap.

            // If there is loads of gap it might be that the operator is having a break etc. In which case I don't
            // want loads of spaces in the string, so only add spaces if there isn't already
            if !rc.ends_with(' ') {
                rc += " ";
            }
            status.reset_under();
        }
    }

    if !symbols.is_empty() {
        // probably the end of a word since it's the end of the file so
        // try to decode it. No guarantee that the final symbol has ended with a
        // long enough gap to trigger an end of symbol decode
        rc += decode(&symbols);
        symbols.clear();
    }
    rc.trim_end().to_owned()
}

fn decode(morse: &[Symbol]) -> &'static str {
    // WTF is this formatting?
    // the vecs aren't a good idea I don't think
    let map: std::collections::HashMap<Vec<Symbol>, &'static str> = [
        (vec![Symbol::Short, Symbol::Long], "a"),
        (
            vec![Symbol::Long, Symbol::Short, Symbol::Short, Symbol::Short],
            "b",
        ),
        (
            vec![Symbol::Long, Symbol::Short, Symbol::Long, Symbol::Short],
            "c",
        ),
        (vec![Symbol::Long, Symbol::Short, Symbol::Short], "d"),
        (vec![Symbol::Short], "e"),
        (
            vec![Symbol::Short, Symbol::Short, Symbol::Long, Symbol::Short],
            "f",
        ),
        (vec![Symbol::Long, Symbol::Long, Symbol::Short], "g"),
        (
            vec![Symbol::Short, Symbol::Short, Symbol::Short, Symbol::Short],
            "h",
        ),
        (vec![Symbol::Short, Symbol::Short], "i"),
        (
            vec![Symbol::Short, Symbol::Long, Symbol::Long, Symbol::Long],
            "j",
        ),
        (vec![Symbol::Long, Symbol::Short, Symbol::Long], "k"),
        (
            vec![Symbol::Short, Symbol::Long, Symbol::Short, Symbol::Short],
            "l",
        ),
        (vec![Symbol::Long, Symbol::Long], "m"),
        (vec![Symbol::Long, Symbol::Short], "n"),
        (vec![Symbol::Long, Symbol::Long, Symbol::Long], "o"),
        (
            vec![Symbol::Short, Symbol::Long, Symbol::Long, Symbol::Short],
            "p",
        ),
        (
            vec![Symbol::Long, Symbol::Long, Symbol::Short, Symbol::Long],
            "q",
        ),
        (vec![Symbol::Short, Symbol::Long, Symbol::Short], "r"),
        (vec![Symbol::Short, Symbol::Short, Symbol::Short], "s"),
        (vec![Symbol::Long], "t"),
        (vec![Symbol::Short, Symbol::Short, Symbol::Long], "u"),
        (
            vec![Symbol::Short, Symbol::Short, Symbol::Short, Symbol::Long],
            "v",
        ),
        (vec![Symbol::Short, Symbol::Long, Symbol::Long], "w"),
        (
            vec![Symbol::Long, Symbol::Short, Symbol::Short, Symbol::Long],
            "x",
        ),
        (
            vec![Symbol::Long, Symbol::Short, Symbol::Long, Symbol::Long],
            "y",
        ),
        (
            vec![Symbol::Long, Symbol::Long, Symbol::Short, Symbol::Short],
            "z",
        ),
        (vec![Symbol::Space], " "),
        (
            vec![
                Symbol::Long,
                Symbol::Long,
                Symbol::Long,
                Symbol::Long,
                Symbol::Long,
            ],
            "0",
        ),
        (
            vec![
                Symbol::Short,
                Symbol::Long,
                Symbol::Long,
                Symbol::Long,
                Symbol::Long,
            ],
            "1",
        ),
        (
            vec![
                Symbol::Short,
                Symbol::Short,
                Symbol::Long,
                Symbol::Long,
                Symbol::Long,
            ],
            "2",
        ),
        (
            vec![
                Symbol::Short,
                Symbol::Short,
                Symbol::Short,
                Symbol::Long,
                Symbol::Long,
            ],
            "3",
        ),
        (
            vec![
                Symbol::Short,
                Symbol::Short,
                Symbol::Short,
                Symbol::Short,
                Symbol::Long,
            ],
            "4",
        ),
        (
            vec![
                Symbol::Short,
                Symbol::Short,
                Symbol::Short,
                Symbol::Short,
                Symbol::Short,
            ],
            "5",
        ),
        (
            vec![
                Symbol::Long,
                Symbol::Short,
                Symbol::Short,
                Symbol::Short,
                Symbol::Short,
            ],
            "6",
        ),
        (
            vec![
                Symbol::Long,
                Symbol::Long,
                Symbol::Short,
                Symbol::Short,
                Symbol::Short,
            ],
            "7",
        ),
        (
            vec![
                Symbol::Long,
                Symbol::Long,
                Symbol::Long,
                Symbol::Short,
                Symbol::Short,
            ],
            "8",
        ),
        (
            vec![
                Symbol::Long,
                Symbol::Long,
                Symbol::Long,
                Symbol::Long,
                Symbol::Short,
            ],
            "9",
        ),
        (
            vec![
                Symbol::Long,
                Symbol::Short,
                Symbol::Long,
                Symbol::Short,
                Symbol::Long,
                Symbol::Long,
            ],
            "!",
        ),
        (
            vec![
                Symbol::Short,
                Symbol::Long,
                Symbol::Long,
                Symbol::Long,
                Symbol::Long,
                Symbol::Short,
            ],
            "'",
        ),
    ]
    .iter()
    .cloned()
    .collect();
    map.get(morse).unwrap_or(&"?")
}

#[derive(Debug, PartialEq)]
struct SymbolError;

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Symbol::Long => "-",
            Symbol::Short => ".",
            Symbol::Space => "/",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for SymbolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "?")
    }
}

impl std::convert::TryFrom<u64> for Symbol {
    type Error = SymbolError;
    fn try_from(sample_count: u64) -> Result<Self, Self::Error> {
        // A dot is one unit
        // A dash is three units
        if sample_count > 1200 {
            return Ok(Symbol::Long);
        }
        if sample_count > 400 {
            return Ok(Symbol::Short);
        }
        Err(SymbolError)
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

    #[test]
    fn parse_e() {
        // Short is 60ms
        // Long is 180 ms
        // Gap between is 60ms
        // For a short you'd expect to see ~500 samples greater than the threshold
        // For a gap you'd expect to see   ~500 samples lower than the threshold
        // For a long you'd expect to see  ~1500 samples greater than the threshold
        let mut data: [i16; 2000] = [0; 2000];
        let thresh = 100;
        for d in &mut data[100..601] {
            *d = thresh + 1;
        }
        let symbols = parse(&data, thresh);
        assert_eq!("e", symbols);
    }
    #[test]
    fn parse_j() {
        // Short is 60ms
        // Long is 180 ms
        // Gap between is 60ms
        // For a short you'd expect to see ~500 samples greater than the threshold
        // For a gap you'd expect to see   ~500 samples lower than the threshold
        // For a long you'd expect to see  ~1500 samples greater than the threshold
        // j is .--- = 1000, 2000,2000,2000
        let mut data: [i16; 10000] = [0; 10000];
        let thresh = 100;
        for d in &mut data[100..601] {
            *d = thresh + 1;
        }
        for d in &mut data[1101..3000] {
            *d = thresh + 1;
        }
        for d in &mut data[3501..5500] {
            *d = thresh + 1;
        }
        for d in &mut data[6001..8000] {
            *d = thresh + 1;
        }
        let symbols = parse(&data, thresh);
        assert_eq!("j", symbols);
    }

    #[test]
    fn parse_je() {
        // Short is 60ms
        // Long is 180 ms
        // Gap between is 60ms
        // For a short you'd expect to see ~500 samples greater than the threshold
        // For a gap you'd expect to see   ~500 samples lower than the threshold
        // For a long you'd expect to see  ~1500 samples greater than the threshold
        // j is .--- = 1000, 2000,2000,2000
        let mut data: [i16; 15000] = [0; 15000];
        let thresh = 100;
        for d in &mut data[100..601] {
            *d = thresh + 1;
        }
        for d in &mut data[1101..3000] {
            *d = thresh + 1;
        }
        for d in &mut data[3501..5500] {
            *d = thresh + 1;
        }
        for d in &mut data[6000..8000] {
            *d = thresh + 1;
        }
        for d in &mut data[10001..10500] {
            *d = thresh + 1;
        }
        let symbols = parse(&data, thresh);
        assert_eq!("j e", symbols);
    }
}
