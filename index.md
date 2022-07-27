<style>
  .center {
    margin-right: auto;
    margin-left: auto;
    margin-bottom: 10%;
    padding-bottom: 8%;
  }
  .big {
    width: 80%;
  }
  .small {
    width: 40%;
    display: inline-block;
    border: 2px dashed #9CA091;
    margin-right: 5%;
    margin-left: 5%;
    float: left;
  }
  .small-vertical {
    width: 100%;
    border: 2px dashed #9CA091;
    margin-top: 3%;
    float: left;
  }
  p, h1, h2{
    text-align: center;
  }
  .small p, input, select {
    margin-bottom: -5px;
  }
  .small hr {
    margin-bottom: -3px;
  }
  #button-container {
    width: 100px;
  }
  #button-container-container {
    padding-top: 1%;
  }
</style>

## This page is designed to turn \*.wav files into 8D audio.
All you need to do is upload a \*.wav file and select a HRIR sphere (or upload your own)\
All pre-selected HRIR spheres are from [here](https://github.com/mrDIMAS/hrir_sphere_builder/tree/master/hrtf_base/IRCAM).
<div class="center big">
  <div class="small">
    <div class="small-vertical">
      Upload Audio
      <input type="file" id="audio-file">
    </div>
    <div class="small-vertical">
      Rotation rate
      <input type="number" id="rate" min="1" value="10">
    </div>
  </div>
  <div class="small">
    Select Sphere
    <input type="file" id="hrir-file-upload">
    <hr>
    <label for="hrir-select">Choose a pre-selected HRIR sphere</label>
    <select name="hrir-select" id="hrir-select" value="IRC_1002_C.bin"></select>
  </div>
</div>
<div class="center big" id="button-container-container">
  <div class="center" id="button-container">
    <button id="parse" onclick="parseAudio()">8D-ify</button>
  </div>
</div>

<script>
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
  function parseAudio() {
    let rate = document.getElementById("rate").value;
    let sphere = document.getElementById("hrir-file-upload").files[0];
    let audio = document.getElementById("audio-file").files[0];
    if(sphere == undefined) {
      sphere = select.value;
    }
    console.log(sphere);
  }

  hrir_spheres.forEach(appendOption); 
</script>