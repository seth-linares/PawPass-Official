import { memo } from 'react'
import { motion } from 'framer-motion'
import { Minus, Square, X, Minimize2 } from 'lucide-react'

interface WindowControlsProps {
  isMaximized: boolean;
  onMinimize: () => void;
  onMaximize: () => void;
  onClose: () => void;
}

export const WindowControls = memo(function WindowControls({ 
  isMaximized, 
  onMinimize, 
  onMaximize, 
  onClose 
}: WindowControlsProps) {
  return (
    <div className="flex">
      <motion.button
        whileHover={{ backgroundColor: "var(--btn-hover)" }}
        whileTap={{ scale: 0.95 }}
        onClick={onMinimize}
        className="btn btn-ghost btn-sm h-8 w-12 rounded-none"
        aria-label="Minimize"
      >
        <Minus className="h-4 w-4" />
      </motion.button>
      <motion.button
        whileHover={{ backgroundColor: "var(--btn-hover)" }}
        whileTap={{ scale: 0.95 }}
        onClick={onMaximize}
        className="btn btn-ghost btn-sm h-8 w-12 rounded-none"
        aria-label="Maximize"
      >
        {isMaximized ? (
          <Minimize2 className="h-4 w-4" />
        ) : (
          <Square className="h-4 w-4" />
        )}
      </motion.button>
      <motion.button
        whileHover={{ backgroundColor: "var(--btn-hover)" }}
        whileTap={{ scale: 0.95 }}
        onClick={onClose}
        className="btn btn-ghost btn-sm h-8 w-12 rounded-none"
        aria-label="Close"
      >
        <X className="h-6 w-6 text-red-700" />
      </motion.button>
    </div>
  )
})
