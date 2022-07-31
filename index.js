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

	// Call the Add function export from wasm, save the result
	const audio = Blob(wasm.convert_data_to_audio_blob(audio_buffer, hrir_buffer, rate));

	//https://stackoverflow.com/questions/33247716/javascript-file-download-with-blob
	get_download(window.URL.createObjectURL(blob));
};

const PREFIX = "./hrir spheres/";
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

async function parseAudio() {
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
    let audio_buffer = new Uint8Array(audio.arrayBuffer());
    let hrir_buffer = new Uint8Array(sphere.arrayBuffer());
    await getBlob(audio_buffer, hrir_buffer, rate);
}

window.onload = function() {
    var btn = document.getElementById("parse");
    btn.onclick = parseAudio;
}

hrir_spheres.forEach(appendOption); 