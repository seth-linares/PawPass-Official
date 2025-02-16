import { useTitleBar } from '@/hooks/useTitleBar'
import { motion } from 'framer-motion'
import { AnimatedTitle } from './ui/AnimatedTitle'
import { WindowControls } from './ui/WindowControls'

export default function TitleBar() {
  const { isMaximized, error, handleMinimize, handleMaximize, handleClose } = useTitleBar();

  if (error) {
    return (
      <div className="h-8 flex justify-between items-center bg-error text-error-content px-4">
        <div>Error: {error}</div>
      </div>
    );
  }

  return (
    <motion.div
      data-tauri-drag-region 
      className="fixed top-0 left-0 right-0 z-50 h-8 flex justify-between items-center bg-base-200 border-b border-base-300"
      initial={{ opacity: 0, y: -10 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.3 }}
    >
      <div className="px-4">
        <AnimatedTitle />
      </div>
      <WindowControls
        isMaximized={isMaximized}
        onMinimize={handleMinimize}
        onMaximize={handleMaximize}
        onClose={handleClose}
      />
    </motion.div>
  )
}