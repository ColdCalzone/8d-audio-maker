<style>
  .center {
    margin: 0 auto;
  }
  #big {
    width: 80%;
  }
  .small {
    width: 40%;
    display: inline-block;
    border: 2px dashed #9CA091;
    margin-right: 10%;
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
    margin-top: -2px;
  }
  #button-container {
    position: absolute;
    bottom: 0px;
    margin: 0 auto;
  }
  .site-footer {
    padding-top: 2rem;
    margin-top: 12rem;
    border-top: solid 1px #eff0f1;
  }
  
</style>
## This page is designed to turn \*.wav files into 8D audio.
All you need to do is upload a \*.wav file and select a HRIR sphere (or upload your own)\
All pre-selected HRIR spheres are from [here](https://github.com/mrDIMAS/hrir_sphere_builder/tree/master/hrtf_base/IRCAM).
<div id="big">
  <div class="small">
    Upload Audio
    <input type="file" id="audio-file">
  </div>
  <div class="small">
    Select Sphere
    <input type="file" id="hrir-file-upload">
    <hr>
    <label for="hrir-select">Choose a pre-selected HRIR sphere</label>
    <select name="hrir-select" id="hrir-select"></select>
  </div>
</div>
<p id="button-container">
  <button id="parse" onclick="parseAudio()">8D-ify</button>
</p>
