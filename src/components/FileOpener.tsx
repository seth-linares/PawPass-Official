// src/components/FileOpener.tsx

import { open } from "@tauri-apps/plugin-dialog";

// Props interface defines what the parent component needs to provide
// onFileSelect: callback function that receives the selected file path
interface FileOpenerProps {
  onFileSelect: (path: string) => void;
}

function FileOpener({ onFileSelect }: FileOpenerProps) {
  // Handler for file selection
  // Uses Tauri's dialog API to show native OS file picker
  const handleOpenFile = async () => {
    console.log("📂 FileOpener: Starting file selection...");
    try {
      console.log("📂 FileOpener: Showing file dialog...");
      // The open() function returns a Promise that resolves to:
      // - a string (single file path) when multiple: false
      // - an array of strings (multiple file paths) when multiple: true
      // - null if user cancels the dialog
      const selected = await open({
        multiple: false,  // Set to true to allow multiple file selection
        filters: [{
          name: "Backup Files",
          extensions: ["backup", "*"]
        }]
        // Other useful options include:
        // - directory: true // Pick directories instead of files
        // - defaultPath: "/some/path" // Set initial directory
      });

      console.log("📂 FileOpener: Dialog result:", selected);

      // Type guard: check if we have a valid selection
      // selected will be null if user cancels
      // Array.isArray check is needed because multiple: true returns string[]
      if (selected && !Array.isArray(selected)) {
        console.log("📂 FileOpener: File selected:", selected);
        // At this point, selected is guaranteed to be a single file path string
        // The path is OS-specific and absolute, e.g.:
        // Windows: C:\\Users\\username\\file.txt
        // macOS/Linux: /home/username/file.txt
        onFileSelect(selected);
      } else {
        console.log("📂 FileOpener: No file selected or selection cancelled");
      }
    } catch (err) {
      // Handle any errors that might occur during file selection
      // Common errors: permissions, file system issues
      console.error("❌ FileOpener: Error during file selection:", err);
    }
  };

  return (
    <button 
      onClick={handleOpenFile} 
      className="btn btn-info"
      type="button"
    >
      Select Backup
    </button>
  );
}

export default FileOpener;