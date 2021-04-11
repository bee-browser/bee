'use strict';

function codepoint(c) {
  return c.codePointAt(0);
}

class DotMatrixFont {
  constructor() {
    this.glyphs_ = DotMatrixFont.createGlyphs_();
    this.ROWS = 7;
    this.COLUMNS = 5;
  }

  getGlyphs(text) {
    return text.split('').map((c) => {
      const cp = codepoint(c);
      if (cp in this.glyphs_) {
        return this.glyphs_[cp];
      } else {
        return this.glyphs_[0xFFFD];
      }
    });
  }

  static createGlyphs_() {
    function _(strings) {
      return strings[0].split('').map((c) => {
        return c === '*' ? true : false;
      });
    }

    let glyphs = {};

    glyphs[codepoint('A')] = [
      _` *** `,
      _`*   *`,
      _`*   *`,
      _`*****`,
      _`*   *`,
      _`*   *`,
      _`*   *`,
    ];

    glyphs[codepoint('B')] = [
      _`**** `,
      _`*   *`,
      _`*   *`,
      _`**** `,
      _`*   *`,
      _`*   *`,
      _`**** `,
    ];

    glyphs[codepoint('C')] = [
      _` *** `,
      _`*   *`,
      _`*    `,
      _`*    `,
      _`*    `,
      _`*   *`,
      _` *** `,
    ];

    glyphs[codepoint('D')] = [
      _`***  `,
      _`*  * `,
      _`*   *`,
      _`*   *`,
      _`*   *`,
      _`*  * `,
      _`***  `,
    ];

    glyphs[codepoint('E')] = [
      _`*****`,
      _`*    `,
      _`*    `,
      _`**** `,
      _`*    `,
      _`*    `,
      _`*****`,
    ];

    glyphs[codepoint('F')] = [
      _`*****`,
      _`*    `,
      _`*    `,
      _`**** `,
      _`*    `,
      _`*    `,
      _`*    `,
    ];

    glyphs[codepoint('G')] = [
      _` *** `,
      _`*   *`,
      _`*    `,
      _`* ***`,
      _`*   *`,
      _`*   *`,
      _` *** `,
    ];

    glyphs[codepoint('H')] = [
      _`*   *`,
      _`*   *`,
      _`*   *`,
      _`*****`,
      _`*   *`,
      _`*   *`,
      _`*   *`,
    ];

    glyphs[codepoint('I')] = [
      _` *** `,
      _`  *  `,
      _`  *  `,
      _`  *  `,
      _`  *  `,
      _`  *  `,
      _` *** `,
    ];

    glyphs[codepoint('J')] = [
      _`  ***`,
      _`   * `,
      _`   * `,
      _`   * `,
      _`   * `,
      _`*  * `,
      _` **  `,
    ];

    glyphs[codepoint('K')] = [
      _`*   *`,
      _`*  * `,
      _`* *  `,
      _`**   `,
      _`* *  `,
      _`*  * `,
      _`*   *`,
    ];

    glyphs[codepoint('L')] = [
      _`*    `,
      _`*    `,
      _`*    `,
      _`*    `,
      _`*    `,
      _`*    `,
      _`*****`,
    ];

    glyphs[codepoint('M')] = [
      _`*   *`,
      _`*   *`,
      _`** **`,
      _`* * *`,
      _`*   *`,
      _`*   *`,
      _`*   *`,
    ];

    glyphs[codepoint('N')] = [
      _`*   *`,
      _`*   *`,
      _`**  *`,
      _`* * *`,
      _`*  **`,
      _`*   *`,
      _`*   *`,
    ];

    glyphs[codepoint('O')] = [
      _` *** `,
      _`*   *`,
      _`*   *`,
      _`*   *`,
      _`*   *`,
      _`*   *`,
      _` *** `,
    ];

    glyphs[codepoint('P')] = [
      _`**** `,
      _`*   *`,
      _`*   *`,
      _`**** `,
      _`*    `,
      _`*    `,
      _`*    `,
    ];

    glyphs[codepoint('Q')] = [
      _` *** `,
      _`*   *`,
      _`*   *`,
      _`*   *`,
      _`* * *`,
      _`*  * `,
      _` ** *`,
    ];

    glyphs[codepoint('R')] = [
      _`**** `,
      _`*   *`,
      _`*   *`,
      _`**** `,
      _`* *  `,
      _`*  * `,
      _`*   *`,
    ];

    glyphs[codepoint('S')] = [
      _` *** `,
      _`*   *`,
      _`*    `,
      _` *** `,
      _`    *`,
      _`*   *`,
      _` *** `,
    ];

    glyphs[codepoint('T')] = [
      _`*****`,
      _`  *  `,
      _`  *  `,
      _`  *  `,
      _`  *  `,
      _`  *  `,
      _`  *  `,
    ];

    glyphs[codepoint('U')] = [
      _`*   *`,
      _`*   *`,
      _`*   *`,
      _`*   *`,
      _`*   *`,
      _`*   *`,
      _` *** `,
    ];

    glyphs[codepoint('V')] = [
      _`*   *`,
      _`*   *`,
      _`*   *`,
      _`*   *`,
      _`*   *`,
      _` * * `,
      _`  *  `,
    ];

    glyphs[codepoint('W')] = [
      _`*   *`,
      _`*   *`,
      _`*   *`,
      _`* * *`,
      _`** **`,
      _`*   *`,
      _`*   *`,
    ];

    glyphs[codepoint('X')] = [
      _`*   *`,
      _`*   *`,
      _` * * `,
      _`  *  `,
      _` * * `,
      _`*   *`,
      _`*   *`,
    ];

    glyphs[codepoint('Y')] = [
      _`*   *`,
      _`*   *`,
      _` * * `,
      _`  *  `,
      _`  *  `,
      _`  *  `,
      _`  *  `,
    ];

    glyphs[codepoint('Z')] = [
      _`*****`,
      _`    *`,
      _`   * `,
      _`  *  `,
      _` *   `,
      _`*    `,
      _`*****`,
    ];

    glyphs[codepoint(',')] = [
      _`     `,
      _`     `,
      _`     `,
      _`     `,
      _` **  `,
      _`  *  `,
      _` *   `,
    ];

    glyphs[codepoint('.')] = [
      _`     `,
      _`     `,
      _`     `,
      _`     `,
      _`     `,
      _` **  `,
      _` **  `,
    ];

    glyphs[codepoint(' ')] = [
      _`     `,
      _`     `,
      _`     `,
      _`     `,
      _`     `,
      _`     `,
      _`     `,
    ];

    // REPLACEMENT CHARACTER
    glyphs[0x00FFFD] = [
      _`*****`,
      _`*   *`,
      _`*   *`,
      _`*   *`,
      _`*   *`,
      _`*   *`,
      _`*****`,
    ];

    return glyphs;
  }
}

export const DOT_MATRIX_FONT = new DotMatrixFont();
