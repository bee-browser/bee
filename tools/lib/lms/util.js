'use strict';

const colors = require('color-name');

class Color {
  static parse(str) {
    if (typeof str !== 'string') {
      // Error.message will be ignored in Command._validateArgs.
      throw new Error('Invalid format.');
    }
    if (colors[str] !== undefined) {
      const c = colors[str];
      return [c[0], c[1], c[2], 255];
    }
    if (str.match(/^rgb\((\d+),(\d+),(\d+)\)/)) {
      const r = parseInt(RegExp.$1);
      const g = parseInt(RegExp.$2);
      const b = parseInt(RegExt.$3);
      return [r, g, b, 255];
    }
    if (str.match(/^rgba\((\d+),(\d+),(\d+),(\d+)\)/)) {
      const r = parseInt(RegExp.$1);
      const g = parseInt(RegExp.$2);
      const b = parseInt(RegExt.$3);
      const a = parseInt(RegExt.$4);
      return [r, g, b, a];
    }
    // Error.message will be ignored in Command._validateArgs.
    throw new Error(`Invalid format.`);
  }
}

class Size {
  constructor(width, height) {
    this.width = width;
    this.height = height;
  }

  toString() {
    return `${this.width}x${this.height}`;
  }

  static parse(str) {
    if (typeof str !== 'string') {
      // Error.message will be ignored in Command._validateArgs.
      throw new Error('Invalid format.');
    }
    if (str.match(/^(\d+)x(\d+)$/)) {
      const width = parseInt(RegExp.$1);
      const height = parseInt(RegExp.$2);
      return new Size(width, height);
    }
    // Error.message will be ignored in Command._validateArgs.
    throw new Error('Invalid format.');
  }
}

module.exports.Color = Color;
module.exports.Size = Size;
