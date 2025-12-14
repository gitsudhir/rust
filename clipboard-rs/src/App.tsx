import { useState, useEffect, useCallback, useMemo } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { motion, AnimatePresence } from "framer-motion";
import createAppTheme from "./theme";
import {
  ThemeProvider,
  CssBaseline,
} from "@mui/material";
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
  InputAdornment,
  Switch,
  FormControlLabel,
  Accordion,
  AccordionSummary,
  AccordionDetails,
  Slider,
  Modal,
  Backdrop,
  Fade,
} from "@mui/material";
import {
  ContentCopy,
  ContentPaste,
  Delete,
  Refresh,
  Image as ImageIcon,
  TextFields,
  Search as SearchIcon,
  Star as StarIcon,
  StarBorder as StarBorderIcon,
  ExpandMore as ExpandMoreIcon,
  Settings as SettingsIcon,
  BarChart as BarChartIcon,
  Close as CloseIcon,
  Brightness4 as Brightness4Icon,
  Brightness7 as Brightness7Icon,
} from "@mui/icons-material";

function App() {
  // Helper function to truncate text and show first few lines
  const truncateTextPreview = (text: string, maxLines: number = 4, maxCharsPerLine: number = 100) => {
    if (!text) return '';
    
    // Split text into lines
    const lines = text.split('\n');
    
    // Take only the first maxLines lines
    const previewLines = lines.slice(0, maxLines);
    
    // Truncate each line if it's too long
    const truncatedLines = previewLines.map(line => {
      if (line.length > maxCharsPerLine) {
        return line.substring(0, maxCharsPerLine) + '...';
      }
      return line;
    });
    
    // Join the lines back together
    let result = truncatedLines.join('\n');
    
    // Add indicator if there are more lines
    if (lines.length > maxLines) {
      result += `\n... (${lines.length - maxLines} more lines)`;
    }
    
    return result;
  };

  const [clipboardText, setClipboardText] = useState("");
  const [clipboardContent, setClipboardContent] = useState("");
  const [clipboardHistory, setClipboardHistory] = useState<any[]>([]);
  const [filteredHistory, setFilteredHistory] = useState<any[]>([]);
  const [searchQuery, setSearchQuery] = useState("");
  const [favorites, setFavorites] = useState<string[]>([]);
  const [showFavoritesOnly, setShowFavoritesOnly] = useState(false);
  const [autoDeleteEnabled, setAutoDeleteEnabled] = useState(false);
  const [autoDeleteDays, setAutoDeleteDays] = useState(7);
  const [imageThumbnails, setImageThumbnails] = useState<Record<string, string>>({});
  const [selectedImage, setSelectedImage] = useState<{filePath: string, dimensions: string} | null>(null);
  const [statistics, setStatistics] = useState<any>(null);
  const [showStatistics, setShowStatistics] = useState(false);
  const [darkMode, setDarkMode] = useState(false);
  const [snackbarOpen, setSnackbarOpen] = useState(false);
  const [snackbarMessage, setSnackbarMessage] = useState("");
  const [snackbarSeverity, setSnackbarSeverity] = useState<"success" | "error" | "warning" | "info">("success");
  const [loading, setLoading] = useState(false);

  const theme = useMemo(() => createAppTheme(darkMode ? 'dark' : 'light'), [darkMode]);

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
      const history: any[] = await invoke("load_clipboard_history");
      setClipboardHistory(history);
      setFilteredHistory(history);
      
      // Load thumbnails for image entries
      const newThumbnails: Record<string, string> = {};
      for (const item of history) {
        const itemContent = typeof item === 'string' ? item : item.content;
        if (itemContent.startsWith("[Image]")) {
          console.log("Processing image item:", itemContent); // Debugging
          const parts = itemContent.split('|');
          if (parts.length >= 3) {
            const filePath = parts[parts.length - 2]; // Second to last part is the file path
            console.log("File path:", filePath); // Debugging
            try {
              const thumbnail = await invoke<string>("get_image_thumbnail", { filePath: filePath });
              newThumbnails[itemContent] = thumbnail;
            } catch (error) {
              console.error("Failed to load thumbnail for", itemContent, "with file path", filePath, error);
              // Use a placeholder if thumbnail fails to load
              newThumbnails[itemContent] = "🖼️";
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

  async function loadFavorites() {
    try {
      const favs = await invoke<string[]>("load_favorites");
      setFavorites(favs);
    } catch (error) {
      console.error("Failed to load favorites:", error);
      showSnackbar("Failed to load favorites", "error");
    }
  }

  async function toggleFavorite(item: any) {
    const itemContent = typeof item === 'string' ? item : item.content;
    
    try {
      const isNowFavorite = await invoke<boolean>("toggle_favorite", { itemContent });
      
      // Update the favorites list
      if (isNowFavorite) {
        setFavorites(prev => [...prev, itemContent]);
        showSnackbar("Item added to favorites!", "success");
      } else {
        setFavorites(prev => prev.filter(fav => fav !== itemContent));
        showSnackbar("Item removed from favorites", "info");
      }
      
      // Reload history to reflect changes
      loadHistory();
    } catch (error) {
      console.error("Failed to toggle favorite:", error);
      showSnackbar("Failed to update favorite status", "error");
    }
  }

  async function viewFullImage(item: any) {
    const itemContent = typeof item === 'string' ? item : item.content;
    
    if (!itemContent.startsWith("[Image]")) return;
    
    try {
      // Extract file path and dimensions from the item
      const parts = itemContent.split('|');
      if (parts.length >= 3) {
        const filePath = parts[parts.length - 2];
        const dimensions = parts[0].replace('[Image] ', '');
        setSelectedImage({ filePath, dimensions });
      }
    } catch (error) {
      console.error("Failed to view full image:", error);
      showSnackbar("Failed to view full image", "error");
    }
  }

  async function exportHistory() {
    try {
      const jsonData = await invoke<string>("export_history");
      const blob = new Blob([jsonData], { type: 'application/json' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `clipboard-history-${new Date().toISOString().slice(0, 10)}.json`;
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      URL.revokeObjectURL(url);
      showSnackbar("History exported successfully!", "success");
    } catch (error) {
      console.error("Failed to export history:", error);
      showSnackbar("Failed to export history", "error");
    }
  }

  async function importHistory(event: React.ChangeEvent<HTMLInputElement>) {
    const file = event.target.files?.[0];
    if (!file) return;

    try {
      const reader = new FileReader();
      reader.onload = async (e) => {
        const jsonData = e.target?.result as string;
        if (jsonData) {
          const addedCount = await invoke<number>("import_history", { jsonData });
          showSnackbar(`Imported ${addedCount} items successfully!`, "success");
          loadHistory();
        }
      };
      reader.readAsText(file);
    } catch (error) {
      console.error("Failed to import history:", error);
      showSnackbar("Failed to import history", "error");
    }
    
    // Reset the file input
    event.target.value = '';
  }

  async function removeTagFromItem(item: string, tag: string) {
    try {
      await invoke("remove_tag_from_item", { itemContent: item, tag });
      showSnackbar(`Tag "${tag}" removed successfully!`, "success");
      loadHistory();
    } catch (error) {
      console.error("Failed to remove tag:", error);
      showSnackbar("Failed to remove tag", "error");
    }
  }

  async function loadStatistics() {
    try {
      const stats = await invoke<any>("get_clipboard_statistics");
      setStatistics(stats);
    } catch (error) {
      console.error("Failed to load statistics:", error);
      showSnackbar("Failed to load statistics", "error");
    }
  }

  // Helper function to truncate text and show first few lines

  // Handle search input changes and filters
  useEffect(() => {
    let results = clipboardHistory;
    
    // Apply favorites filter if enabled
    if (showFavoritesOnly) {
      results = results.filter(item => {
        const itemObj = typeof item === 'string' ? { content: item, is_favorite: false } : item;
        return itemObj.is_favorite;
      });
    }
    
    // Apply search filter
    if (searchQuery.trim()) {
      results = results.filter(item => {
        const itemObj = typeof item === 'string' ? { content: item } : item;
        // Extract the text part (before the timestamp)
        const textPart = itemObj.content.split('|')[0];
        return textPart.toLowerCase().includes(searchQuery.toLowerCase());
      });
    }
    
    setFilteredHistory(results);
  }, [searchQuery, clipboardHistory, showFavoritesOnly]);

  // Load favorites when component mounts
  useEffect(() => {
    loadFavorites();
  }, []);

  // Handle keyboard shortcuts
  const handleKeyDown = useCallback((event: KeyboardEvent) => {
    // Ctrl/Cmd + Shift + C - Copy current text to clipboard
    if ((event.ctrlKey || event.metaKey) && event.shiftKey && event.key === 'C') {
      event.preventDefault();
      if (clipboardText.trim()) {
        writeClipboard();
      }
    }
    
    // Ctrl/Cmd + Shift + V - Paste from clipboard
    if ((event.ctrlKey || event.metaKey) && event.shiftKey && event.key === 'V') {
      event.preventDefault();
      readClipboard();
    }
    
    // Ctrl/Cmd + Shift + R - Refresh history
    if ((event.ctrlKey || event.metaKey) && event.shiftKey && event.key === 'R') {
      event.preventDefault();
      loadHistory();
    }
    
    // Ctrl/Cmd + Shift + F - Focus search
    if ((event.ctrlKey || event.metaKey) && event.shiftKey && event.key === 'F') {
      event.preventDefault();
      const searchInput = document.querySelector('input[placeholder*="Search"]');
      if (searchInput instanceof HTMLElement) {
        searchInput.focus();
      }
    }
  }, [clipboardText, writeClipboard, readClipboard, loadHistory]);

  useEffect(() => {
    document.addEventListener('keydown', handleKeyDown);
    return () => {
      document.removeEventListener('keydown', handleKeyDown);
    };
  }, [handleKeyDown]);

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

  async function copyFromHistory(item: any) {
    const itemContent = typeof item === 'string' ? item : item.content;
    
    if (itemContent.startsWith("[Image]")) {
      setLoading(true);
      try {
        // Extract file path from the image entry (it's the second to last part)
        const parts = itemContent.split('|');
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
      const textPart = itemContent.split('|')[0];
      await invoke("write_clipboard_text", { text: textPart });
      setClipboardContent(textPart);
      showSnackbar("Text copied from history!", "success");
    } catch (error) {
      showSnackbar("Failed to copy from history: " + error, "error");
    } finally {
      setLoading(false);
    }
  }

  async function cleanupOldItems() {
    try {
      const maxAgeSeconds = autoDeleteDays * 24 * 60 * 60;
      const removedCount = await invoke<number>("cleanup_old_items", { maxAgeSeconds });
      showSnackbar(`Removed ${removedCount} old items from history`, "success");
      loadHistory();
    } catch (error) {
      console.error("Failed to cleanup old items:", error);
      showSnackbar("Failed to cleanup old items", "error");
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
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <Box sx={{ flexGrow: 1, minHeight: "100vh", bgcolor: "background.default", display: 'flex', flexDirection: 'column' }}>
        <AppBar position="static" color="primary" sx={{ 
          background: (theme) => `linear-gradient(45deg, ${theme.palette.primary.main} 30%, ${theme.palette.secondary.main} 90%)`,
          boxShadow: (theme) => theme.shadows[2],
          height: { xs: 40, sm: 48 },
        }}>
          <Toolbar sx={{ minHeight: 'inherit !important', py: 0.5 }}>
            <Typography variant="body1" component="div" sx={{ flexGrow: 1, fontWeight: '500', fontSize: { xs: '0.85rem', sm: '0.95rem' }, color: 'white' }}>
              Clipboard Manager
            </Typography>
            <Tooltip title={`Switch to ${darkMode ? 'light' : 'dark'} mode`}>
              <IconButton 
                onClick={() => setDarkMode(!darkMode)}
                sx={{ 
                  color: 'white',
                  '&:hover': {
                    backgroundColor: darkMode ? 'rgba(255, 255, 255, 0.1)' : 'rgba(0, 0, 0, 0.1)'
                  }
                }}
              >
                {darkMode ? <Brightness7Icon sx={{ fontSize: { xs: '1rem', sm: '1.25rem' } }} /> : <Brightness4Icon sx={{ fontSize: { xs: '1rem', sm: '1.25rem' } }} />}
              </IconButton>
            </Tooltip>
            <Chip label="Beta" color="secondary" size="small" sx={{ fontWeight: '500', fontSize: { xs: '0.55rem', sm: '0.65rem' }, height: { xs: 16, sm: 20 } }} />
          </Toolbar>
        </AppBar>

      <Container maxWidth={false} sx={{ mb: { xs: 1, sm: 2 }, px: 0 }}>
        {/* Clipboard History Card - Prominent placement at top */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.5 }}
          style={{ margin: 0, padding: 0 }}
        >
          <CardContent sx={{ py: { xs: 1, sm: 2 }, px: 0 }}>
              <AppBar position="static" color="primary" sx={{ 
                background: (theme) => `linear-gradient(45deg, ${theme.palette.primary.main} 30%, ${theme.palette.secondary.main} 90%)`,
                boxShadow: (theme) => theme.shadows[2],
                height: { xs: 40, sm: 48 },
                mb: 2,
                borderRadius: 0,
              }}>
                <Toolbar sx={{ minHeight: 'inherit !important', py: 0.5 }}>
                  <Typography variant="body1" component="div" sx={{ flexGrow: 1, fontWeight: '500', fontSize: { xs: '0.85rem', sm: '0.95rem' }, color: 'white' }}>
                    Clipboard History
                  </Typography>
                  <Box>
                    <Tooltip title="Show Statistics">
                      <IconButton 
                        onClick={() => {
                          loadStatistics();
                          setShowStatistics(true);
                        }} 
                        sx={{ 
                          color: 'white',
                          mr: 1,
                          '&:hover': {
                            backgroundColor: darkMode ? 'rgba(255, 255, 255, 0.1)' : 'rgba(0, 0, 0, 0.1)'
                          }
                        }}
                      >
                        <BarChartIcon />
                      </IconButton>
                    </Tooltip>
                    <Tooltip title="Refresh History (Ctrl+Shift+R)">
                      <IconButton onClick={loadHistory} sx={{ 
                        color: 'white',
                        mr: 1,
                        '&:hover': {
                          backgroundColor: darkMode ? 'rgba(255, 255, 255, 0.1)' : 'rgba(0, 0, 0, 0.1)'
                        }
                      }}>
                        <Refresh />
                      </IconButton>
                    </Tooltip>
                    <Tooltip title="Clear History">
                      <IconButton onClick={clearHistory} sx={{ 
                        color: 'white',
                        '&:hover': {
                          backgroundColor: darkMode ? 'rgba(255, 255, 255, 0.1)' : 'rgba(0, 0, 0, 0.1)'
                        }
                      }}>
                        <Delete />
                      </IconButton>
                    </Tooltip>
                  </Box>
                </Toolbar>
              </AppBar>
              
              {/* Search Field */}
              <Box sx={{ display: 'flex', gap: 2, mb: 2, px: 0 }}>
                <TextField
                  fullWidth
                  label="Search clipboard history..."
                  variant="outlined"
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                  InputProps={{
                    startAdornment: (
                      <InputAdornment position="start">
                        <SearchIcon sx={{ fontSize: '1rem' }} />
                      </InputAdornment>
                    ),
                  }}
                  sx={{ 
                    fontSize: { xs: '0.75rem', sm: '0.85rem' },
                    '& .MuiOutlinedInput-root': {
                      height: { xs: 32, sm: 36 }
                    }
                  }}
                  title="Search clipboard history (Ctrl+Shift+F)"
                />
                <FormControlLabel
                  control={
                    <Switch
                      checked={showFavoritesOnly}
                      onChange={(e) => setShowFavoritesOnly(e.target.checked)}
                      color="primary"
                    />
                  }
                  label="Favorites Only"
                />
              </Box>
              
              {filteredHistory.length > 0 ? (
                <List sx={{ maxHeight: 400, overflow: "auto", px: 0, py: 0, mx: 0 }}>
                  <AnimatePresence>
                    {filteredHistory.map((item, index) => (
                      <motion.div
                        key={index}
                        initial={{ opacity: 0, x: -20 }}
                        animate={{ opacity: 1, x: 0 }}
                        exit={{ opacity: 0, x: 20 }}
                        transition={{ duration: 0.3 }}
                        layout
                      >
                        <ListItem
                          divider={index !== filteredHistory.length - 1}
                          sx={{
                            bgcolor: index % 2 === 0 ? "background.default" : "background.paper",
                            "&:hover": {
                              bgcolor: "action.hover",
                              transform: "translateX(2px)",
                              transition: "transform 0.1s ease-in-out",
                            },
                            py: { xs: 0.75, sm: 1 },
                            px: 0,
                            borderRadius: 0.5,
                            mb: 0.25,
                          }}
                        >
                          <Avatar 
                            sx={{ 
                              mr: 1.5, 
                              width: { xs: 40, sm: 48, md: 56 }, 
                              height: { xs: 40, sm: 48, md: 56 },
                              background: 'linear-gradient(45deg, #2196F3 30%, #21CBF3 90%)',
                              boxShadow: '0 1px 3px rgba(0,0,0,0.1)',
                              cursor: (typeof item === 'string' ? item : item.content).startsWith("[Image]") ? 'pointer' : 'default',
                            }}
                            onClick={() => (typeof item === 'string' ? item : item.content).startsWith("[Image]") && viewFullImage(item)}
                          >
                            {(typeof item === 'string' ? item : item.content).startsWith("[Image]") ? (
                              imageThumbnails[typeof item === 'string' ? item : item.content] ? (
                                <img 
                                  src={imageThumbnails[typeof item === 'string' ? item : item.content]} 
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
                              <Box sx={{ display: "flex", alignItems: "flex-start", gap: 1, width: '100%' }}>
                                {(typeof item === 'string' ? item : item.content).startsWith("[Image]") ? (
                                  <>
                                    <Typography component="span" sx={{ fontWeight: 'medium', fontSize: '1.1rem' }}>
                                      🖼️ Image
                                    </Typography>
                                    <Chip 
                                      label={(typeof item === 'string' ? item : item.content).split('|')[0].replace('[Image] ', '')} 
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
                                      component="div"
                                      sx={{ 
                                        overflow: "hidden", 
                                        textOverflow: "ellipsis", 
                                        whiteSpace: "pre-wrap",
                                        width: "100%",
                                        fontWeight: 'normal',
                                        fontSize: { xs: '0.75rem', sm: '0.8rem', md: '0.85rem' },
                                        fontFamily: 'monospace',
                                        lineHeight: 1.3,
                                      }}
                                    >
                                      {truncateTextPreview((typeof item === 'string' ? item : item.content).split('|')[0])}
                                    </Typography>
                                  </>
                                )}
                              </Box>
                            }
                            secondary={
                              <Box sx={{ display: "flex", flexDirection: "column", gap: 1 }}>
                                <Box sx={{ display: "flex", alignItems: "center", gap: 1 }}>
                                  <Chip 
                                    label={(typeof item === 'string' ? item : item.content).startsWith("[Image]") ? "IMAGE" : "TEXT"} 
                                    size="small" 
                                    sx={{ 
                                      background: (typeof item === 'string' ? item : item.content).startsWith("[Image]") ? 
                                        'linear-gradient(45deg, #FF9800 30%, #FFC107 90%)' : 
                                        'linear-gradient(45deg, #2196F3 30%, #21CBF3 90%)',
                                      color: 'white',
                                      fontWeight: '500',
                                      fontSize: '0.6rem',
                                      height: '18px'
                                    }} 
                                  />
                                  <Typography component="span" variant="caption" sx={{ color: 'text.secondary', fontStyle: 'italic' }}>
                                    #{index + 1}
                                  </Typography>
                                  <Typography component="span" variant="caption" sx={{ color: 'text.disabled' }}>
                                    •
                                  </Typography>
                                  <Typography component="span" variant="caption" sx={{ color: 'text.disabled', fontSize: '0.7rem' }}>
                                    {(() => {
                                      // Parse timestamp from the item
                                      const itemContent = typeof item === 'string' ? item : item.content;
                                      const parts = itemContent.split('|');
                                      const timestamp = parts[parts.length - 1];
                                      if (timestamp && !isNaN(parseInt(timestamp))) {
                                        const date = new Date(parseInt(timestamp) * 1000);
                                        return date.toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'});
                                      }
                                      return new Date().toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'});
                                    })()}
                                  </Typography>
                                </Box>
                                
                                {/* Tags Display */}
                                {typeof item !== 'string' && item.tags && item.tags.length > 0 && (
                                  <Box sx={{ display: "flex", gap: 0.5, flexWrap: "wrap" }}>
                                    {item.tags.map((tag: string, tagIndex: number) => (
                                      <Chip
                                        key={tagIndex}
                                        label={tag}
                                        size="small"
                                        onDelete={() => removeTagFromItem(typeof item === 'string' ? item : item.content, tag)}
                                        sx={{
                                          background: 'linear-gradient(45deg, #9C27B0 30%, #E91E63 90%)',
                                          color: 'white',
                                          fontWeight: '500',
                                          fontSize: '0.6rem',
                                          height: '18px',
                                          '& .MuiChip-deleteIcon': {
                                            color: 'white',
                                            '&:hover': {
                                              color: darkMode ? '#ffff00' : '#ffcc00'
                                            },
                                            fontSize: '0.8rem'
                                          }
                                        }}
                                      />
                                    ))}
                                  </Box>
                                )}
                              </Box>
                            }
                          />
                          <ListItemSecondaryAction>
                            <Tooltip title={(typeof item === 'string' ? item : item.content).startsWith("[Image]") ? 
                              (favorites.includes(typeof item === 'string' ? item : item.content) ? "Remove from Favorites" : "Add to Favorites") :
                              (favorites.includes(typeof item === 'string' ? item : item.content) ? "Remove from Favorites" : "Add to Favorites")}>
                              <IconButton
                                edge="end"
                                aria-label="favorite"
                                onClick={() => toggleFavorite(item)}
                                sx={{ 
                                  ml: 0.5,
                                  background: favorites.includes(typeof item === 'string' ? item : item.content) ? 
                                    'linear-gradient(45deg, #FFD700 30%, #FFA500 90%)' : 
                                    'linear-gradient(45deg, #e0e0e0 30%, #bdbdbd 90%)',
                                  color: favorites.includes(typeof item === 'string' ? item : item.content) ? 'white' : 'text.secondary',
                                  '&:hover': {
                                    background: favorites.includes(typeof item === 'string' ? item : item.content) ? 
                                      (darkMode ? 'linear-gradient(45deg, #FFC107 30%, #FF9800 90%)' : 'linear-gradient(45deg, #FFC107 30%, #FF9800 90%)') : 
                                      (darkMode ? 'linear-gradient(45deg, #757575 30%, #616161 90%)' : 'linear-gradient(45deg, #bdbdbd 30%, #9e9e9e 90%)'),
                                  },
                                  width: { xs: 28, sm: 32 },
                                  height: { xs: 28, sm: 32 },
                                  '& svg': {
                                    fontSize: { xs: '1rem', sm: '1.25rem' }
                                  }
                                }}
                              >
                                {favorites.includes(typeof item === 'string' ? item : item.content) ? <StarIcon sx={{ fontSize: '1rem' }} /> : <StarBorderIcon sx={{ fontSize: '1rem' }} />}
                              </IconButton>
                            </Tooltip>
                            <Tooltip title="Copy to Clipboard">
                              <IconButton
                                edge="end"
                                aria-label="copy"
                                onClick={() => copyFromHistory(item)}
                                disabled={loading}
                                sx={{ 
                                  ml: 0.5,
                                  background: (theme) => darkMode ? 
                                    `linear-gradient(45deg, ${theme.palette.success.dark} 30%, ${theme.palette.success.main} 90%)` :
                                    `linear-gradient(45deg, ${theme.palette.success.main} 30%, ${theme.palette.success.light} 90%)`,
                                  color: 'white',
                                  '&:hover': {
                                    background: (theme) => darkMode ?
                                      `linear-gradient(45deg, ${theme.palette.success.main} 30%, ${theme.palette.success.light} 90%)` :
                                      `linear-gradient(45deg, ${theme.palette.success.dark} 30%, ${theme.palette.success.main} 90%)`,
                                  },
                                  '&.Mui-disabled': {
                                    background: 'rgba(0, 0, 0, 0.12)',
                                    color: 'rgba(0, 0, 0, 0.26)',
                                  },
                                  width: { xs: 28, sm: 32 },
                                  height: { xs: 28, sm: 32 },
                                  '& svg': {
                                    fontSize: { xs: '1rem', sm: '1.25rem' }
                                  }
                                }}
                              >
                                <ContentCopy sx={{ fontSize: '1rem' }} />
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
                  <ImageIcon sx={{ fontSize: 64, color: "primary.main", mb: 2 }} />
                  <Typography variant="h6" color="text.secondary" gutterBottom sx={{ fontWeight: 'medium' }}>
                    {searchQuery ? "No matching items found" : "No clipboard history yet"}
                  </Typography>
                  <Typography variant="body2" color="text.secondary">
                    {searchQuery ? "Try a different search term" : "Copy some text or images to see them appear here"}
                  </Typography>
                </Box>
              )}
            </CardContent>
          {/*</Card>*/}
        </motion.div>

        {/* Clipboard Operations Card */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.5, delay: 0.1 }}
          style={{ margin: 0, padding: 0 }}
        >
          <Card elevation={4} sx={{ bgcolor: 'background.paper', borderRadius: 0, mx: 0 }}>
            <CardContent sx={{ py: { xs: 1, sm: 2 }, px: 0 }}>
              <Typography variant="body1" gutterBottom sx={{ display: "flex", alignItems: "center", gap: 1, fontWeight: '500', color: 'primary.main', fontSize: { xs: '0.95rem', sm: '1.1rem' } }}>
                <ContentCopy sx={{ fontSize: { xs: '1rem', sm: '1.25rem' } }} /> Clipboard Operations
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
                  startIcon={<ContentCopy sx={{ fontSize: '0.9rem' }} />}
                  onClick={writeClipboard}
                  disabled={loading}
                  sx={{ 
                    flex: 1, 
                    minWidth: { xs: 80, sm: 100 },
                    background: (theme) => darkMode ? 
                      `linear-gradient(45deg, ${theme.palette.primary.dark} 30%, ${theme.palette.primary.main} 90%)` :
                      `linear-gradient(45deg, ${theme.palette.primary.main} 30%, ${theme.palette.primary.light} 90%)`,
                    '&:hover': {
                      background: (theme) => darkMode ?
                        `linear-gradient(45deg, ${theme.palette.primary.main} 30%, ${theme.palette.primary.light} 90%)` :
                        `linear-gradient(45deg, ${theme.palette.primary.dark} 30%, ${theme.palette.primary.main} 90%)`,
                    },
                    fontSize: { xs: '0.75rem', sm: '0.85rem' },
                    py: 0.75,
                    height: { xs: 32, sm: 36 }
                  }}
                  title="Copy to clipboard (Ctrl+Shift+C)"
                >
                  Copy Text
                </Button>
                
                <Button
                  variant="outlined"
                  color="secondary"
                  startIcon={loading ? <CircularProgress size={16} /> : <ContentPaste sx={{ fontSize: '0.9rem' }} />}
                  onClick={readClipboard}
                  disabled={loading}
                  sx={{ 
                    flex: 1, 
                    minWidth: { xs: 80, sm: 100 },
                    borderColor: 'rgba(33, 150, 243, 0.5)',
                    fontSize: { xs: '0.75rem', sm: '0.85rem' },
                    py: 0.75,
                    height: { xs: 32, sm: 36 }
                  }}
                  title="Paste from clipboard (Ctrl+Shift+V)"
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
          py: { xs: 1, sm: 2 }, 
          px: 0, 
          mt: 'auto',
          bgcolor: 'background.paper',
          borderTop: '1px solid rgba(0, 0, 0, 0.12)',
          textAlign: 'center',
          borderRadius: 0
        }}
      >
        <Accordion sx={{ mb: 1 }}>
          <AccordionSummary
            expandIcon={<ExpandMoreIcon />}
            aria-controls="settings-content"
            id="settings-header"
          >
            <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
              <SettingsIcon />
              <Typography>Settings</Typography>
            </Box>
          </AccordionSummary>
          <AccordionDetails>
            <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
              <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                <Typography>Auto-delete old items</Typography>
                <Switch
                  checked={autoDeleteEnabled}
                  onChange={(e) => setAutoDeleteEnabled(e.target.checked)}
                  color="primary"
                />
              </Box>
              
              {autoDeleteEnabled && (
                <Box sx={{ display: 'flex', flexDirection: 'column', gap: 1 }}>
                  <Typography>Auto-delete after: {autoDeleteDays} days</Typography>
                  <Slider
                    value={autoDeleteDays}
                    onChange={(_, value) => setAutoDeleteDays(value as number)}
                    min={1}
                    max={30}
                    step={1}
                    valueLabelDisplay="auto"
                  />
                  <Button
                    variant="outlined"
                    onClick={cleanupOldItems}
                    sx={{ alignSelf: 'flex-start' }}
                  >
                    Clean Up Now
                  </Button>
                </Box>
              )}
              
              {/* Export/Import Section */}
              <Box sx={{ display: 'flex', flexDirection: 'column', gap: 1, pt: 2 }}>
                <Typography variant="h6">Export/Import History</Typography>
                <Box sx={{ display: 'flex', gap: 2 }}>
                  <Button
                    variant="outlined"
                    onClick={exportHistory}
                  >
                    Export History
                  </Button>
                  <Button
                    variant="outlined"
                    component="label"
                  >
                    Import History
                    <input
                      type="file"
                      hidden
                      accept=".json"
                      onChange={importHistory}
                    />
                  </Button>
                </Box>
              </Box>
            </Box>
          </AccordionDetails>
        </Accordion>
        
        <Typography variant="body2" color="text.secondary" sx={{ fontSize: { xs: '0.7rem', sm: '0.8rem' } }}>
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
              },
              fontSize: { xs: '0.7rem', sm: '0.8rem' }
            }}
          >
            sudhirkumar.in
          </Link>
        </Typography>
      </Box>

      {/* Image Modal */}
      <Modal
        open={!!selectedImage}
        onClose={() => setSelectedImage(null)}
        closeAfterTransition
        BackdropComponent={Backdrop}
        BackdropProps={{
          timeout: 500,
        }}
      >
        <Fade in={!!selectedImage}>
          <Box sx={{
            position: 'absolute',
            top: '50%',
            left: '50%',
            transform: 'translate(-50%, -50%)',
            maxWidth: '90vw',
            maxHeight: '90vh',
            outline: 'none',
          }}>
            {selectedImage && (
              <Box sx={{ 
                bgcolor: 'background.paper', 
                borderRadius: 2, 
                p: 2,
                boxShadow: 24,
                display: 'flex',
                flexDirection: 'column',
                alignItems: 'center',
              }}>
                <Typography variant="h6" sx={{ mb: 2 }}>
                  Image Preview ({selectedImage.dimensions})
                </Typography>
                <img 
                  src={`file://${selectedImage.filePath}`} 
                  alt="Full size preview" 
                  style={{ 
                    maxWidth: '80vw', 
                    maxHeight: '70vh',
                    objectFit: 'contain',
                    borderRadius: 4,
                  }} 
                />
                <Button 
                  onClick={() => setSelectedImage(null)} 
                  variant="contained" 
                  sx={{ mt: 2 }}
                >
                  Close
                </Button>
              </Box>
            )}
          </Box>
        </Fade>
      </Modal>
      
      {/* Statistics Modal */}
      <Modal
        open={showStatistics}
        onClose={() => setShowStatistics(false)}
        closeAfterTransition
        BackdropComponent={Backdrop}
        BackdropProps={{
          timeout: 500,
        }}
      >
        <Fade in={showStatistics}>
          <Box sx={{
            position: 'absolute',
            top: '50%',
            left: '50%',
            transform: 'translate(-50%, -50%)',
            width: '80vw',
            maxHeight: '80vh',
            outline: 'none',
          }}>
            <Card sx={{ 
              bgcolor: 'background.paper', 
              borderRadius: 2, 
              p: 3,
              boxShadow: 24,
              overflow: 'auto',
            }}>
              <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 3 }}>
                <Typography variant="h5" sx={{ fontWeight: 'bold', color: 'primary.main' }}>
                  <BarChartIcon sx={{ mr: 1, verticalAlign: 'middle' }} />
                  Clipboard Statistics
                </Typography>
                <IconButton onClick={() => setShowStatistics(false)}>
                  <CloseIcon />
                </IconButton>
              </Box>
              
              {statistics ? (
                <Box sx={{ display: 'grid', gridTemplateColumns: { xs: '1fr', md: '1fr 1fr' }, gap: 3 }}>
                  <Card sx={{ p: 2, bgcolor: 'primary.light' }}>
                    <Typography variant="h6" sx={{ mb: 2, fontWeight: 'bold' }}>
                      Overview
                    </Typography>
                    <Box sx={{ display: 'flex', flexDirection: 'column', gap: 1 }}>
                      <Typography>
                        <strong>Total Items:</strong> {statistics.totalItems}
                      </Typography>
                      <Typography>
                        <strong>Text Items:</strong> {statistics.textItems}
                      </Typography>
                      <Typography>
                        <strong>Image Items:</strong> {statistics.imageItems}
                      </Typography>
                      <Typography>
                        <strong>Favorites:</strong> {statistics.favoriteItems}
                      </Typography>
                    </Box>
                  </Card>
                  
                  <Card sx={{ p: 2, bgcolor: 'secondary.light' }}>
                    <Typography variant="h6" sx={{ mb: 2, fontWeight: 'bold' }}>
                      Top Tags
                    </Typography>
                    {statistics.topTags && statistics.topTags.length > 0 ? (
                      <Box sx={{ display: 'flex', flexDirection: 'column', gap: 1 }}>
                        {statistics.topTags.map(([tag, count]: [string, number], index: number) => (
                          <Box key={index} sx={{ display: 'flex', justifyContent: 'space-between' }}>
                            <Chip label={tag} size="small" sx={{ bgcolor: 'secondary.main', color: 'white' }} />
                            <Typography>{count} items</Typography>
                          </Box>
                        ))}
                      </Box>
                    ) : (
                      <Typography>No tags found</Typography>
                    )}
                  </Card>
                  
                  <Card sx={{ p: 2, bgcolor: 'success.light', gridColumn: '1 / -1' }}>
                    <Typography variant="h6" sx={{ mb: 2, fontWeight: 'bold' }}>
                      Activity Timeline
                    </Typography>
                    <Typography>
                      <strong>First Item:</strong> {statistics.earliestTimestamp ? new Date(statistics.earliestTimestamp * 1000).toLocaleDateString() : 'N/A'}
                    </Typography>
                    <Typography>
                      <strong>Latest Item:</strong> {statistics.latestTimestamp ? new Date(statistics.latestTimestamp * 1000).toLocaleDateString() : 'N/A'}
                    </Typography>
                  </Card>
                </Box>
              ) : (
                <Typography>Loading statistics...</Typography>
              )}
              
              <Box sx={{ display: 'flex', justifyContent: 'flex-end', mt: 3 }}>
                <Button 
                  onClick={() => setShowStatistics(false)} 
                  variant="contained" 
                  color="primary"
                >
                  Close
                </Button>
              </Box>
            </Card>
          </Box>
        </Fade>
      </Modal>
      
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
    </ThemeProvider>
  );
}

export default App;