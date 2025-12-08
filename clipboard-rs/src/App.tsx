import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { motion, AnimatePresence } from "framer-motion";
import {
  Container,
  Box,
  Typography,
  TextField,
  Button,
  Card,
  CardContent,
  List,
  ListItem,
  ListItemText,
  ListItemSecondaryAction,
  IconButton,
  Snackbar,
  Alert,
  AppBar,
  Toolbar,
  Tooltip,
  Chip,
  Paper,
  CircularProgress,
  Avatar,
  Link,
} from "@mui/material";
import {
  ContentCopy,
  ContentPaste,
  Delete,
  Refresh,
  Image as ImageIcon,
  TextFields,
  History as HistoryIcon,
} from "@mui/icons-material";

function App() {
  const [clipboardText, setClipboardText] = useState("");
  const [clipboardContent, setClipboardContent] = useState("");
  const [clipboardHistory, setClipboardHistory] = useState<string[]>([]);
  const [imageThumbnails, setImageThumbnails] = useState<Record<string, string>>({});
  const [snackbarOpen, setSnackbarOpen] = useState(false);
  const [snackbarMessage, setSnackbarMessage] = useState("");
  const [snackbarSeverity, setSnackbarSeverity] = useState<"success" | "error" | "warning" | "info">("success");
  const [loading, setLoading] = useState(false);

  const showSnackbar = (message: string, severity: "success" | "error" | "warning" | "info" = "success") => {
    setSnackbarMessage(message);
    setSnackbarSeverity(severity);
    setSnackbarOpen(true);
  };

  const handleSnackbarClose = () => {
    setSnackbarOpen(false);
  };

  async function readClipboard() {
    setLoading(true);
    try {
      const text = await invoke<string>("read_clipboard_text");
      // Extract the text part (first part before the timestamp)
      const textPart = text.split('|')[0];
      setClipboardContent(textPart);
      showSnackbar("Text pasted from clipboard!", "success");
    } catch (error) {
      // Try to read image if text reading fails
      try {
        const imageData = await invoke<string>("read_clipboard_image");
        setClipboardContent(imageData);
        showSnackbar("Image detected in clipboard!", "success");
      } catch (imageError) {
        showSnackbar("Failed to read clipboard: " + error, "error");
      }
    } finally {
      setLoading(false);
    }
  }

  async function writeClipboard() {
    if (!clipboardText.trim()) {
      showSnackbar("Please enter some text to copy", "warning");
      return;
    }

    setLoading(true);
    try {
      await invoke("write_clipboard_text", { text: clipboardText });
      // Save to history
      await invoke("save_clipboard_history", { text: clipboardText });
      // Refresh history
      loadHistory();
      showSnackbar("Text copied to clipboard!", "success");
    } catch (error) {
      showSnackbar("Failed to write clipboard: " + error, "error");
    } finally {
      setLoading(false);
    }
  }

  async function loadHistory() {
    try {
      const history = await invoke<string[]>("load_clipboard_history");
      setClipboardHistory(history);
      
      // Load thumbnails for image entries
      const newThumbnails: Record<string, string> = {};
      for (const item of history) {
        if (item.startsWith("[Image]")) {
          console.log("Processing image item:", item); // Debugging
          const parts = item.split('|');
          if (parts.length >= 3) {
            const filePath = parts[parts.length - 2]; // Second to last part is the file path
            console.log("File path:", filePath); // Debugging
            try {
              const thumbnail = await invoke<string>("get_image_thumbnail", { filePath: filePath });
              newThumbnails[item] = thumbnail;
            } catch (error) {
              console.error("Failed to load thumbnail for", item, "with file path", filePath, error);
              // Use a placeholder if thumbnail fails to load
              newThumbnails[item] = "🖼️";
            }
          }
        }
      }
      console.log("Image thumbnails:", newThumbnails); // Debugging
      setImageThumbnails(newThumbnails);
    } catch (error) {
      console.error("Failed to load clipboard history:", error);
      showSnackbar("Failed to load clipboard history", "error");
    }
  }

  async function clearHistory() {
    try {
      await invoke("clear_clipboard_history");
      setClipboardHistory([]);
      setImageThumbnails({});
      showSnackbar("Clipboard history cleared!", "success");
    } catch (error) {
      console.error("Failed to clear clipboard history:", error);
      showSnackbar("Failed to clear clipboard history", "error");
    }
  }

  async function copyFromHistory(item: string) {
    if (item.startsWith("[Image]")) {
      setLoading(true);
      try {
        // Extract file path from the image entry (it's the second to last part)
        const parts = item.split('|');
        if (parts.length >= 3) {
          const filePath = parts[parts.length - 2]; // Second to last part is the file path
          await invoke("copy_image_from_file_to_clipboard", { file_path: filePath });
          showSnackbar("Image copied from history!", "success");
        } else {
          showSnackbar("Invalid image entry format", "error");
        }
      } catch (error) {
        showSnackbar("Failed to copy image from history: " + error, "error");
      } finally {
        setLoading(false);
      }
      return;
    }

    setLoading(true);
    try {
      // Extract the text part (first part before the timestamp)
      const textPart = item.split('|')[0];
      await invoke("write_clipboard_text", { text: textPart });
      setClipboardContent(textPart);
      showSnackbar("Text copied from history!", "success");
    } catch (error) {
      showSnackbar("Failed to copy from history: " + error, "error");
    } finally {
      setLoading(false);
    }
  }

  // Load history when component mounts
  useEffect(() => {
    loadHistory();
    
    // Listen for clipboard updates
    const unlisten = listen('clipboard-update', () => {
      loadHistory();
    });
    
    // Cleanup listener on component unmount
    return () => {
      unlisten.then((u) => u());
    };
  }, []);

  return (
    <Box sx={{ flexGrow: 1, minHeight: "100vh", bgcolor: "background.default", display: 'flex', flexDirection: 'column' }}>
      <AppBar position="static" color="primary" sx={{ 
        background: 'linear-gradient(45deg, #2196F3 30%, #21CBF3 90%)',
        boxShadow: '0 3px 5px 2px rgba(33, 203, 243, .3)',
      }}>
        <Toolbar sx={{ minHeight: 56 }}>
          <Typography variant="h6" component="div" sx={{ flexGrow: 1, fontWeight: 'bold' }}>
            Clipboard Manager
          </Typography>
          <Chip label="Beta" color="secondary" size="small" sx={{ fontWeight: 'bold' }} />
        </Toolbar>
      </AppBar>

      <Container maxWidth="md" sx={{ mt: 2, mb: 2 }}>
        {/* Clipboard History Card - Prominent placement at top */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.5 }}
        >
          <Card elevation={4} sx={{ mb: 2, background: 'linear-gradient(to bottom right, #ffffff, #f5f5f5)' }}>
            <CardContent>
              <Box sx={{ display: "flex", justifyContent: "space-between", alignItems: "center", mb: 2 }}>
                <Typography variant="h5" gutterBottom sx={{ display: "flex", alignItems: "center", gap: 1, fontWeight: 'bold', color: '#1976d2' }}>
                  <HistoryIcon /> Clipboard History
                </Typography>
                <Box>
                  <Tooltip title="Refresh History">
                    <IconButton onClick={loadHistory} color="primary" sx={{ 
                      background: 'linear-gradient(45deg, #2196F3 30%, #21CBF3 90%)',
                      color: 'white',
                      '&:hover': {
                        background: 'linear-gradient(45deg, #1976D2 30%, #03A9F4 90%)',
                      },
                      mr: 1
                    }}>
                      <Refresh />
                    </IconButton>
                  </Tooltip>
                  <Tooltip title="Clear History">
                    <IconButton onClick={clearHistory} color="error" sx={{ 
                      background: 'linear-gradient(45deg, #f44336 30%, #ff9800 90%)',
                      color: 'white',
                      '&:hover': {
                        background: 'linear-gradient(45deg, #d32f2f 30%, #ef6c00 90%)',
                      }
                    }}>
                      <Delete />
                    </IconButton>
                  </Tooltip>
                </Box>
              </Box>
              
              {clipboardHistory.length > 0 ? (
                <List sx={{ maxHeight: 400, overflow: "auto" }}>
                  <AnimatePresence>
                    {clipboardHistory.map((item, index) => (
                      <motion.div
                        key={index}
                        initial={{ opacity: 0, x: -20 }}
                        animate={{ opacity: 1, x: 0 }}
                        exit={{ opacity: 0, x: 20 }}
                        transition={{ duration: 0.3 }}
                        layout
                      >
                        <ListItem
                          divider={index !== clipboardHistory.length - 1}
                          sx={{
                            bgcolor: index % 2 === 0 ? "background.default" : "#f8f9fa",
                            "&:hover": {
                              bgcolor: "action.hover",
                              transform: "translateX(5px)",
                              transition: "transform 0.2s ease-in-out",
                            },
                            py: 1.5,
                            borderRadius: 1,
                            mb: 0.5,
                          }}
                        >
                          <Avatar sx={{ 
                            mr: 2, 
                            width: 48, 
                            height: 48,
                            background: 'linear-gradient(45deg, #2196F3 30%, #21CBF3 90%)',
                            boxShadow: '0 3px 5px 2px rgba(33, 203, 243, .3)',
                          }}>
                            {item.startsWith("[Image]") ? (
                              imageThumbnails[item] ? (
                                <img 
                                  src={imageThumbnails[item]} 
                                  alt="Thumbnail" 
                                  style={{ width: '100%', height: '100%', objectFit: 'cover' }} 
                                />
                              ) : (
                                <ImageIcon />
                              )
                            ) : (
                              <TextFields />
                            )}
                          </Avatar>
                          <ListItemText
                            primary={
                              <Box sx={{ display: "flex", alignItems: "center", gap: 1 }}>
                                {item.startsWith("[Image]") ? (
                                  <>
                                    <Typography component="span" sx={{ fontWeight: 'medium', fontSize: '1.1rem' }}>
                                      🖼️ Image
                                    </Typography>
                                    <Chip 
                                      label={item.split('|')[0].replace('[Image] ', '')} 
                                      size="small" 
                                      sx={{ 
                                        background: 'linear-gradient(45deg, #4CAF50 30%, #8BC34A 90%)',
                                        color: 'white',
                                        fontWeight: 'bold',
                                        fontSize: '0.7rem'
                                      }} 
                                    />
                                  </>
                                ) : (
                                  <>
                                    <Typography 
                                      component="span" 
                                      sx={{ 
                                        overflow: "hidden", 
                                        textOverflow: "ellipsis", 
                                        whiteSpace: "nowrap",
                                        maxWidth: { xs: "200px", sm: "400px" },
                                        fontWeight: 'medium',
                                        fontSize: '1.1rem'
                                      }}
                                    >
                                      {item.split('|')[0]}
                                    </Typography>
                                  </>
                                )}
                              </Box>
                            }
                            secondary={
                              <Box sx={{ display: "flex", alignItems: "center", gap: 1, mt: 0.5 }}>
                                <Chip 
                                  label={item.startsWith("[Image]") ? "IMAGE" : "TEXT"} 
                                  size="small" 
                                  sx={{ 
                                    background: item.startsWith("[Image]") ? 
                                      'linear-gradient(45deg, #FF9800 30%, #FFC107 90%)' : 
                                      'linear-gradient(45deg, #2196F3 30%, #21CBF3 90%)',
                                    color: 'white',
                                    fontWeight: 'bold',
                                    fontSize: '0.6rem',
                                    height: '20px'
                                  }} 
                                />
                                <Typography component="span" variant="caption" sx={{ color: '#666', fontStyle: 'italic' }}>
                                  #{index + 1}
                                </Typography>
                                <Typography component="span" variant="caption" sx={{ color: '#999' }}>
                                  •
                                </Typography>
                                <Typography component="span" variant="caption" sx={{ color: '#999', fontSize: '0.7rem' }}>
                                  {(() => {
                                    // Parse timestamp from the item
                                    const parts = item.split('|');
                                    const timestamp = parts[parts.length - 1];
                                    if (timestamp && !isNaN(parseInt(timestamp))) {
                                      const date = new Date(parseInt(timestamp) * 1000);
                                      return date.toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'});
                                    }
                                    return new Date().toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'});
                                  })()}
                                </Typography>
                              </Box>
                            }
                          />
                          <ListItemSecondaryAction>
                            <Tooltip title="Copy to Clipboard">
                              <IconButton
                                edge="end"
                                aria-label="copy"
                                onClick={() => copyFromHistory(item)}
                                disabled={loading}
                                sx={{ 
                                  ml: 1,
                                  background: 'linear-gradient(45deg, #4CAF50 30%, #8BC34A 90%)',
                                  color: 'white',
                                  '&:hover': {
                                    background: 'linear-gradient(45deg, #388E3C 30%, #689F38 90%)',
                                  },
                                  '&.Mui-disabled': {
                                    background: 'rgba(0, 0, 0, 0.12)',
                                    color: 'rgba(0, 0, 0, 0.26)',
                                  }
                                }}
                              >
                                <ContentCopy />
                              </IconButton>
                            </Tooltip>
                          </ListItemSecondaryAction>
                        </ListItem>
                      </motion.div>
                    ))}
                  </AnimatePresence>
                </List>
              ) : (
                <Box sx={{ textAlign: "center", py: 6 }}>
                  <ImageIcon sx={{ fontSize: 64, color: "#1976d2", mb: 2 }} />
                  <Typography variant="h6" color="text.secondary" gutterBottom sx={{ fontWeight: 'medium' }}>
                    No clipboard history yet
                  </Typography>
                  <Typography variant="body2" color="text.secondary">
                    Copy some text or images to see them appear here
                  </Typography>
                </Box>
              )}
            </CardContent>
          </Card>
        </motion.div>

        {/* Clipboard Operations Card */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.5, delay: 0.1 }}
        >
          <Card elevation={4} sx={{ background: 'linear-gradient(to bottom right, #ffffff, #f5f5f5)' }}>
            <CardContent>
              <Typography variant="h5" gutterBottom sx={{ display: "flex", alignItems: "center", gap: 1, fontWeight: 'bold', color: '#1976d2' }}>
                <ContentCopy /> Clipboard Operations
              </Typography>
              
              <TextField
                fullWidth
                label="Enter text to copy"
                variant="outlined"
                value={clipboardText}
                onChange={(e) => setClipboardText(e.target.value)}
                sx={{ mb: 2 }}
                disabled={loading}
              />
              
              <Box sx={{ display: "flex", gap: 2, flexWrap: "wrap" }}>
                <Button
                  variant="contained"
                  color="primary"
                  startIcon={<ContentCopy />}
                  onClick={writeClipboard}
                  disabled={loading}
                  sx={{ 
                    flex: 1, 
                    minWidth: 120,
                    background: 'linear-gradient(45deg, #2196F3 30%, #21CBF3 90%)',
                    '&:hover': {
                      background: 'linear-gradient(45deg, #1976D2 30%, #03A9F4 90%)',
                    }
                  }}
                >
                  Copy Text
                </Button>
                
                <Button
                  variant="outlined"
                  color="secondary"
                  startIcon={loading ? <CircularProgress size={20} /> : <ContentPaste />}
                  onClick={readClipboard}
                  disabled={loading}
                  sx={{ 
                    flex: 1, 
                    minWidth: 120,
                    borderColor: 'linear-gradient(45deg, #2196F3 30%, #21CBF3 90%)',
                  }}
                >
                  Paste
                </Button>
              </Box>
              
              {clipboardContent && (
                <motion.div
                  initial={{ opacity: 0, height: 0 }}
                  animate={{ opacity: 1, height: "auto" }}
                  exit={{ opacity: 0, height: 0 }}
                  transition={{ duration: 0.3 }}
                >
                  <Paper elevation={1} sx={{ mt: 2, p: 2, bgcolor: "background.paper", borderRadius: 2 }}>
                    <Typography variant="body1" sx={{ wordBreak: "break-word" }}>
                      {clipboardContent}
                    </Typography>
                  </Paper>
                </motion.div>
              )}
            </CardContent>
          </Card>
        </motion.div>
      </Container>

      {/* Footer with developer info */}
      <Box 
        sx={{ 
          py: 2, 
          px: 2, 
          mt: 'auto',
          bgcolor: 'background.paper',
          borderTop: '1px solid rgba(0, 0, 0, 0.12)',
          textAlign: 'center'
        }}
      >
        <Typography variant="body2" color="text.secondary">
          Developed by Sudhir Kumar •{' '}
          <Link 
            href="https://sudhirkumar.in" 
            target="_blank" 
            rel="noopener noreferrer"
            sx={{ 
              color: 'primary.main', 
              textDecoration: 'none',
              '&:hover': {
                textDecoration: 'underline'
              }
            }}
          >
            sudhirkumar.in
          </Link>
        </Typography>
      </Box>

      {/* Snackbar for notifications */}
      <Snackbar
        open={snackbarOpen}
        autoHideDuration={4000}
        onClose={handleSnackbarClose}
        anchorOrigin={{ vertical: "bottom", horizontal: "center" }}
      >
        <Alert
          onClose={handleSnackbarClose}
          severity={snackbarSeverity}
          sx={{ width: "100%" }}
        >
          {snackbarMessage}
        </Alert>
      </Snackbar>
    </Box>
  );
}

export default App;