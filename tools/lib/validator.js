'use strict';

const fs = require('fs');

function fileExists(file) {
  if (!fs.existsSync(file)) {
    // Error.message will be ignored in Command._validateArgs.
    throw new Error(`${file} does not exist.`);
  }
  return file;
}

// exports

module.exports.fileExists = fileExists;
