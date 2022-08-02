use hrtf::{HrirPoint, HrirSphere, HrtfContext, HrtfProcessor, Vec3};
use std::{io::{Seek, Write}};
use wav::{BitDepth, Header};
#[cfg(not(target_family = "wasm"))]
use std::{
    ffi::OsString,
    fs::File,
    path::{Path, PathBuf},
};
#[cfg(target_family = "wasm")]
use {std::{io::Cursor, collections::HashMap}, js_sys::Uint8Array, wasm_bindgen::prelude::*, serde::{Serialize, Deserialize}};

fn mul_vec(vec: Vec3, rhs: f32) -> Vec3 {
    Vec3 {
        x: vec.x * rhs,
        y: vec.y * rhs,
        z: vec.z * rhs,
    }
}

fn format_audio(buffer: BitDepth, header: &Header) -> BitDepth {
    let new_buf : Vec<f32> = match buffer {
        BitDepth::ThirtyTwoFloat(buf) => {
            buf
        },
        _ => panic!("Expected 32 bit float audio!")
    };
    let Header {
        channel_count,
        ..
    } = *header;

    let mut point : isize = -1;
    
    BitDepth::ThirtyTwoFloat(new_buf.iter()
    .filter(|_| {
        point += 1;
        point % (channel_count as isize) == 0})
    .map(|x| *x)
    .collect())
}

#[cfg(not(target_family = "wasm"))]
pub fn args_to_audio(
    hrir_path: Option<OsString>,
    audio_path: Option<OsString>,
    output_path_maybe: Option<OsString>,
    speed: usize,
    strength: f32,
) -> (Vec<(f32, f32)>, Header, PathBuf) {
    let audio_data;
    let audio_header;

    let hrir_sphere: HrirSphere;

    let output_path: PathBuf;

    // turn the audio file into a vector of samples and the
    // other file into the HrirSphere.
    let mut source_vec = if let (Some(hrir), Some(audio), Some(output)) =
        (hrir_path, audio_path, output_path_maybe)
    {
        hrir_sphere = HrirSphere::from_file(hrir, 44100).unwrap();
        let mut audio_file = File::open(Path::new(&audio)).expect("Error opening audio");
        (audio_header, audio_data) = wav::read(&mut audio_file).expect("Error parsing audio");
        let data = format_audio(audio_data, &audio_header).try_into_thirty_two_float();
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
        if index != 0 && index % 1000 == 0 {
            println!("Performed {} iterations", index);
        }
    }
    let updated_header = Header {
        channel_count: 2,
        ..audio_header
    };
    (output, updated_header, output_path)
}


pub fn save_audio_to_buffer<W>(output: Vec<(f32, f32)>, header: Header, mut buffer: &mut W)
where
    W: Write + Seek,
{
    wav::write(
        header,
        &BitDepth::ThirtyTwoFloat(
            output
                .into_iter()
                .map(|x| vec![x.0, x.1])
                .flatten()
                .collect(),
        ),
        &mut buffer,
    )
    .map_err(|op| op.to_string())
    .expect("Error writing wav file");
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
        fn alert(s: &str);
}

#[cfg(target_family = "wasm")]
#[derive(Serialize, Deserialize)]
pub struct UInt8 {
    array: HashMap<u32, u32>,
}

#[cfg(target_family = "wasm")]
pub fn buffers_to_audio(buffer_audio: &mut [u8], buffer_hrir : &mut [u8], rate : i32) -> (Vec<(f32, f32)>, Header) {
    let mut audio = Cursor::new(buffer_audio);
    let hrir = Cursor::new(buffer_hrir);
    
    let (audio_header, audio_data) = wav::read(&mut audio).expect("Error parsing audio");
    let hrir_sphere : HrirSphere = HrirSphere::new(hrir, 44100).expect("Error making sphere");
    let speed = rate as usize;
    
    let data = format_audio(audio_data, &audio_header).try_into_thirty_two_float();
    let mut source_vec = if data.is_ok() {
        data.unwrap()
    } else {
        alert("Expected audio to have 32 bit floating point samples, please reencode your audio.");
        panic!();
    };

    let points: Vec<HrirPoint> = hrir_sphere.points().iter().map(|x| x.clone()).collect();

    source_vec.extend(&vec![0.0f32; 128 - (source_vec.len() % 128)]);
    let source: Box<[&[f32]]> = source_vec.chunks(128).collect();

    let mut processor = HrtfProcessor::new(hrir_sphere, source[0].len() / 128, 128);

    println!("Performing {} iterations", source.len());

    let mut prev_left_samples = vec![];
    let mut prev_right_samples = vec![];

    let mut prev_vec = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let mut current_point = 0;

    let mut output : Vec<(f32, f32)> = Vec::new();

    for index in 0..source.len() {
        let mut current = vec![(0.0, 0.0); source[index].len()];
        let point = mul_vec(points[current_point % points.len()].pos, 1.0);
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
        if index != 0 && index % 1000 == 0 {
            println!("Performed {} iterations", index);
        }
    }
    let updated_header = Header {
        channel_count: 2,
        ..audio_header
    };
    
    (output, updated_header)
}

#[wasm_bindgen]
#[cfg(target_family = "wasm")]
pub fn convert_data_to_audio_blob(audio_jsvalue : &wasm_bindgen::JsValue, hrir_jsvalue : &wasm_bindgen::JsValue, rate : i32) -> Uint8Array {
    console_error_panic_hook::set_once();
    let audio : UInt8 = audio_jsvalue.into_serde().expect("Error parsing array");
    let hrir : UInt8 = hrir_jsvalue.into_serde().expect("Error parsing array");

    let mut buffer_audio : Vec<u8> = Vec::new();
    let mut buffer_hrir : Vec<u8> = Vec::new();

    for i in 0..hrir.array.len() {
        buffer_hrir.push(hrir.array[&(i as u32)] as u8);
    }
    for i in 0..audio.array.len() {
        buffer_audio.push(audio.array[&(i as u32)] as u8);
    }
    
    let (output, updated_header) = buffers_to_audio(buffer_audio.as_mut_slice(), buffer_hrir.as_mut_slice(), rate);
    let mut output_blob : Cursor<Vec<u8>> = Cursor::new(Vec::new());
    save_audio_to_buffer(output, updated_header, &mut output_blob);
    
    // SAFETY: Probably lol
    unsafe {
        Uint8Array::view(output_blob.into_inner().as_slice())
    }
}

#[test]
fn compare_results() {
    use std::io::Read;
    println!("running command line version");
    let mut buffer_audio : Vec<u8> = Vec::new();
    let mut buffer_hrir : Vec<u8> = Vec::new();
    
    {
        let mut file_audio = File::open("./test.wav").unwrap();
        let mut file_hrir = File::open("./test.bin").unwrap();
        file_audio.read_to_end(&mut buffer_audio).unwrap();
        file_hrir.read_to_end(&mut buffer_hrir).unwrap();
    }

    let (audio_from_args, header_args, _) = args_to_audio(Some(OsString::from(String::from("./test.bin"))), Some(OsString::from(String::from("./test.wav"))), Some(OsString::from(String::from(""))), 10, 1.0);
    println!("running wasm version");
    let (audio_from_buffer, header_buffer) = buffers_to_audio(buffer_audio.as_mut_slice(), buffer_hrir.as_mut_slice(), 10);

    let mut buffer_out_args : Cursor<Vec<u8>> = Cursor::new(Vec::new());
    let mut buffer_out_buffer : Cursor<Vec<u8>> = Cursor::new(Vec::new());

    println!("Comparing audio");
    assert_eq!(audio_from_buffer, audio_from_args);
    println!("Comparing headers");
    assert_eq!(header_buffer, header_args);
    println!("Comparing saved results");
    save_audio_to_buffer(audio_from_buffer, header_buffer, &mut buffer_out_buffer);
    save_audio_to_buffer(audio_from_args, header_args, &mut buffer_out_args);
    assert_eq!(buffer_out_buffer, buffer_out_args);
}
