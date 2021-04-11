'use strict';

import { path } from '../deps.js';

const DIRNAME = path.dirname(path.fromFileUrl(import.meta.url));

export const PROJ_DIR = path.resolve(DIRNAME, '..', '..');
export const TOOLS_DIR = path.join(PROJ_DIR, 'tools');
export const RESOURCES_DIR = path.join(TOOLS_DIR, 'resources');
