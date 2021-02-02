// At this point, it's not suitable for bee-tools to use the --import-map option.
// This option does not work well with shebangs.

// TODO: automate upgrades of packages in this file.

// std

export * as path from 'https://deno.land/std@0.85.0/path/mod.ts';
export * as testing from 'https://deno.land/std@0.85.0/testing/asserts.ts';

// third party

export * as changeCase from 'https://deno.land/x/case@v2.1.0/mod.ts';
export { default as docopt } from 'https://deno.land/x/docopt@v1.0.6/mod.ts';
//export { default as puppeteer } from 'https://deno.land/x/puppeteer@5.5.1/mod.ts';
export { default as puppeteer } from 'https://raw.githubusercontent.com/masnagam/deno-puppeteer/main/mod.ts';
