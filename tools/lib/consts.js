'use strict';

import * as path from 'std/path/mod.ts';

const DIRNAME = path.dirname(path.fromFileUrl(import.meta.url));

export const PROJ_DIR = path.resolve(DIRNAME, '..', '..');
export const TOOLS_DIR = path.join(PROJ_DIR, 'tools');
export const RESOURCES_DIR = path.join(TOOLS_DIR, 'resources');
export const WORKERS_DIR = path.join(RESOURCES_DIR, 'workers');
