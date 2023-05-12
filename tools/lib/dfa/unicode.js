import {
  assert,
  unreachable,
} from 'https://deno.land/std@0.186.0/testing/asserts.ts';

const CODE_POINT_MIN = 0;
const CODE_POINT_MAX = 0x10FFFF;
const CODE_POINT_END = 0x110000;

function ToCodePoint(value) {
  if (typeof value === 'string') {
    return value.codePointAt(0);
  }
  assert(Number.isInteger(value));
  assert(value >= CODE_POINT_MIN);
  assert(value <= CODE_POINT_MAX);
  return value;
}

function toDisplayChar(cp) {
  const asciiTable = [
    '<NUL>', '<SOH>', '<STX>', '<ETX>', '<EOT>', '<ENQ>', '<ACK>', '<BEL>',
    '<BS>', '<HT>', '<LF>', '<VT>', '<FF>', '<CR>', '<SO>', '<SI>',
    '<DLE>', '<DC1>', '<DC2>', '<DC3>', '<DC4>', '<NAK>', '<SYN>', '<ETB>',
    '<CAN>', '<EM>', '<SUB>', '<ESC>', '<FS>', '<GS>', '<RS>', '<US>',
    '<SP>',
  ];
  if (cp <= 0x20) {
    return asciiTable[cp];
  }
  if (cp < 0x7F) {
    return String.fromCodePoint(cp);
  }
  if (cp === 0x7F) {
    return '<DEL>';
  }
  if (cp < 0x10000) {
    return `U+${cp.toString(16).toUpperCase().padStart(4, '0')}`;
  }
  return `U+${cp.toString(16).toUpperCase().padStart(6, '0')}`;
}

export class UnicodeSpan {
  static EMPTY = new UnicodeSpan();
  static ANY = new UnicodeSpan(CODE_POINT_MIN, CODE_POINT_MAX);

  constructor(first, last) {
    if (first === undefined) {
      this.base_ = 0;
      this.length_ = 0;
    } else if (last === undefined) {
      this.base_ = ToCodePoint(first);
      this.length_ = 1;
    } else {
      this.base_ = ToCodePoint(first);
      const lastCodePoint = ToCodePoint(last);
      this.length_ = lastCodePoint - this.base_ + 1;
    }
    Object.freeze(this);  // immutable
  }

  get length() {
    return this.length_;
  }

  get isEmpty() {
    return this.length_ <= 0;
  }

  get firstCodePoint() {
    assert(!this.isEmpty);
    return this.base_;
  }

  get firstDisplayChar() {
    return toDisplayChar(this.firstCodePoint);
  }

  get lastCodePoint() {
    assert(!this.isEmpty);
    return this.base_ + this.length_ - 1;
  }

  get lastDisplayChar() {
    return toDisplayChar(this.lastCodePoint);
  }

  get prevCodePoint() {
    assert(!this.isEmpty);
    assert(this.base_ > CODE_POINT_MIN);
    return this.base_ - 1;
  }

  get nextCodePoint() {
    assert(!this.isEmpty);
    assert(this.base_ + this.length_ < CODE_POINT_END);
    return this.base_ + this.length_;
  }

  equals(other) {
    assert(other instanceof UnicodeSpan);
    if (this === other) {
      return true;
    }
    if (this.isEmpty && other.isEmpty) {
      return true;
    }
    if (this.isEmpty || other.isEmpty) {
      return false;
    }
    if (this.firstCodePoint !== other.firstCodePoint) {
      return false;
    }
    if (this.lastCodePoint !== other.lastCodePoint) {
      return false;
    }
    return true;
  }

  has(cp) {
    cp = ToCodePoint(cp);
    if (this.isEmpty) {
      return false;
    }
    if (this.firstCodePoint > cp) {
      return false;
    }
    if (this.lastCodePoint < cp) {
      return false;
    }
    return true;
  }

  includes(other) {
    assert(other instanceof UnicodeSpan);
    if (other.isEmpty) {
      return false;
    }
    return this.has(other.firstCodePoint) && this.has(other.lastCodePoint);
  }

  expand(n) {
    if (this.isEmpty) {
      return this;
    }
    assert(Number.isInteger(n));
    assert(n > 0);
    let first = Math.max(this.firstCodePoint - n, CODE_POINT_MIN);
    let last = Math.min(this.lastCodePoint + n, CODE_POINT_MAX);
    return new UnicodeSpan(first, last);
  }

  canMerge(other) {
    assert(other instanceof UnicodeSpan);
    return !other.expand(1).intersect(this).isEmpty;
  }

  merge(other) {
    assert(other instanceof UnicodeSpan);
    if (this.includes(other)) {
      return this;
    }
    if (other.includes(this)) {
      return other;
    }
    if (this.has(other.firstCodePoint)) {
      return new UnicodeSpan(this.firstCodePoint, other.lastCodePoint);
    }
    if (this.has(other.lastCodePoint)) {
      return new UnicodeSpan(other.firstCodePoint, this.lastCodePoint);
    }
    const expanded = other.expand(1);
    if (this.firstCodePoint === expanded.lastCodePoint) {
      return new UnicodeSpan(other.firstCodePoint, this.lastCodePoint);
    }
    assert(this.lastCodePoint === expanded.firstCodePoint);
    return new UnicodeSpan(this.firstCodePoint, other.lastCodePoint);
  }

  intersect(other) {
    assert(other instanceof UnicodeSpan);
    if (other.isEmpty) {
      return UnicodeSpan.EMPTY;
    }
    // this
    // --*----#----#-------*-------
    //      other
    if (this.includes(other)) {
      return other;
    }
    //       this
    // --#----*----*-------#-------
    // other
    if (other.includes(this)) {
      return this;
    }
    // this
    // --*----#----*-------#-------
    //      other
    if (this.has(other.firstCodePoint)) {
      return new UnicodeSpan(other.firstCodePoint, this.lastCodePoint);
    }
    //      this
    // --#----*----#-------*-------
    // other
    if (this.has(other.lastCodePoint)) {
      return new UnicodeSpan(this.firstCodePoint, other.lastCodePoint);
    }
    // this
    // --*-------*----#-------#-------
    //              other
    //
    //              this
    // --#-------#----*-------*-------
    // other
    return UnicodeSpan.EMPTY;
  }

  exclude(other) {
    assert(other instanceof UnicodeSpan);
    if (this.isEmpty) {
      return [];
    }
    if (other.isEmpty ||
        this.firstCodePoint > other.lastCodePoint ||
        this.lastCodePoint < other.firstCodePoint) {
      // No intersection.
      return [this];
    }
    const remaining = [];
    if (this.has(other.firstCodePoint) &&
        this.firstCodePoint !== other.firstCodePoint) {
      remaining.push(new UnicodeSpan(this.firstCodePoint, other.prevCodePoint));
    }
    if (this.has(other.lastCodePoint) &&
        this.lastCodePoint !== other.lastCodePoint) {
      remaining.push(new UnicodeSpan(other.nextCodePoint, this.lastCodePoint));
    }
    return remaining;
  }

  toString() {
    if (this.isEmpty) {
      return `()`;
    }
    if (this.length === 1) {
      return this.firstDisplayChar;
    }
    return `${this.firstDisplayChar}..${this.lastDisplayChar}`;
  }
}

export class UnicodePattern {
  constructor(prop, exclude = false) {
    this.prop = prop;
    this.exclude = exclude
    // immutable
    Object.freeze(this);
  }
}

export class UnicodeSet {
  static EMPTY = new UnicodeSet();
  static ANY = new UnicodeSet(UnicodeSpan.ANY);
  static TAB = new UnicodeSet(new UnicodeSpan('\u0009'));
  static VT = new UnicodeSet(new UnicodeSpan('\u000B'));
  static FF = new UnicodeSet(new UnicodeSpan('\u000C'));
  static SP = new UnicodeSet(new UnicodeSpan(' '));
  static USP = new UnicodeSet([
    new UnicodeSpan(' '),
    new UnicodeSpan('\u00A0'),
    new UnicodeSpan('\u1680'),
    new UnicodeSpan('\u2000', '\u200A'),
    new UnicodeSpan('\u200F'),
    new UnicodeSpan('\u205F'),
  ]);
  static LF = new UnicodeSet(new UnicodeSpan('\u000A'));
  static CR = new UnicodeSet(new UnicodeSpan('\u000D'));
  static LS = new UnicodeSet(new UnicodeSpan('\u2028'));
  static PS = new UnicodeSet(new UnicodeSpan('\u2029'));
  static ZWNJ = new UnicodeSet(new UnicodeSpan('\u200C'));
  static ZWJ = new UnicodeSet(new UnicodeSpan('\u200D'));
  static ZWNBSP = new UnicodeSet(new UnicodeSpan('\uFEFF'));

  constructor(value) {
    let spans = [];
    if (value === undefined || value === null) {
      // Nothing to do.
    } else if (value instanceof UnicodeSpan) {
      spans = [value];
    } else if (Array.isArray(value)) {
      assert(value.every((v) => v instanceof UnicodeSpan));
      // Assumed that `value` has already been normalized.
      spans = Array.from(value);
    } else {
      unreachable();
    }

    this.spans_ = spans.filter((span) => !span.isEmpty);

    // immutable
    Object.freeze(this);
    Object.freeze(this.spans_);
  }

  get isEmpty() {
    return this.spans_.length === 0;
  }

  get spans() {
    return this.spans_;
  }

  equals(other) {
    assert(other instanceof UnicodeSet);
    if (this === other) {
      return true;
    }
    if (this.spans_.length !== other.spans_.length) {
      return false;
    }
    if (!this.spans_.every((span, i) => span.equals(other.spans_[i]))) {
      return false;
    }
    return true;
  }

  includes(other) {
    assert(other instanceof UnicodeSet);
    // There are more efficient algorithms, but we choice a simple one which
    // takes O(N*M) time complexity, from maintenance cost point of view.
    return other.spans_.every((span) => this.includesSpan(span));
  }

  includesSpan(span) {
    return this.spans_.some((thisSpan) => thisSpan.includes(span));
  }

  merge(other) {
    assert(other instanceof UnicodeSet);
    if (other.isEmpty) {
      return this;
    }
    // There are more efficient algorithms, but we choice a simple one which
    // takes O(N*M) time complexity, from maintenance cost point of view.
    let cc = this;
    for (const span of other.spans_) {
      cc = cc.mergeSpan(span);
    }
    return cc;
  }

  mergeSpan(span) {
    assert(!span.isEmpty);
    const list = [];
    let added = false;
    for (const thisSpan of this.spans_) {
      if (span.canMerge(thisSpan)) {
        span = span.merge(thisSpan);
      } else if (span.firstCodePoint > thisSpan.lastCodePoint) {
        list.push(thisSpan);
      } else {
        assert(span.lastCodePoint < thisSpan.firstCodePoint);
        if (!added) {
          list.push(span);
          added = true;
        }
        list.push(thisSpan);
      }
    }
    if (!added) {
      list.push(span);
    }
    return new UnicodeSet(list);
  }

  intersect(other) {
    assert(other instanceof UnicodeSet);
    if (other.isEmpty) {
      return UnicodeSet.EMPTY;
    }
    // There are more efficient algorithms, but we choice a simple one which
    // takes O(N*M) time complexity, from maintenance cost point of view.
    let spans = [];
    for (const span of other.spans_) {
      const intersection = this.intersectSpan(span);
      spans = spans.concat(intersection.spans_);
    }
    return new UnicodeSet(spans);
  }

  intersectSpan(span) {
    assert(!span.isEmpty);
    const values = [];
    for (const thisSpan of this.spans_) {
      const intersection = thisSpan.intersect(span);
      if (!intersection.isEmpty) {
        values.push(intersection);
      }
    }
    return new UnicodeSet(values);
  }

  exclude(other) {
    assert(other instanceof UnicodeSet);
    if (other.isEmpty) {
      return this;
    }
    // There are more efficient algorithms, but we choice a simple one which
    // takes O(N*M) time complexity, from maintenance cost point of view.
    let cc = this;
    for (const span of other.spans_) {
      cc = cc.excludeSpan(span);
    }
    return cc;
  }

  excludeSpan(span) {
    assert(!span.isEmpty);
    let values = [];
    for (const thisSpan of this.spans_) {
      values = values.concat(thisSpan.exclude(span));
    }
    return new UnicodeSet(values);
  }

  toString() {
    return `[${this.spans_.join(', ')}]`;
  }
}

export class UnicodeSetsBuilder {
  constructor() {
    this.sets_ = [];  // immutable
  }

  includes(us) {
    return this.sets_.some((thisUs) => thisUs.equals(us));
  }

  add(us) {
    if (us.isEmpty) {
      return this;
    }
    const sets = [];
    for (const thisUs of this.sets_) {
      const intersection = us.intersect(thisUs);
      if (intersection.isEmpty) {
        sets.push(thisUs);
      } else {
        sets.push(intersection);
        const remaining = thisUs.exclude(intersection);
        if (!remaining.isEmpty) {
          sets.push(remaining);
        }
        us = us.exclude(intersection);
      }
    }
    if (!us.isEmpty) {
      sets.push(us);
    }
    this.sets_ = sets;
    return this;
  }

  build() {
    const sets = this.sets_;
    this.sets_ = [];
    return sets;
  }

  toString() {
    return `[${this.sets_.join(', ')}]`;
  }
}
