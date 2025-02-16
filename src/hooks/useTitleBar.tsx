import { Window } from '@tauri-apps/api/window'
import { useState, useEffect } from 'react'

export function useTitleBar() {
  const [isMaximized, setIsMaximized] = useState(false);
  const [window, setWindow] = useState<Window>();
  const [error, setError] = useState<string>();


  useEffect(() => {
    async function initWindow() {
      try {
        console.log('🪟 Initializing window...');
        const win = Window.getCurrent();
        console.log('🪟 Window instance:', { win, label: win.label });
        setWindow(win);

        const maximized = await win.isMaximized();
        console.log('🪟 Is maximized:', maximized);
        setIsMaximized(maximized);
      } catch (err) {
        const errorMessage = err instanceof Error ? err.message : 'Unknown error';
        console.error('🚨 Window initialization error:', errorMessage, err);
        setError(errorMessage);
      }
    }

    initWindow();

    return () => {
      console.log('🧹 TitleBar cleanup');
    };
  }, []);

  const handleMinimize = async () => {
    try {
      console.log('🪟 Minimizing window...');
      await window?.minimize();
    } catch (err) {
      console.error('🚨 Minimize error:', err);
    }
  };

  const handleMaximize = async () => {
    if (!window) {
      console.error('🚨 No window instance available');
      return;
    }
    
    try {
      console.log('🪟 Toggling maximize state...');
      if (isMaximized) {
        await window.unmaximize();
        setIsMaximized(false);
      } else {
        await window.maximize();
        setIsMaximized(true);
      }
    } catch (err) {
      console.error('🚨 Maximize error:', err);
    }
  };

  const handleClose = async () => {
    try {
      console.log('🪟 Closing window...');
      await window?.close();
    } catch (err) {
      console.error('🚨 Close error:', err);
    }
  };

  return {
    isMaximized,
    error,
    handleMinimize,
    handleMaximize,
    handleClose
  }
}
