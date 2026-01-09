<script>
  export let videoPath = '';
</script>

<div class="video-player">
  <video controls src={"file://" + videoPath}>
    Your browser does not support the video tag.
  </video>
</div>

<style>
  .video-player {
    background: #2a2a2a;
    padding: 1rem;
    border-radius: 4px;
  }

  video {
    width: 100%;
    max-height: 400px;
    border-radius: 4px;
  }
</style>
