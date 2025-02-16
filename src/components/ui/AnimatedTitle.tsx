import { motion } from 'framer-motion'
import { useState, memo } from 'react'

const bounceTransition = {
  y: {
    duration: 0.4,
    repeat: Infinity,
    repeatType: "reverse",
    ease: "easeOut"
  },
  rotate: {
    duration: 0.4,
    repeat: Infinity,
    repeatType: "reverse",
    ease: "easeOut"
  }
} as const;

export const AnimatedTitle = memo(function AnimatedTitle() {
  const [isHappy, setIsHappy] = useState(false)
  const letters = "PawPass".split("")
  const catEmoji = isHappy ? "😺" : "😸"

  const getLetterColor = (index: number) => {
    return index < 3 ? 'text-success' : 'text-error'
  }

  return (
    <div className="flex items-center space-x-1">
      <motion.div
        className="cursor-pointer select-none"
        animate={{
          y: [0, -3, 0],
          rotate: [0, -5, 5, 0],
        }}
        transition={bounceTransition}
        whileHover={{ scale: 1.2 }}
        onHoverStart={() => setIsHappy(true)}
        onHoverEnd={() => setIsHappy(false)}
      >
        {catEmoji}
        {"🐾"}
      </motion.div>
      {letters.map((letter, index) => (
        <motion.span
          key={index}
          className={`font-bold ${getLetterColor(index)}`}
          initial={{ opacity: 0, y: -20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{
            duration: 0.5,
            delay: index * 0.1,
            type: "spring",
            stiffness: 200
          }}
          whileHover={{
            scale: 1.2,
            rotate: [0, -5, 5, 0],
            transition: { duration: 0.3 }
          }}
        >
          {letter}
        </motion.span>
      ))}
      <motion.div
        className="w-1 h-4 bg-info rounded-full"
        animate={{
          opacity: [1, 0.5],
          height: ["16px", "8px"],
          transition: {
            duration: 0.8,
            repeat: Infinity,
            repeatType: "reverse",
            ease: "easeInOut"
          }
        }}
      />
    </div>
  )
})
