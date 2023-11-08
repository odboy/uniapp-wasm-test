import { defineConfig } from "vite";
import uni from "@dcloudio/vite-plugin-uni";
import wasmPack from 'vite-plugin-wasm-pack';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [uni(),wasmPack('./wasm')],
});
