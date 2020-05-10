use portaudio;
use std::sync::mpsc;
use std::convert::TryFrom;
use three;

// https://dev.to/maniflames/audio-visualization-with-rust-4nhg

struct UserConfig{
    fpb : u32
}

struct ParseError;

impl Default for UserConfig{
    fn default()-> UserConfig{
        UserConfig{
            fpb: portaudio::FRAMES_PER_BUFFER_UNSPECIFIED
        }
    }
}

impl std::fmt::Display for ParseError{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"Some error with you arguments")
    }
}

impl<'a> std::convert::TryFrom<&'a [String]> for UserConfig{
    type Error = ParseError;
    fn try_from(args: &'a [String]) -> Result<Self, Self::Error>{
        if args.len() == 1{
            return Ok(UserConfig::default());
        }

        if args.len() == 2 {
            if let Ok(user_fbp) = &args[1].parse::<u32>(){
                return Ok(UserConfig{ fpb: *user_fbp });   
            }
            return Err(ParseError);
        }

        Err(ParseError)
    }
}
// ----------------------------------------------

struct State {
    samples : Vec<f32>,
    scene_meshes : Vec<three::Mesh>
}

impl Default for State{
    fn default() -> State{
        State{
            samples: Vec::new(), 
            scene_meshes: Vec::new()
        }
    }
}

// ---------------------------------

fn get_window() -> three::window::Window {
    let mut builder = three::Window::builder("My microphone");
    builder.fullscreen(false);

    let mut win = builder.build();
    win.scene.background = three::Background::Color(0x000000);

    win
}

// -----------------------------------

fn main() {
    let args : Vec<String> = std::env::args().collect();
    let config = UserConfig::try_from(args.as_slice()).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });
    let pa = portaudio::PortAudio::new().expect("Unable to initialise portaudio instance");
    println!("{:?}", pa);

    let devices = pa.devices().unwrap();
    for device in devices{
        let (idx, info) = device.unwrap();
        println!("---------------- {:?} ----------------", idx);
        println!("{:?}", info);
    }

    let mic_index = pa.default_input_device().expect("Cant cget device");
    println!("{:?}", mic_index);

    let mic = pa.device_info(mic_index).expect("Unable to get mic instance");
    println!("{:?}", mic);

    let num_channels = 1;
    let interleave = true; // doesn't matter since I only want one channel?
    let latency = mic.default_low_input_latency;
    let input_params = portaudio::StreamParameters::<f32>::new(mic_index, num_channels, interleave, latency);
    println!("{:?}", input_params);

    let fs = mic.default_sample_rate;
    let frames_per_buffer = config.fpb;
    let input_settings = portaudio::InputStreamSettings::new(input_params, fs, frames_per_buffer);

    println!("{:?}", input_settings);

    let (sender, consumer) = mpsc::channel();
    let cb = move |portaudio::InputStreamCallbackArgs{ buffer, ..} | {
        match sender.send(buffer){
            Ok(_) => portaudio::Continue,
            Err(_) => portaudio::Complete
        }
    };

    let mut stream = pa.open_non_blocking_stream(input_settings, cb).expect("Unable to create stream");
    let mut win = get_window();
    let mut state = State::default();

    let camera = win.factory.orthographic_camera([0.0, 0.0], 1.0, -1.0..1.0);

    stream.start().expect("Unable to start streaming");

    while win.update() && !win.input.hit(three::KEY_ESCAPE){

        update_lines(&mut win, &mut state);
        win.render(&camera);

        remove_lines(&mut win, &mut state);
        while let Ok(buffer) = consumer.try_recv(){
            update_samples(&buffer, &mut state);
        }
    }
}

fn update_lines(win: &mut three::window::Window, state: &mut State){

    for(index, y_position) in state.samples.iter().enumerate(){
         let i = index as f32;
         let num_samples = state.samples.len() as f32;
         let scale = 3.0;
         let x_pos = (i/(num_samples/scale)) - (0.5 * scale);

         let geometry = three::Geometry::with_vertices(vec![
            [x_pos, y_position.clone(), 0.0].into(), 
            [x_pos, -y_position.clone(), 0.0].into()
         ]);

         let r = *y_position * 1024.0;
         let r_abs = std::cmp::min(r.abs() as u32, 255);
         let g = 0xff - r_abs;
         let color = (r_abs << 16) | (g << 8) | r_abs;
         eprintln!("r = {}, r_abs = {}, colour = {:#X?}", r, r_abs, color);
         let material = three::material::Line{
             color: color,
         };

         let mesh = win.factory.mesh(geometry, material);

         win.scene.add(&mesh);
         state.scene_meshes.push(mesh);
    }
}

fn remove_lines(win : &mut three::window::Window, state:&mut State){
    for mesh in &state.scene_meshes{
        win.scene.remove(&mesh);
    }
    state.scene_meshes.clear();
}

fn update_samples(samples:&[f32], state : &mut State){
    state.samples = samples.to_vec();
}