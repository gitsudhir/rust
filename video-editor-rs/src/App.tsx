import { useState } from "react";
import { open, save } from "@tauri-apps/plugin-dialog";
import { invoke, convertFileSrc } from "@tauri-apps/api/core";
import {
  Container,
  Grid,
  Button,
  TextField,
  CircularProgress,
  Alert,
  Box,
  Typography,
  ThemeProvider,
  createTheme,
  CssBaseline,
  ButtonGroup,
} from "@mui/material";
import MovieIcon from "@mui/icons-material/Movie";
import MusicNoteIcon from "@mui/icons-material/MusicNote";
import ImageIcon from "@mui/icons-material/Image";
import DownloadIcon from "@mui/icons-material/Download";
import AudiotrackIcon from "@mui/icons-material/Audiotrack";
import ClearIcon from "@mui/icons-material/Clear";

const darkTheme = createTheme({
  palette: {
    mode: "dark",
  },
});

function App() {
  const [imageUrl, setImageUrl] = useState("");
  const [musicUrl, setMusicUrl] = useState("");
  const [audioPath, setAudioPath] = useState("");
  const [videoUrl, setVideoUrl] = useState("");
  const [isCreating, setIsCreating] = useState(false);
  const [error, setError] = useState("");
  const [imageError, setImageError] = useState("");
  const [audioError, setAudioError] = useState("");

  async function selectImage() {
    const selected = await open({
      multiple: false,
      filters: [{ name: "Image", extensions: ["png", "jpg", "jpeg"] }],
    });
    if (selected) {
      setImageUrl(selected);
      setImageError("");
      console.log("Selected image:", selected);
    }
  }

  async function selectAudio() {
    const selected = await open({
      multiple: false,
      filters: [{ name: "Audio", extensions: ["mp3", "wav", "ogg"] }],
    });
    if (selected) {
      setAudioPath(selected);
      setMusicUrl(""); // Clear music URL when local audio is selected
      setAudioError("");
      console.log("Selected audio:", selected);
    }
  }

  async function createVideo() {
    const audioSource = audioPath || musicUrl;
    if (!imageUrl || !audioSource) {
      setError("Please select an image and provide an audio source.");
      return;
    }

    setIsCreating(true);
    setError("");
    setVideoUrl("");

    try {
      // Ask user where to save the file
      const savePath = await save({
        filters: [{
          name: "MP4 Video",
          extensions: ["mp4"]
        }]
      });
      
      if (!savePath) {
        setIsCreating(false);
        return; // User cancelled save dialog
      }

      console.log("Creating video with image:", imageUrl, "and audio:", audioSource);
      const result = await invoke("create_video", {
        imagePath: imageUrl,
        audioUrl: audioSource,
        savePath: savePath
      });
      console.log("Video created successfully:", result);
      setVideoUrl(result as string);
    } catch (err) {
      console.error("Error creating video:", err);
      setError(err as string);
    } finally {
      setIsCreating(false);
    }
  }

  const audioSrc = audioPath ? convertFileSrc(audioPath) : musicUrl;

  return (
    <ThemeProvider theme={darkTheme}>
      <CssBaseline />
      <Container maxWidth="md" sx={{ mt: 4 }}>
        <Box sx={{ textAlign: "center", mb: 4 }}>
          <MovieIcon sx={{ fontSize: 40 }} />
          <Typography variant="h4" component="h1">
            Video Editor
          </Typography>
        </Box>

        <Grid container spacing={4}>
          <Grid size={{ xs: 12, md: 6 }}>
            <Box sx={{ display: "flex", flexDirection: "column", gap: 2 }}>
              <ButtonGroup fullWidth variant="contained">
                <Button onClick={selectImage} startIcon={<ImageIcon />}>
                  Select Image
                </Button>
                <Button onClick={selectAudio} startIcon={<AudiotrackIcon />}>
                  Select Audio
                </Button>
              </ButtonGroup>

              {audioPath ? (
                <Box sx={{ display: "flex", alignItems: "center", gap: 1 }}>
                  <Typography variant="body2" sx={{ flexGrow: 1 }}>
                    {audioPath.split("/").pop()}
                  </Typography>
                  <Button size="small" onClick={() => setAudioPath("")}>
                    <ClearIcon />
                  </Button>
                </Box>
              ) : (
                <TextField
                  label="Music URL"
                  variant="outlined"
                  fullWidth
                  value={musicUrl}
                  onChange={(e) => setMusicUrl(e.target.value)}
                  InputProps={{
                    startAdornment: <MusicNoteIcon sx={{ mr: 1 }} />,
                  }}
                />
              )}

              {audioSrc && (
                <audio 
                  controls 
                  src={audioSrc} 
                  style={{ width: "100%" }}
                  onError={(e) => {
                    console.error("Audio error:", e);
                    setAudioError("Failed to load audio file");
                  }}
                >
                  Your browser does not support the audio element.
                </audio>
              )}
              {audioError && <Alert severity="error">{audioError}</Alert>}

              <Button
                variant="contained"
                color="primary"
                onClick={createVideo}
                disabled={!imageUrl || (!musicUrl && !audioPath) || isCreating}
                startIcon={
                  isCreating ? <CircularProgress size={20} /> : <MovieIcon />
                }
              >
                {isCreating ? "Creating Video..." : "Create Video"}
              </Button>

              {error && <Alert severity="error">{error}</Alert>}
            </Box>
          </Grid>

          <Grid size={{ xs: 12, md: 6 }}>
            <Box
              sx={{
                display: "flex",
                flexDirection: "column",
                alignItems: "center",
                gap: 2,
              }}
            >
              {imageUrl && (
                <Box>
                  <Typography variant="h6">Selected Image</Typography>
                  <img
                    src={convertFileSrc(imageUrl)}
                    alt="Selected"
                    style={{ maxWidth: "100%", borderRadius: "8px" }}
                    onError={(e) => {
                      console.error("Image error:", e);
                      setImageError("Failed to load image file");
                    }}
                  />
                  {imageError && <Alert severity="error">{imageError}</Alert>}
                </Box>
              )}

              {videoUrl && (
                <Box>
                  <Typography variant="h6">Generated Video</Typography>
                  <video
                    controls
                    src={convertFileSrc(videoUrl)}
                    style={{ maxWidth: "100%", borderRadius: "8px" }}
                  />
                  <Button
                    variant="contained"
                    color="secondary"
                    href={convertFileSrc(videoUrl)}
                    download="video.mp4"
                    startIcon={<DownloadIcon />}
                    sx={{ mt: 1 }}
                  >
                    Download Video
                  </Button>
                </Box>
              )}
            </Box>
          </Grid>
        </Grid>
      </Container>
    </ThemeProvider>
  );
}

export default App;