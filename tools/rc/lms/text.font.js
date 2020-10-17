'use strict';

function codepoint(c) {
  return c.codePointAt(0);
}

module.exports[codepoint('A')] = [
  " *** ",
  "*   *",
  "*   *",
  "*****",
  "*   *",
  "*   *",
  "*   *"
];

module.exports[codepoint('B')] = [
  "**** ",
  "*   *",
  "*   *",
  "**** ",
  "*   *",
  "*   *",
  "**** "
];

module.exports[codepoint('C')] = [
  " *** ",
  "*   *",
  "*    ",
  "*    ",
  "*    ",
  "*   *",
  " *** "
];

module.exports[codepoint('D')] = [
  "***  ",
  "*  * ",
  "*   *",
  "*   *",
  "*   *",
  "*  * ",
  "***  "
];

module.exports[codepoint('E')] = [
  "*****",
  "*    ",
  "*    ",
  "**** ",
  "*    ",
  "*    ",
  "*****"
];

module.exports[codepoint('F')] = [
  "*****",
  "*    ",
  "*    ",
  "**** ",
  "*    ",
  "*    ",
  "*    "
];

module.exports[codepoint('G')] = [
  " *** ",
  "*   *",
  "*    ",
  "* ***",
  "*   *",
  "*   *",
  " *** "
];

module.exports[codepoint('H')] = [
  "*   *",
  "*   *",
  "*   *",
  "*****",
  "*   *",
  "*   *",
  "*   *"
];

module.exports[codepoint('I')] = [
  " *** ",
  "  *  ",
  "  *  ",
  "  *  ",
  "  *  ",
  "  *  ",
  " *** "
];

module.exports[codepoint('J')] = [
  "  ***",
  "   * ",
  "   * ",
  "   * ",
  "   * ",
  "*  * ",
  " **  "
];

module.exports[codepoint('K')] = [
  "*   *",
  "*  * ",
  "* *  ",
  "**   ",
  "* *  ",
  "*  * ",
  "*   *"
];

module.exports[codepoint('L')] = [
  "*    ",
  "*    ",
  "*    ",
  "*    ",
  "*    ",
  "*    ",
  "*****"
];

module.exports[codepoint('M')] = [
  "*   *",
  "*   *",
  "** **",
  "* * *",
  "*   *",
  "*   *",
  "*   *"
];

module.exports[codepoint('N')] = [
  "*   *",
  "*   *",
  "**  *",
  "* * *",
  "*  **",
  "*   *",
  "*   *"
];

module.exports[codepoint('O')] = [
  " *** ",
  "*   *",
  "*   *",
  "*   *",
  "*   *",
  "*   *",
  " *** "
];

module.exports[codepoint('P')] = [
  "**** ",
  "*   *",
  "*   *",
  "**** ",
  "*    ",
  "*    ",
  "*    "
];

module.exports[codepoint('Q')] = [
  " *** ",
  "*   *",
  "*   *",
  "*   *",
  "* * *",
  "*  * ",
  " ** *"
];

module.exports[codepoint('R')] = [
  "**** ",
  "*   *",
  "*   *",
  "**** ",
  "* *  ",
  "*  * ",
  "*   *"
];

module.exports[codepoint('S')] = [
  " *** ",
  "*   *",
  "*    ",
  " *** ",
  "    *",
  "*   *",
  " *** "
];

module.exports[codepoint('T')] = [
  "*****",
  "  *  ",
  "  *  ",
  "  *  ",
  "  *  ",
  "  *  ",
  "  *  "
];

module.exports[codepoint('U')] = [
  "*   *",
  "*   *",
  "*   *",
  "*   *",
  "*   *",
  "*   *",
  " *** "
];

module.exports[codepoint('V')] = [
  "*   *",
  "*   *",
  "*   *",
  "*   *",
  "*   *",
  " * * ",
  "  *  "
];

module.exports[codepoint('W')] = [
  "*   *",
  "*   *",
  "*   *",
  "* * *",
  "** **",
  "*   *",
  "*   *"
];

module.exports[codepoint('X')] = [
  "*   *",
  "*   *",
  " * * ",
  "  *  ",
  " * * ",
  "*   *",
  "*   *"
];

module.exports[codepoint('Y')] = [
  "*   *",
  "*   *",
  " * * ",
  "  *  ",
  "  *  ",
  "  *  ",
  "  *  "
];

module.exports[codepoint('Z')] = [
  "*****",
  "    *",
  "   * ",
  "  *  ",
  " *   ",
  "*    ",
  "*****"
];

module.exports[codepoint(',')] = [
  "     ",
  "     ",
  "     ",
  "     ",
  " **  ",
  "  *  ",
  " *   "
];

module.exports[codepoint('.')] = [
  "     ",
  "     ",
  "     ",
  "     ",
  "     ",
  " **  ",
  " **  "
];

module.exports[codepoint(' ')] = [
  "     ",
  "     ",
  "     ",
  "     ",
  "     ",
  "     ",
  "     "
];

// REPLACEMENT CHARACTER
module.exports[0x00FFFD] = [
  "*****",
  "*   *",
  "*   *",
  "*   *",
  "*   *",
  "*   *",
  "*****"
];
