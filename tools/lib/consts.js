'use strict';

const path = require('path');

const PROJ_DIR = path.resolve(__dirname, '..', '..');
const OUTPUT_DIR = path.join(PROJ_DIR, 'output');
const OUTPUT_BIN_DIR = path.join(OUTPUT_DIR, 'bin');
const OUTPUT_LIB_DIR = path.join(OUTPUT_DIR, 'lib');
const OUTPUT_STUDIES_DIR = path.join(OUTPUT_DIR, 'studies');
const OUTPUT_STUDIES_LAYOUT_DIR = path.join(OUTPUT_STUDIES_DIR, 'layout');
const STUDIES_DIR = path.join(PROJ_DIR, 'studies');
const STUDIES_LAYOUT_DIR = path.join(STUDIES_DIR, 'layout');
const TESTS_DIR = path.join(PROJ_DIR, 'tests');
const TESTS_LAYOUT_DIR = path.join(TESTS_DIR, 'layout');
const TOOLS_DIR = path.join(PROJ_DIR, 'tools');
const BINDIR = path.join(TOOLS_DIR, 'bin');
const RCDIR= path.join(TOOLS_DIR, 'rc');
const TARGET_DIR = path.join(PROJ_DIR, 'target');
const DEBUG_BUILD_DIR = path.join(TARGET_DIR, 'debug');
const RELEASE_BUILD_DIR = path.join(TARGET_DIR, 'release');
const WEBUI_DIR = path.join(PROJ_DIR, 'webui');
const WEBUI_ASSETS_DIR = path.join(WEBUI_DIR, 'assets');

// exports

module.exports.PROJ_DIR = PROJ_DIR;
module.exports.OUTPUT_DIR = OUTPUT_DIR
module.exports.OUTPUT_BIN_DIR = OUTPUT_BIN_DIR
module.exports.OUTPUT_LIB_DIR = OUTPUT_LIB_DIR
module.exports.OUTPUT_STUDIES_DIR = OUTPUT_STUDIES_DIR
module.exports.OUTPUT_STUDIES_LAYOUT_DIR = OUTPUT_STUDIES_LAYOUT_DIR
module.exports.STUDIES_DIR = STUDIES_DIR
module.exports.STUDIES_LAYOUT_DIR = STUDIES_LAYOUT_DIR
module.exports.TESTS_DIR = TESTS_DIR
module.exports.TESTS_LAYOUT_DIR = TESTS_LAYOUT_DIR
module.exports.TOOLS_DIR = TOOLS_DIR;
module.exports.BINDIR = BINDIR;
module.exports.RCDIR = RCDIR;
module.exports.DEBUG_BUILD_DIR = DEBUG_BUILD_DIR;
module.exports.RELEASE_BUILD_DIR = RELEASE_BUILD_DIR;
module.exports.WEBUI_ASSETS_DIR = WEBUI_ASSETS_DIR;
