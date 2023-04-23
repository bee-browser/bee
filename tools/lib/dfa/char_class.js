import {
  assert,
  unreachable,
} from 'https://deno.land/std@0.184.0/testing/asserts.ts';

import { UnicodeSpan } from './unicode.js';

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

  constructor(value) {
    let list;
    if (value === undefined || value === null) {
      list = [];
    } else if (value instanceof UnicodeSpan) {
      list = [value];
    } else if (Array.isArray(value)) {
      assert(value.every((v) => v instanceof UnicodeSpan));
      // Assumed that `value` has already been normalized.
      list = value;
    } else {
      unreachable();
    }
    this.list = list.filter((span) => !span.isEmpty);

    this.amount = 0;
    for (const span of this.list) {
      assert(span instanceof UnicodeSpan);
      this.amount += span.length;
    }

    // immutable
    Object.freeze(this);
    Object.freeze(this.list);
  }

  get isEmpty() {
    return this.amount === 0;
  }

  get spans() {
    return this.list;
  }

  equals(other) {
    assert(other instanceof CharClass);
    if (this === other) {
      return true;
    }
    if (this.list.length !== other.list.length) {
      return false;
    }
    for (let i = 0; i < this.list.length; ++i) {
      if (!this.list[i].equals(other.list[i])) {
        return false;
      }
    }
    return true;
  }

  includes(other) {
    assert(other instanceof CharClass);
    // There are more efficient algorithms, but we choice a simple one which
    // takes O(N*M) time complexity, from maintenance cost point of view.
    return other.list.every((span) => this.includesUnicodeSpan(span));
  }

  includesUnicodeSpan(span) {
    return this.list.some((thisSpan) => thisSpan.includes(span));
  }

  merge(other) {
    assert(other instanceof CharClass);
    if (other.isEmpty) {
      return this;
    }
    // There are more efficient algorithms, but we choice a simple one which
    // takes O(N*M) time complexity, from maintenance cost point of view.
    let cc = this;
    for (const span of other.list) {
      cc = cc.mergeUnicodeSpan(span);
    }
    return cc;
  }

  mergeUnicodeSpan(span) {
    assert(!span.isEmpty);
    const list = [];
    let added = false;
    for (const thisSpan of this.list) {
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
    let list = [];
    for (const span of other.list) {
      const intersection = this.intersectUnicodeSpan(span);
      list = list.concat(intersection.list);
    }
    return new CharClass(list);
  }

  intersectUnicodeSpan(span) {
    assert(!span.isEmpty);
    const list = [];
    for (const thisSpan of this.list) {
      const intersection = thisSpan.intersect(span);
      if (!intersection.isEmpty) {
        list.push(intersection);
      }
    }
    return new CharClass(list);
  }

  exclude(other) {
    assert(other instanceof CharClass);
    if (other.isEmpty) {
      return this;
    }
    // There are more efficient algorithms, but we choice a simple one which
    // takes O(N*M) time complexity, from maintenance cost point of view.
    let cc = this;
    for (const span of other.list) {
      cc = cc.excludeUnicodeSpan(span);
    }
    return cc;
  }

  excludeUnicodeSpan(span) {
    assert(!span.isEmpty);
    let list = [];
    for (const thisSpan of this.list) {
      list = list.concat(thisSpan.exclude(span));
    }
    return new CharClass(list);
  }

  toString() {
    return `[${this.list.join(', ')}]`;
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
