import { debug, info, warn, error } from "@tauri-apps/plugin-log";
export const LOG_PREFIX = "[TALK-BRIDGE-LIVE-FRONTEND]";

// Define the log levels as a tuple, ensuring type safety
// Severity: error > warn > info > debug > trace
const levels = ["debug", "info", "warn", "error"] as const;
type Level = typeof levels[number]; // Type for allowed log levels
type LogFn = (...args: unknown[]) => void; // Type for a log function
type Logger = {
    [K in Level]: LogFn; // Logger object type mapping levels to log functions
};

// Create a mapping for the Tauri logger
const tauriLogMap: Record<Level, (message: string) => Promise<void>> = {
    error,
    warn,
    info,
    debug,
};

// Factory function to create a log function for a specific level
function createLogFunction(level: Level): LogFn {
    return (...args: unknown[]) => {
        // 1️⃣ Normal frontend console output with prefix
        console[level](LOG_PREFIX, ...args); // Use the console method corresponding to the log level
        // 2️⃣ Concatenate the string and send it to the Tauri logging system
        const message = `${LOG_PREFIX} ${args.map(arg => String(arg)).join(' ')}`;
        tauriLogMap[level](message).catch(err => {
            // Prevent tauri log exceptions from affecting frontend debugging
            console.error("[LOG-FORWARD-FAILED]", err);
        });
    };
}

// Create the logger object by reducing over the levels array
export const log: Logger = levels.reduce((acc, level) => {
    acc[level] = createLogFunction(level); // Assign a log function for each level
    return acc;
}, {} as Logger); // Assert the initial value as Logger type
