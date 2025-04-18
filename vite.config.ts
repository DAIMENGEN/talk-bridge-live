// @ts-ignore

import {defineConfig} from "vite";
import react from "@vitejs/plugin-react";

// // @ts-expect-error process is a Node.js global
// const host = process.env.TAURI_DEV_HOST;
const host = "0.0.0.0";

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [react()],
    // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
    // 1. prevent vite from obscuring rust errors
    clearScreen: false,
    // 2. tauri expects a fixed port, fail if that port is not available
    server: {
        port: 1420,
        strictPort: true,
        host: host || false,
        hmr: host
            ? {
                protocol: "ws",
                host,
                port: 1421,
            }
            : undefined,
        watch: {
            // 3. tell vite to ignore watching `src-tauri`
            ignored: ["**/src-tauri/**"],
        },
        headers: {
            "Cross-Origin-Opener-Policy": "same-origin",
            "Cross-Origin-Embedder-Policy": "require-corp"
        }
    },
    worker: {
        format: "es",
    },
    resolve: {
        alias: {
            "@src": "/src",
        },
    },
    optimizeDeps: {
        exclude: ["src-wasm"],
    },
});
