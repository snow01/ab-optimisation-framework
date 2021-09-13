<script>
  export let value = 10; //'slider value'
  export let disabled = false; //'whether the slider is disabled'
  export let start = 0; //'[noUi Slider start](https://refreshless.com/nouislider/slider-options/#section-start)'
  export let connect = [true, false]; //'[noUi Slider connect](https://refreshless.com/nouislider/slider-options/#section-connect)'
  export let min = 0;
  export let max = 100;
  export let className = "";
</script>

<style>
  /* button range start */
  .custom-range::-webkit-slider-thumb {
    background-color: #5e72e4 !important;
    box-shadow: none;
  }

  .custom-range::-webkit-slider-thumb:active {
    background-color: #5e72e4 !important;
    transform: scale(1.2);
    transition: transform 0.2s ease-in-out;
  }
  /* button range end */

  .custom-range::-webkit-slider-runnable-track{
    border-radius: 0px;
    background-color: #EBEEEF!important; 
  }

  .custom-range::-moz-range-thumb{
    border-radius: 0px;
  }


  .custom-range::-ms-thumb {
    border-radius: 0px;
  }

  .custom-range::-ms-track {
    border-radius: 0px;
  }

  .custon-radius::-ms-fill-lower {
    border-radius: 0px;
    background-color: #5e72e4!important;
  }

  .custom-range::-ms-fill-uper {
    border-radius: 0px;
    background-color: #E8EAEB!important; 
  }
</style>

<div class="slider {disabled ? 'disabled' : ''} {className}">
  <input
    type="range"
    class="custom-range"
    {min}
    {max}
    {value}
    step="0.01"
    style="height: 5px;" />
</div>
