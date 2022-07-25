use hrtf::{HrirPoint, HrirSphere, HrtfContext, HrtfProcessor, Vec3};
use std::{
    env::args_os,
    ffi::OsString,
    fs::File,
    path::{Path, PathBuf},
    time::Instant,
};
use wav::{BitDepth, Header};


fn mul_vec(vec: Vec3, rhs: f32) -> Vec3 {
    Vec3 {
        x: vec.x * rhs,
        y: vec.y * rhs,
        z: vec.z * rhs,
    }
}

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

    let audio_data;
    let audio_header;

    let hrir_sphere: HrirSphere;

    let output_path: PathBuf;

    // turn the audio file into a vector of samples and the
    // other file into the HrirSphere.
    let mut source_vec = if let (Some(hrir), Some(audio), Some(output)) =
        (hrir_path, audio_path, output_path_maybe)
    {
        hrir_sphere = HrirSphere::from_file(hrir, 44100)
            .unwrap();
        let mut audio_file = File::open(Path::new(&audio)).expect("Error opening audio:");
        (audio_header, audio_data) = wav::read(&mut audio_file).expect("Error parsing audio:");
        let data = audio_data.try_into_thirty_two_float();
        output_path = PathBuf::from(output);
        if data.is_ok() {
            data.unwrap()
        } else {
            panic!("Error: Expected bitdepth of 32 float.");
        }
    } else {
        println!("USAGE:\n--hrir [path]\n    load the hrir sphere from the path\n--audio [path]\n    load the audio from a WAV file (must be *mono*)\n--speed [number]\n    Default: 10\n    The rate at which the audio rotates (higher = slower)\n--strength [number]\n    The strength of the 8D effect (might not work...)\n--output [path]\n    The file to put your output in");
        std::process::exit(0);
    };

    let points: Vec<HrirPoint> = hrir_sphere.points().iter().map(|x| x.clone()).collect();

    // Pad the source vector to ensure it aligns with the size of the processor and box it
    source_vec.extend(&vec![0.0f32; 128 - (source_vec.len() % 128)]);
    let source: Box<[&[f32]]> = source_vec.chunks(128).collect();

    let mut processor = HrtfProcessor::new(hrir_sphere, source[0].len() / 128, 128);

    println!("Performing {} iterations", source.len());

    let mut output = Vec::new();
    let mut prev_left_samples = vec![];
    let mut prev_right_samples = vec![];

    let mut prev_vec = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let mut current_point = 0;

    for index in 0..source.len() {
        let mut current = vec![(0.0, 0.0); source[index].len()];
        let point = mul_vec(points[current_point % points.len()].pos, strength);
        let context = HrtfContext {
            source: &source[index],
            output: &mut current,
            new_sample_vector: point,
            prev_sample_vector: prev_vec,
            prev_left_samples: &mut prev_left_samples,
            prev_right_samples: &mut prev_right_samples,
            // For simplicity, keep gain at 1.0 so there will be no interpolation.
            new_distance_gain: 1.0,
            prev_distance_gain: 1.0,
        };
        processor.process_samples(context);
        output.extend(current);
        if index % speed == 0 {
            prev_vec = point;
            current_point += 1;
        }
        if index != 0 && index % 100 == 0 {
            println!("Performed {} iterations", index);
        }
    }
    let mut file_out = File::create(output_path).expect("File error:");
    let updated_header = Header {
        channel_count: 2,
        ..audio_header
    };
    wav::write(
        updated_header,
        &BitDepth::ThirtyTwoFloat(
            output
                .into_iter()
                .map(|x| vec![x.0, x.1])
                .flatten()
                .collect(),
        ),
        &mut file_out,
    )
    .map_err(|op| op.to_string())
    .expect("Error writing wav file: ");
    println!("elapsed time: {}", begin.elapsed().as_secs());
}
