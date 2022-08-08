import init from "./pkg/audio_maker.js";

function get_download(url) {
    let link = document.createElement('a');
    link.download = 'audio.wav';

    link.href = url;

    link.click();

    URL.revokeObjectURL(link.href);
}

const getBlob = async (audio_buffer, hrir_buffer, rate) => {
	// Instantiate our wasm module
    const wasm = await init("./pkg/audio_maker_bg.wasm");

    console.log(audio_buffer);
    console.log(hrir_buffer);
    console.log(rate);

    for(let i = 0; i < audio_buffer.length; i++) {
        wasm.write_to_audio(audio_buffer[i]);
    }

    for(let i = 0; i < hrir_buffer.length; i++) {
        wasm.write_to_hrir(hrir_buffer[i]);
    }

    wasm.write_rate(rate);

    const audio = new Blob(wasm.convert_data_to_audio_blob());

	get_download(window.URL.createObjectURL(audio));
    
    let elements = document.getElementsByClassName("input");
    for(let i = 0; i < elements.length; i++) {
        elements[i].disabled = false;
    }
};

const PREFIX = "./hrir/";
let hrir_spheres = [
	"IRC_1002_C.bin",
	"IRC_1003_C.bin",
	"IRC_1004_C.bin",
	"IRC_1005_C.bin",
	"IRC_1006_C.bin",
	"IRC_1007_C.bin",
	"IRC_1008_C.bin",
	"IRC_1009_C.bin",
	"IRC_10012_C.bin",
	"IRC_10013_C.bin",
	"IRC_10014_C.bin",
];
let select = document.getElementById("hrir-select");

function appendOption(value, index, array) {
	let option = document.createElement("option");
	option.value = value;
	option.innerHTML = value;
	select.appendChild(option);
}

window.onload = function() {
    var btn = document.getElementById("parse");
    btn.onclick = async () => {
        let elements = document.getElementsByClassName("input");
        for(let i = 0; i < elements.length; i++) {
            elements[i].disabled = false;
        }
        
        let rate = parseInt(document.getElementById("rate").value);
        let sphere = document.getElementById("hrir-file-upload").files[0];
        let audio = document.getElementById("audio-file").files[0];
        if(sphere == undefined) {
            await fetch(PREFIX + select.value)
            .then(res => res.blob())
            .then(blob => sphere = blob);
        }
        if(audio == undefined) {
            return;
        }
        let reader_audio = new FileReader();
        let reader_hrir = new FileReader();

        let audio_buffer;
        let hrir_buffer;

        reader_audio.readAsArrayBuffer(audio);
        reader_hrir.readAsArrayBuffer(sphere);
        reader_audio.onload = async () => {
            audio_buffer = new Uint8Array(reader_audio.result);
            if(reader_hrir.readyState == FileReader.DONE) await getBlob(audio_buffer, hrir_buffer, rate);
        }
        reader_hrir.onload = async () => {
            hrir_buffer = new Uint8Array(reader_hrir.result);
            if(reader_audio.readyState == FileReader.DONE) await getBlob(audio_buffer, hrir_buffer, rate);
        }
    };
}

hrir_spheres.forEach(appendOption); 