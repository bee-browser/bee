'use strict';

import * as path from 'https://deno.land/std@0.220.1/path/mod.ts';

const DIRNAME = path.dirname(path.fromFileUrl(import.meta.url));

export const PROJ_DIR = path.resolve(DIRNAME, '..', '..');
export const TOOLS_DIR = path.join(PROJ_DIR, 'tools');
export const VENDOR_DIR = path.join(PROJ_DIR, 'vendor');
export const RESOURCES_DIR = path.join(TOOLS_DIR, 'resources');
export const WORKERS_DIR = path.join(RESOURCES_DIR, 'workers');
