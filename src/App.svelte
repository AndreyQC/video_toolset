<script>
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import VideoPlayer from './components/VideoPlayer.svelte';
  import RoiEditor from './components/RoiEditor.svelte';
  import ParameterPanel from './components/ParameterPanel.svelte';
  import SceneList from './components/SceneList.svelte';

  let selectedVideo = '';
  let videoMetadata = null;
  let config = {
    threshold: 30.0,
    roi: { x: 0, y: 0, width: 1920, height: 1080 },
    min_scene_len: 15,
    blur_size: 5
  };
  let scenes = [];
  let isDetecting = false;
  let status = '';

  async function loadVideo() {
    if (!selectedVideo) return;

    status = 'Loading video...';
    try {
      videoMetadata = await invoke('get_video_metadata', { videoPath: selectedVideo });
      // Update ROI to match video dimensions
      config.roi.width = videoMetadata.width;
      config.roi.height = videoMetadata.height;
      status = 'Video loaded';
    } catch (err) {
      status = `Error: ${err}`;
      console.error(err);
    }
  }

  async function runDetection() {
    if (!selectedVideo) {
      status = 'Please select a video file';
      return;
    }

    isDetecting = true;
    status = 'Running scene detection...';

    try {
      scenes = await invoke('detect_scenes', {
        videoPath: selectedVideo,
        outputDir: './Output'
      });
      status = `Detection complete: ${scenes.length} scenes found`;
    } catch (err) {
      status = `Error: ${err}`;
      console.error(err);
    } finally {
      isDetecting = false;
    }
  }

  async function loadConfig() {
    try {
      config = await invoke('get_config');
      status = 'Config loaded';
    } catch (err) {
      console.error(err);
    }
  }

  async function saveConfig() {
    try {
      await invoke('save_config', {
        configPath: 'config.yaml',
        config: config
      });
      status = 'Config saved';
    } catch (err) {
      status = `Error saving config: ${err}`;
      console.error(err);
    }
  }

  onMount(() => {
    loadConfig();
  });
</script>

<main class="app">
  <header class="header">
    <h1>Video Scene Detection Tool</h1>
  </header>

  <div class="content">
    <div class="left-panel">
      <div class="video-section">
        <h2>Video Input</h2>
        <input
          type="file"
          accept="video/*"
          on:change={(e) => selectedVideo = e.target.files[0]?.path || ''}
          class="file-input"
        />
        <button on:click={loadVideo} disabled={!selectedVideo}>
          Load Video
        </button>

        {#if videoMetadata}
          <div class="video-info">
            <p><strong>Resolution:</strong> {videoMetadata.width}x{videoMetadata.height}</p>
            <p><strong>FPS:</strong> {videoMetadata.fps.toFixed(2)}</p>
            <p><strong>Duration:</strong> {videoMetadata.duration_seconds.toFixed(2)}s</p>
            <p><strong>Frames:</strong> {videoMetadata.frame_count}</p>
          </div>
        {/if}
      </div>

      <div class="video-player-section">
        {#if selectedVideo}
          <VideoPlayer videoPath={selectedVideo} />
        {/if}
      </div>
    </div>

    <div class="right-panel">
      <div class="roi-section">
        <h2>Region of Interest</h2>
        <RoiEditor
          bind:roi={config.roi}
          videoMetadata={videoMetadata}
          on:roiChange={(e) => config.roi = e.detail}
        />
      </div>

      <div class="params-section">
        <h2>Parameters</h2>
        <ParameterPanel
          bind:config={config}
          on:configChange={(e) => config = e.detail}
        />
      </div>

      <div class="actions-section">
        <button
          class="detect-button"
          on:click={runDetection}
          disabled={!selectedVideo || isDetecting}
        >
          {isDetecting ? 'Detecting...' : 'Run Detection'}
        </button>
        <button on:click={saveConfig}>
          Save Config
        </button>
      </div>

      <div class="status-section">
        <p class="status">{status}</p>
      </div>

      {#if scenes.length > 0}
        <div class="scenes-section">
          <h2>Detected Scenes ({scenes.length})</h2>
          <SceneList scenes={scenes} />
        </div>
      {/if}
    </div>
  </div>
</main>

<style>
  .app {
    display: flex;
    flex-direction: column;
    height: 100vh;
    background: #1a1a1a;
    color: #ffffff;
  }

  .header {
    padding: 1rem 2rem;
    background: #2a2a2a;
    border-bottom: 1px solid #3a3a3a;
  }

  .header h1 {
    margin: 0;
    font-size: 1.5rem;
    font-weight: 600;
  }

  .content {
    display: flex;
    flex: 1;
    overflow: hidden;
  }

  .left-panel {
    flex: 2;
    padding: 1.5rem;
    overflow-y: auto;
  }

  .right-panel {
    flex: 1;
    padding: 1.5rem;
    background: #222;
    overflow-y: auto;
    border-left: 1px solid #3a3a3a;
  }

  .video-section,
  .video-player-section,
  .roi-section,
  .params-section,
  .actions-section,
  .status-section,
  .scenes-section {
    margin-bottom: 2rem;
  }

  .video-section h2,
  .roi-section h2,
  .params-section h2,
  .scenes-section h2 {
    margin-top: 0;
    font-size: 1.1rem;
    color: #4a9eff;
  }

  .file-input {
    width: 100%;
    padding: 0.5rem;
    margin-bottom: 1rem;
    background: #333;
    border: 1px solid #444;
    border-radius: 4px;
    color: #fff;
  }

  button {
    padding: 0.6rem 1.2rem;
    background: #4a9eff;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.9rem;
    transition: background 0.2s;
  }

  button:hover:not(:disabled) {
    background: #3a8eef;
  }

  button:disabled {
    background: #555;
    cursor: not-allowed;
    opacity: 0.6;
  }

  .detect-button {
    width: 100%;
    padding: 0.8rem;
    font-size: 1rem;
    font-weight: 600;
    background: #00cc66;
  }

  .detect-button:hover:not(:disabled) {
    background: #00bb66;
  }

  .video-info {
    margin-top: 1rem;
    padding: 1rem;
    background: #2a2a2a;
    border-radius: 4px;
  }

  .video-info p {
    margin: 0.3rem 0;
    font-size: 0.9rem;
  }

  .status {
    padding: 0.8rem;
    background: #2a2a2a;
    border-radius: 4px;
    font-size: 0.9rem;
    color: #aaa;
  }
</style>
