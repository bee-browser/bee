// At this point, it's not suitable for bee-tools to use the --import-map option.
// This option does not work well with shebangs.

// std

export * as fs from 'https://deno.land/std@0.92.0/fs/mod.ts';
export * as http from 'https://deno.land/std@0.92.0/http/mod.ts';
export * as http_file_server from 'https://deno.land/std@0.92.0/http/file_server.ts';
export * as path from 'https://deno.land/std@0.92.0/path/mod.ts';
export * as testing from 'https://deno.land/std@0.92.0/testing/asserts.ts';
export * as ws from 'https://deno.land/std@0.92.0/ws/mod.ts';

// third party

export * as changeCase from 'https://deno.land/x/case@v2.1.0/mod.ts';
export { default as docopt } from 'https://deno.land/x/docopt@v1.0.6/mod.ts';
//export { default as puppeteer } from 'https://deno.land/x/puppeteer@5.5.1/mod.ts';
export { default as puppeteer } from 'https://raw.githubusercontent.com/masnagam/deno-puppeteer/main/mod.ts';
export * as oak from 'https://deno.land/x/oak@v6.5.0/mod.ts';
export { default as oak_logger } from 'https://deno.land/x/oak_logger@1.0.0/mod.ts';
export * as handlebars from 'https://deno.land/x/handlebars@v0.6.0/mod.ts';
export * as base64 from 'https://denopkg.com/chiefbiiko/base64@v0.2.1/mod.ts';
