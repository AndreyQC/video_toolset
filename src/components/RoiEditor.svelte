<script>
  import { createEventDispatcher } from 'svelte';

  export let roi = { x: 0, y: 0, width: 1920, height: 1080 };
  export let videoMetadata = null;

  const dispatch = createEventDispatcher();

  let roiCanvas;
  let videoCanvas;
  let isDragging = false;
  let dragStart = null;
  let videoElement;

  function handleInputChange() {
    dispatch('roiChange', roi);
  }

  function startDrag(e) {
    if (!videoCanvas) return;

    isDragging = true;
    const rect = videoCanvas.getBoundingClientRect();
    dragStart = {
      x: e.clientX - rect.left,
      y: e.clientY - rect.top
    };
    roi.x = dragStart.x;
    roi.y = dragStart.y;
    dispatch('roiChange', roi);
  }

  function onDrag(e) {
    if (!isDragging || !dragStart || !videoCanvas) return;

    const rect = videoCanvas.getBoundingClientRect();
    const currentX = e.clientX - rect.left;
    const currentY = e.clientY - rect.top;

    roi.width = Math.abs(currentX - dragStart.x);
    roi.height = Math.abs(currentY - dragStart.y);
    roi.x = Math.min(dragStart.x, currentX);
    roi.y = Math.min(dragStart.y, currentY);

    dispatch('roiChange', roi);
  }

  function endDrag() {
    isDragging = false;
    dragStart = null;
  }

  function centerRoi() {
    if (!videoMetadata) return;

    roi.x = (videoMetadata.width - roi.width) / 2;
    roi.y = (videoMetadata.height - roi.height) / 2;
    dispatch('roiChange', roi);
  }

  function fitToVideo() {
    if (!videoMetadata) return;

    roi.x = 0;
    roi.y = 0;
    roi.width = videoMetadata.width;
    roi.height = videoMetadata.height;
    dispatch('roiChange', roi);
  }
</script>

<div class="roi-editor">
  <div class="canvas-container">
    <canvas
      bind:this={videoCanvas}
      on:mousedown={startDrag}
      on:mousemove={onDrag}
      on:mouseup={endDrag}
      on:mouseleave={endDrag}
    >
    </canvas>
  </div>

  <div class="roi-inputs">
    <div class="input-group">
      <label>X:</label>
      <input
        type="number"
        bind:value={roi.x}
        on:input={handleInputChange}
        min="0"
      />
    </div>
    <div class="input-group">
      <label>Y:</label>
      <input
        type="number"
        bind:value={roi.y}
        on:input={handleInputChange}
        min="0"
      />
    </div>
    <div class="input-group">
      <label>Width:</label>
      <input
        type="number"
        bind:value={roi.width}
        on:input={handleInputChange}
        min="1"
      />
    </div>
    <div class="input-group">
      <label>Height:</label>
      <input
        type="number"
        bind:value={roi.height}
        on:input={handleInputChange}
        min="1"
      />
    </div>
  </div>

  <div class="roi-actions">
    <button on:click={centerRoi}>Center</button>
    <button on:click={fitToVideo}>Fit to Video</button>
  </div>

  {#if videoMetadata}
    <div class="video-size">
      <small>Video size: {videoMetadata.width}x{videoMetadata.height}</small>
    </div>
  {/if}
</div>

<style>
  .roi-editor {
    background: #2a2a2a;
    padding: 1rem;
    border-radius: 4px;
  }

  .canvas-container {
    position: relative;
    width: 100%;
    background: #1a1a1a;
    border-radius: 4px;
    margin-bottom: 1rem;
    overflow: hidden;
  }

  canvas {
    display: block;
    width: 100%;
    height: auto;
    cursor: crosshair;
  }

  .roi-inputs {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 0.5rem;
    margin-bottom: 1rem;
  }

  .input-group {
    display: flex;
    flex-direction: column;
  }

  .input-group label {
    font-size: 0.8rem;
    color: #aaa;
    margin-bottom: 0.2rem;
  }

  .input-group input {
    padding: 0.4rem;
    background: #333;
    border: 1px solid #444;
    border-radius: 4px;
    color: #fff;
    font-size: 0.9rem;
  }

  .roi-actions {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 0.5rem;
  }

  .roi-actions button {
    flex: 1;
    padding: 0.5rem;
    font-size: 0.85rem;
    background: #3a3a3a;
  }

  .roi-actions button:hover {
    background: #4a4a4a;
  }

  .video-size {
    color: #888;
    text-align: center;
  }
</style>
