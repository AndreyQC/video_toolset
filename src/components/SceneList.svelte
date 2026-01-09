<script>
  export let scenes = [];
</script>

<div class="scene-list">
  {#each scenes as scene, index}
    <div class="scene-item">
      <div class="scene-number">{scene.scene_id}</div>
      <div class="scene-info">
        <div class="scene-timestamp">{scene.timestamp}</div>
        <div class="scene-frame">Frame {scene.frame_number}</div>
      </div>
    </div>
  {/each}
</div>

<style>
  .scene-list {
    max-height: 300px;
    overflow-y: auto;
    background: #2a2a2a;
    border-radius: 4px;
  }

  .scene-item {
    display: flex;
    align-items: center;
    padding: 0.8rem;
    border-bottom: 1px solid #3a3a3a;
    transition: background 0.2s;
  }

  .scene-item:last-child {
    border-bottom: none;
  }

  .scene-item:hover {
    background: #2f2f2f;
  }

  .scene-number {
    flex: 0 0 40px;
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: #4a9eff;
    color: white;
    font-weight: 600;
    border-radius: 4px;
    margin-right: 1rem;
  }

  .scene-info {
    flex: 1;
  }

  .scene-timestamp {
    font-weight: 500;
    color: #fff;
    margin-bottom: 0.2rem;
  }

  .scene-frame {
    font-size: 0.85rem;
    color: #aaa;
    font-family: monospace;
  }

  .scene-list::-webkit-scrollbar {
    width: 8px;
  }

  .scene-list::-webkit-scrollbar-track {
    background: #1a1a1a;
  }

  .scene-list::-webkit-scrollbar-thumb {
    background: #4a4a4a;
    border-radius: 4px;
  }

  .scene-list::-webkit-scrollbar-thumb:hover {
    background: #5a5a5a;
  }
</style>
