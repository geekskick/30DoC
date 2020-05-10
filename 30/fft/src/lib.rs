use rustfft::num_complex::Complex;
use three;

// https://dev.to/maniflames/audio-visualization-with-rust-4nhg

pub struct UserConfig {
    pub fpb: u32,
}

pub struct ParseError;

impl Default for UserConfig {
    fn default() -> UserConfig {
        UserConfig {
            fpb: portaudio::FRAMES_PER_BUFFER_UNSPECIFIED,
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Some error with you arguments")
    }
}

impl<'a> std::convert::TryFrom<&'a [String]> for UserConfig {
    type Error = ParseError;
    fn try_from(args: &'a [String]) -> Result<Self, Self::Error> {
        if args.len() == 1 {
            return Ok(UserConfig::default());
        }

        if args.len() == 2 {
            if let Ok(user_fbp) = &args[1].parse::<u32>() {
                return Ok(UserConfig { fpb: *user_fbp });
            }
            return Err(ParseError);
        }

        Err(ParseError)
    }
}
// ----------------------------------------------

pub struct State {
    pub samples: Vec<Complex<f32>>,
    pub scene_meshes: Vec<three::Mesh>,
}

impl Default for State {
    fn default() -> State {
        State {
            samples: Vec::new(),
            scene_meshes: Vec::new(),
        }
    }
}

// ---------------------------------
