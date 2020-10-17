// Copyright 2018 BEE project contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
const RCDIR= path.join(TOOLS_DIR, 'rc');

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
module.exports.RCDIR = RCDIR;
