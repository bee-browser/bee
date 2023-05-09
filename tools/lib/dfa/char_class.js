import {
  assert,
  unreachable,
} from 'https://deno.land/std@0.186.0/testing/asserts.ts';

import { UnicodePattern, UnicodeSpan } from './unicode.js';

function normalize(spans) {
  let normalized = [];
  for (let span of spans) {
    let tmp = [];
    for (const span2 of normalized) {
      if (span.canMerge(span2)) {
        span = span.merge(span2);
      } else {
        tmp.push(span2);
      }
    }
  }
}

export class CharClass {
  static EMPTY = new CharClass();
  static ANY = new CharClass(UnicodeSpan.ANY);
  static TAB = new CharClass(new UnicodeSpan('\u0009'));
  static VT = new CharClass(new UnicodeSpan('\u000B'));
  static FF = new CharClass(new UnicodeSpan('\u000C'));
  static SP = new CharClass(new UnicodeSpan(' '));
  static USP = new CharClass([
    new UnicodeSpan(' '),
    new UnicodeSpan('\u00A0'),
    new UnicodeSpan('\u1680'),
    new UnicodeSpan('\u2000', '\u200A'),
    new UnicodeSpan('\u200F'),
    new UnicodeSpan('\u205F'),
  ]);
  static LF = new CharClass(new UnicodeSpan('\u000A'));
  static CR = new CharClass(new UnicodeSpan('\u000D'));
  static LS = new CharClass(new UnicodeSpan('\u2028'));
  static PS = new CharClass(new UnicodeSpan('\u2029'));
  static ZWNJ = new CharClass(new UnicodeSpan('\u200C'));
  static ZWJ = new CharClass(new UnicodeSpan('\u200D'));
  static ZWNBSP = new CharClass(new UnicodeSpan('\uFEFF'));

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
    assert(other instanceof CharClass);
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
    assert(other instanceof CharClass);
    // There are more efficient algorithms, but we choice a simple one which
    // takes O(N*M) time complexity, from maintenance cost point of view.
    return other.spans_.every((span) => this.includesUnicodeSpan(span));
  }

  includesUnicodeSpan(span) {
    return this.spans_.some((thisSpan) => thisSpan.includes(span));
  }

  merge(other) {
    assert(other instanceof CharClass);
    if (other.isEmpty) {
      return this;
    }
    // There are more efficient algorithms, but we choice a simple one which
    // takes O(N*M) time complexity, from maintenance cost point of view.
    let cc = this;
    for (const span of other.spans_) {
      cc = cc.mergeUnicodeSpan(span);
    }
    return cc;
  }

  mergeUnicodeSpan(span) {
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
    return new CharClass(list);
  }

  intersect(other) {
    assert(other instanceof CharClass);
    if (other.isEmpty) {
      return CharClass.EMPTY;
    }
    // There are more efficient algorithms, but we choice a simple one which
    // takes O(N*M) time complexity, from maintenance cost point of view.
    let spans = [];
    for (const span of other.spans_) {
      const intersection = this.intersectUnicodeSpan(span);
      spans = spans.concat(intersection.spans_);
    }
    return new CharClass(spans);
  }

  intersectUnicodeSpan(span) {
    assert(!span.isEmpty);
    const values = [];
    for (const thisSpan of this.spans_) {
      const intersection = thisSpan.intersect(span);
      if (!intersection.isEmpty) {
        values.push(intersection);
      }
    }
    return new CharClass(values);
  }

  exclude(other) {
    assert(other instanceof CharClass);
    if (other.isEmpty) {
      return this;
    }
    // There are more efficient algorithms, but we choice a simple one which
    // takes O(N*M) time complexity, from maintenance cost point of view.
    let cc = this;
    for (const span of other.spans_) {
      cc = cc.excludeUnicodeSpan(span);
    }
    return cc;
  }

  excludeUnicodeSpan(span) {
    assert(!span.isEmpty);
    let values = [];
    for (const thisSpan of this.spans_) {
      values = values.concat(thisSpan.exclude(span));
    }
    return new CharClass(values);
  }

  toString() {
    return `[${this.spans_.join(', ')}]`;
  }
}

export class CharClassListBuilder {
  constructor() {
    this.list_ = [];  // immutable
  }

  includes(cc) {
    return this.list_.some((thisCc) => thisCc.equals(cc));
  }

  add(cc) {
    if (cc.isEmpty) {
      return this;
    }
    const newList = [];
    for (const thisCc of this.list_) {
      const intersection = cc.intersect(thisCc);
      if (intersection.isEmpty) {
        newList.push(thisCc);
      } else {
        newList.push(intersection);
        const remaining = thisCc.exclude(intersection);
        if (!remaining.isEmpty) {
          newList.push(remaining);
        }
        cc = cc.exclude(intersection);
      }
    }
    if (!cc.isEmpty) {
      newList.push(cc);
    }
    this.list_ = newList;
    return this;
  }

  build() {
    const list = this.list_;
    this.list_ = [];
    return list;
  }

  toString() {
    return `[${this.list_.join(', ')}]`;
  }
}
