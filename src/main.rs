use std::{time::Instant, fs::File, ffi::OsString, env::args_os};

fn get_args() -> (
    Option<OsString>,
    Option<OsString>,
    Option<OsString>,
    usize,
    f32,
) {
    let mut hrir_path: Option<OsString> = None;
    let mut audio_path: Option<OsString> = None;
    let mut output_path: Option<OsString> = None;
    let mut speed: usize = 10;
    let mut strength: f32 = 1.0;
    let mut args = args_os().peekable();
    while args.peek() != None {
        let arg = args.next().unwrap();
        match arg.to_ascii_lowercase().to_str() {
            Some(str_arg) => match str_arg {
                "--hrir" => hrir_path = args.next(),
                "--audio" => audio_path = args.next(),
                "--speed" => {
                    let number = args.next();
                    if let Some(num) = number {
                        let n = num
                            .to_str()
                            .map(|x| x.parse::<usize>())
                            .filter(|x| x.is_ok());
                        if n.is_some() {
                            speed = n.unwrap().unwrap().max(1);
                        }
                    }
                }
                "--strength" => {
                    let number = args.next();
                    if let Some(num) = number {
                        let n = num.to_str().map(|x| x.parse::<f32>()).filter(|x| x.is_ok());
                        if n.is_some() {
                            strength = n.unwrap().unwrap().max(1.0);
                        }
                    }
                }
                "--output" => output_path = args.next(),
                _ => {}
            },
            None => {}
        }
    }
    (hrir_path, audio_path, output_path, speed, strength)
}

fn main() {
    let begin = Instant::now();
    let (hrir_path, audio_path, output_path_maybe, speed, strength) = get_args();

    let audio = audio_maker::args_to_audio(hrir_path, audio_path, output_path_maybe, speed, strength);

    let mut file = File::create(audio.2).expect("File error:");
    audio_maker::save_audio_to_buffer(audio.0, audio.1, &mut file);

    println!("elapsed time: {}", begin.elapsed().as_secs());
}