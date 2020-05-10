mod lib;
use crate::lib::*;
use portaudio;
use rustfft::num_complex::Complex;

use rustfft::num_traits::Zero;
use rustfft::FFT;
use std::convert::TryFrom;
use std::io::Write;
use std::sync::mpsc;

fn get_input() -> String {
    let mut guess = String::new();

    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    guess
}
// -----------------------------------

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let config = UserConfig::try_from(args.as_slice()).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });

    let pa = portaudio::PortAudio::new().expect("Unable to initialise portaudio instance");
    println!("{:?}", pa);

    println!("Devices found:");
    let devices = pa.devices().unwrap();
    for device in devices {
        let (idx, info) = device.unwrap();
        println!("{:?} = {:?}", idx, info);
    }

    let portaudio::DeviceIndex(default) = pa.default_input_device().expect("Unable to get default");

    print!("Which device do you want to use? Leave blank for default\n>\t");
    std::io::stdout().flush().expect("Unable to flush stdout");
    let choice: u32 = get_input().trim().parse().unwrap_or(default);

    let mic_index = portaudio::DeviceIndex(choice);
    println!("Using device: {:?}", mic_index);

    let mic = pa
        .device_info(mic_index)
        .expect("Unable to get mic instance");
    println!("{:?}", mic);

    if mic.max_input_channels == 0 {
        eprintln!("Need to have at least 1 input channel for this application.");
        std::process::exit(1);
    }

    let num_channels = 1;
    let interleave = true; // This matters!
    let latency = mic.default_low_input_latency;
    let input_params =
        portaudio::StreamParameters::<f32>::new(mic_index, num_channels, interleave, latency);
    println!("{:?}", input_params);

    let fs = mic.default_sample_rate;
    let frames_per_buffer = config.fpb;
    let input_settings = portaudio::InputStreamSettings::new(input_params, fs, frames_per_buffer);

    println!("{:?}", input_settings);

    let (sender, consumer) = mpsc::channel();
    let cb = move |portaudio::InputStreamCallbackArgs { buffer, .. }| match sender.send(buffer) {
        Ok(_) => portaudio::Continue,
        Err(_) => portaudio::Complete,
    };

    let mut stream = pa
        .open_non_blocking_stream(input_settings, cb)
        .expect("Unable to create stream");
    let mut win = get_window();
    let mut state = State::default();

    let camera = win.factory.orthographic_camera([0.0, 0.0], 1.0, -1.0..1.0);

    stream.start().expect("Unable to start streaming");

    while win.update() && !win.input.hit(three::KEY_ESCAPE) {
        update_lines(&mut win, &mut state);
        win.render(&camera);

        remove_lines(&mut win, &mut state);
        while let Ok(buffer) = consumer.try_recv() {
            update_samples(&buffer, &mut state);
        }
    }
}

fn get_window() -> three::window::Window {
    let mut builder = three::Window::builder("My microphone");
    builder.fullscreen(false);

    let mut win = builder.build();
    win.scene.background = three::Background::Color(0x000000);

    win
}

fn update_lines(win: &mut three::window::Window, state: &mut State) {
    for (index, y_position) in state.samples.iter().enumerate() {
        let i = index as f32;
        let num_samples = state.samples.len() as f32;
        let scale = 3.0;
        let x_pos = (i / (num_samples / scale)) - (0.5 * scale);

        let geometry = three::Geometry::with_vertices(vec![
            [x_pos, y_position.norm(), 0.0].into(),
            [x_pos, -y_position.norm(), 0.0].into(),
        ]);

        let color = 0x00ff00;
        let material = three::material::Line { color: color };

        let mesh = win.factory.mesh(geometry, material);
        win.scene.add(&mesh);

        state.scene_meshes.push(mesh);

    }
}

fn remove_lines(win: &mut three::window::Window, state: &mut State) {
    for mesh in &state.scene_meshes {
        win.scene.remove(&mesh);
    }
    state.scene_meshes.clear();
}

fn update_samples(samples: &[f32], state: &mut State) {
    let mut input: Vec<Complex<f32>> = samples
        .iter()
        .map(|sample| Complex::new(*sample, 0.0))
        .collect();

    let mut output: Vec<Complex<f32>> = vec![Complex::zero(); input.len()];

    let fft = rustfft::algorithm::Radix4::new(input.len(), false);
    fft.process(&mut input, &mut output);

    state.samples = output;
}
