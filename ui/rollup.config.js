import { createDefaultConfig } from "@open-wc/building-rollup";
import commonjs from "@rollup/plugin-commonjs";
import json from "@rollup/plugin-json";
import replace from "@rollup/plugin-replace";
import resolve from "@rollup/plugin-node-resolve";
import builtins from "rollup-plugin-node-builtins";
import globals from "rollup-plugin-node-globals";

// if you need to support IE11 use "modern-and-legacy-config" instead.
// import { createCompatibilityConfig } from '@open-wc/building-rollup';
// export default createCompatibilityConfig({ input: './index.html' });
const config = createDefaultConfig({ input: "./index.html" });

export default {
  ...config,
  plugins: [
    ...config.plugins,
    globals(),
    replace({ "process.env.NODE_ENV": "'development'" }),
    resolve({ browser: true, preferBuiltins: true }),
    commonjs({
      namedExports: {
        "graphql-tools": ["makeExecutableSchema"]
      }
    }),
    builtins(),
    json()
  ]
};
