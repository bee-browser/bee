'use strict';

import resolve from 'rollup-plugin-node-resolve';
import commonjs from 'rollup-plugin-commonjs';

export default {
  output: {
    format: 'iife',
    sourcemap: true,
  },
  plugins: [
    resolve({ preferBuiltins: false }),
    commonjs()
  ]
};
