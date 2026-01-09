<script>
  import { createEventDispatcher } from 'svelte';

  export let config = {
    threshold: 30.0,
    min_scene_len: 15,
    blur_size: 5
  };

  const dispatch = createEventDispatcher();

  function handleChange() {
    dispatch('configChange', config);
  }
</script>

<div class="parameter-panel">
  <div class="param-group">
    <label for="threshold">
      <span class="param-label">Threshold</span>
      <span class="param-value">{config.threshold.toFixed(1)}</span>
    </label>
    <input
      id="threshold"
      type="range"
      min="0"
      max="255"
      step="1"
      bind:value={config.threshold}
      on:input={handleChange}
    />
    <small>Lower = more sensitive, Higher = less sensitive</small>
  </div>

  <div class="param-group">
    <label for="min-scene-len">
      <span class="param-label">Min Scene Length</span>
      <span class="param-value">{config.min_scene_len} frames</span>
    </label>
    <input
      id="min-scene-len"
      type="range"
      min="1"
      max="60"
      step="1"
      bind:value={config.min_scene_len}
      on:input={handleChange}
    />
    <small>Minimum frames between scene changes</small>
  </div>

  <div class="param-group">
    <label for="blur-size">
      <span class="param-label">Blur Size</span>
      <span class="param-value">{config.blur_size}</span>
    </label>
    <input
      id="blur-size"
      type="range"
      min="1"
      max="15"
      step="2"
      bind:value={config.blur_size}
      on:input={handleChange}
    />
    <small>Gaussian blur kernel size (odd numbers only)</small>
  </div>
</div>

<style>
  .parameter-panel {
    background: #2a2a2a;
    padding: 1rem;
    border-radius: 4px;
  }

  .param-group {
    margin-bottom: 1.5rem;
  }

  .param-group:last-child {
    margin-bottom: 0;
  }

  label {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.5rem;
  }

  .param-label {
    font-weight: 500;
    color: #ddd;
  }

  .param-value {
    font-weight: 600;
    color: #4a9eff;
    font-family: monospace;
  }

  input[type="range"] {
    width: 100%;
    height: 6px;
    background: #444;
    border-radius: 3px;
    outline: none;
    -webkit-appearance: none;
  }

  input[type="range"]::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 16px;
    height: 16px;
    background: #4a9eff;
    border-radius: 50%;
    cursor: pointer;
  }

  input[type="range"]::-moz-range-thumb {
    width: 16px;
    height: 16px;
    background: #4a9eff;
    border-radius: 50%;
    cursor: pointer;
    border: none;
  }

  small {
    display: block;
    margin-top: 0.3rem;
    color: #888;
    font-size: 0.8rem;
  }
</style>
