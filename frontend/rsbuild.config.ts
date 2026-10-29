import { loadEnv, mergeRsbuildConfig, RsbuildConfig } from '@rsbuild/core';
import { pluginReact } from '@rsbuild/plugin-react';
import { pluginTailwindcss } from '@rsbuild/plugin-tailwindcss';

const config: RsbuildConfig = {
  server: {
    port: 3000,
  },
  html: {
    title: 'Zero2Prod Rust + React',
  },
  plugins: [
    pluginReact({
      reactCompiler: true,
    }),
    pluginTailwindcss(),
  ],
};

const env = loadEnv();

// Docs: https://rsbuild.rs/config/
export default mergeRsbuildConfig(config, {
  source: {
    define: env.publicVars,
  },
});
