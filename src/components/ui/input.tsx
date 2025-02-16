import * as React from "react"
import { cn } from "@/lib/utils"

const Input = React.forwardRef<HTMLInputElement, React.InputHTMLAttributes<HTMLInputElement>>(
  ({ className, type, ...props }, ref) => {
    return (
      <input
        type={type}
        className={cn(
          "input input-bordered w-full bg-base-100 text-base placeholder:text-base-content/60 focus:input-primary disabled:cursor-not-allowed disabled:opacity-50 md:text-sm [&::-ms-reveal]:hidden [&::-webkit-credentials-auto-fill-button]:hidden",
          className
        )}
        ref={ref}
        autoComplete="off"
        {...props}
      />
    )
  }
)
Input.displayName = "Input"

export { Input }
