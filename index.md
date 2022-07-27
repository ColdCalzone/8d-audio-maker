# 8D audio creator
##This page is designed to turn \*.wav files into 8D audio.
All you need to do is upload a \*.wav file and select a HRIR sphere (or upload your own)
All pre-selected HRIR spheres are from [here](https://github.com/mrDIMAS/hrir_sphere_builder/tree/master/hrtf_base/IRCAM).
<input type="file" id="audio-file">

<input type="file" id="hrir-file-upload">
<label for="hrir-select">Choose a pre-selected HRIR sphere</label>
<select name="hrir-select" id="hrir-select"></select>
<button onclick="parseAudio()">8D-ify</button>
