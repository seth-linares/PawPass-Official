import { motion, AnimatePresence } from "framer-motion";

const ThemePreview = () => {
  const themeColors = [
    { name: "Primary", class: "bg-primary text-primary-foreground" },
    { name: "Secondary", class: "bg-secondary text-secondary-foreground" },
    { name: "Accent", class: "bg-accent text-accent-content" },
    { name: "Neutral", class: "bg-neutral text-neutral-content" },
    { name: "Info", class: "bg-info text-info-content" },
    { name: "Success", class: "bg-success text-success-content" },
    { name: "Warning", class: "bg-warning text-warning-content" },
    { name: "Error", class: "bg-error text-error-content" },
  ];

  return (
    <AnimatePresence>
      <motion.div
        initial={{ opacity: 0, y: -20 }}
        animate={{ opacity: 1, y: 0 }}
        exit={{ opacity: 0, y: -20 }}
        transition={{ 
          duration: 0.3,
          ease: "easeOut"
        }}
        className="grid gap-4"
      >
        {/* Colors Preview */}
        <motion.div 
          className="grid grid-cols-2 md:grid-cols-4 gap-4"
          variants={{
            hidden: { opacity: 0 },
            show: {
              opacity: 1,
              transition: {
                staggerChildren: 0.05,
                when: "beforeChildren"
              }
            }
          }}
          initial="hidden"
          animate="show"
        >
          {themeColors.map(({ name, class: className }) => (
            <motion.div
              key={name}
              variants={{
                hidden: { opacity: 0, y: 10 },
                show: { opacity: 1, y: 0 }
              }}
              transition={{ duration: 0.2 }}
              className={`p-4 rounded-lg ${className} flex items-center justify-center text-center min-h-[80px] shadow-lg hover:scale-105 transition-transform`}
            >
              <span className="font-medium">{name}</span>
            </motion.div>
          ))}
        </motion.div>

        {/* Components Preview */}
        <div className="grid gap-4">
          <div className="card bg-base-200 shadow-xl">
            <div className="card-body">
              <h3 className="card-title text-base-content">Interactive Elements</h3>
              <div className="flex flex-wrap gap-2">
                <button className="btn btn-primary">Primary</button>
                <button className="btn btn-secondary">Secondary</button>
                <button className="btn btn-accent">Accent</button>
                <button className="btn btn-outline">Outline</button>
              </div>
            </div>
          </div>

          <div className="card bg-base-200 shadow-xl">
            <div className="card-body">
              <h3 className="card-title text-base-content">Form Elements</h3>
              <div className="flex flex-col gap-2">
                <input type="text" placeholder="Text input" className="input input-bordered input-primary w-full placeholder:text-base-content/50" />
                <div className="flex gap-2">
                  <input type="checkbox" className="checkbox checkbox-primary" />
                  <input type="radio" className="radio radio-primary" />
                </div>
              </div>
            </div>
          </div>

          <div className="card bg-base-200 shadow-xl">
            <div className="card-body">
              <h3 className="card-title text-base-content">Status Indicators</h3>
              <div className="flex flex-wrap gap-2">
                <div className="badge badge-primary">Primary</div>
                <div className="badge badge-secondary">Secondary</div>
                <div className="badge badge-accent">Accent</div>
                <div className="badge badge-outline">Outline</div>
              </div>
            </div>
          </div>

          <div className="grid grid-cols-2 gap-2">
            <div className="alert alert-info text-info-content">
              <span>Info message</span>
            </div>
            <div className="alert alert-success text-success-content">
              <span>Success message</span>
            </div>
          </div>
        </div>
      </motion.div>
    </AnimatePresence>
  );
};

export default ThemePreview;
