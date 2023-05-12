/**
 * Pattern tree.
 *
 * A token is a named regular expression.  A regular expression is represented
 * by using a binary tree called a pattern tree in this library.
 *
 * A pattern tree consists of the following immutable nodes.
 *
 * - Empty
 * - Accept
 * - Char
 * - Concatination
 * - Alternation
 * - Repetition
 *
 * @module pattern
 */

import {
  assert,
  assertExists,
} from 'https://deno.land/std@0.186.0/testing/asserts.ts';
import { UnicodeSpan, UnicodeSet } from './unicode.js';

class Node {
  constructor(left, right) {
    this.left = left ?? null;
    this.right = right ?? null;
  }

  get isLeaf() {
    return this.left === null && this.right === null;
  }

  traverseInPreorder(visit, depth = 0) {
    visit(this, depth);
    this.left?.traverseInPreorder(visit, depth + 1);
    this.right?.traverseInPreorder(visit, depth + 1);
  }

  traverseInInorder(visit, depth = 0) {
    this.left?.traverseInInorder(visit, depth + 1);
    visit(this, depth);
    this.right?.traverseInInorder(visit, depth + 1);
  }

  traverseInPostorder(visit, depth = 0) {
    this.left?.traverseInPostorder(visit, depth + 1);
    this.right?.traverseInPostorder(visit, depth + 1);
    visit(this, depth);
  }
}

export class Pattern extends Node {
  // There is no simple way to define class constants in the current ECMA262
  // specification.  As a workaround, we use mutable class variables, instead.
  static EMPTY = 1;
  static ACCEPT = 2;
  static CHAR = 3;
  static CONCATINATION = 4;
  static ALTERNATION = 5;
  static REPETITION = 6;

  static unicodeSet(list, exclude = false) {
    const spans = list.map((v) => {
      if (Array.isArray(v)) {
        assert(v.length === 2);
        return new UnicodeSpan(v[0], v[1]);
      }
      return new UnicodeSpan(v);
    });
    let unicodeSet = UnicodeSet.EMPTY;
    for (const span of spans) {
      unicodeSet = unicodeSet.mergeSpan(span);
    }
    if (exclude) {
      unicodeSet = UnicodeSet.ANY.exclude(unicodeSet);
    }
    return new Char(unicodeSet);
  }

  constructor(type, left, right) {
    assert(type === Pattern.EMPTY ||
           type === Pattern.ACCEPT ||
           type === Pattern.CHAR ||
           type === Pattern.CONCATINATION ||
           type === Pattern.ALTERNATION ||
           type === Pattern.REPETITION);
    super(left, right);
    this.type = type;
  }

  freeze() {
    Object.freeze(this);
  }

  concat(other) {
    return new Concatination(this, other);
  }

  or(other) {
    return new Alternation(this, other);
  }

  zeroOrOne() {
    return EMPTY.or(this);
  }

  zeroOrMore() {
    return new Repetition(this);
  }

  oneOrMore() {
    return this.concat(this.zeroOrMore());
  }

  repeat(n, m) {
    assert(n > 0);
    assert(n <= m);
    let seq = this;
    let i = 1;
    while (i < n) {
      seq = seq.concat(this);
      i += 1;
    }
    let node = seq;
    if (m === Infinity) {
      node = node.concat(this.zeroOrMore());
    } else {
      while (i < m) {
        seq = seq.concat(this);
        node = node.or(seq);
        i += 1;
      }
    }
    return node;
  }

  accept(token) {
    return this.concat(new Accept(token));
  }

  dump(logger) {
    this.traverseInPreorder((node, depth) => {
      logger(`${' '.repeat(depth)}${node}`);
    });
  }
}

class Empty extends Pattern {
  constructor() {
    super(Pattern.EMPTY);
    this.freeze();
  }

  toString() {
    return 'Empty';
  }
}

/**
 * A leaf node representing a terminal of a token.
 */
class Accept extends Pattern {
  constructor(token) {
    assertExists(token.name);
    assertExists(token.index);
    super(Pattern.ACCEPT);
    this.token = token;
    this.freeze();
  }

  toString() {
    return `Accept(${this.token.name})`;
  }
}

/**
 * A leaf node representing a single character belonging to a unicode set.
 */
class Char extends Pattern {
  static ANY = new Char(UnicodeSet.ANY);

  constructor(value) {
    assert(value instanceof UnicodeSet);
    Object.freeze(value);
    super(Pattern.CHAR);
    this.unicodeSet = value;
    this.freeze();
  }

  invert() {
    return new Char(UnicodeSet.ANY.exclude(this.unicodeSet));
  }

  merge(other) {
    assert(other instanceof Char);
    return new Char(this.unicodeSet.merge(other.unicodeSet));
  }

  toString() {
    return `Char(${this.unicodeSet})`;
  }
}

/**
 * A branch node representing a concatination of two nodes.
 */
class Concatination extends Pattern {
  constructor(left, right) {
    assert(left !== undefined);
    assert(right !== undefined);
    super(Pattern.CONCATINATION, left, right);
    this.freeze();
  }

  toString() {
    return 'Concatination';
  }
}

class Alternation extends Pattern {
  constructor(left, right) {
    assert(left !== undefined);
    assert(right !== undefined);
    super(Pattern.ALTERNATION, left, right);
    this.freeze();
  }

  toString() {
    return 'Alternation';
  }
}

/**
 * A branch node representing a repetition.
 */
class Repetition extends Pattern {
  constructor(node) {
    super(Pattern.REPETITION, node);
    this.freeze();
  }

  toString() {
    return 'Repetition';
  }
}

// operators

export const EMPTY = new Empty();

export function cp(value) {
  return new Char(new UnicodeSet(new UnicodeSpan(value)));
}

export function between(min, max) {
  return new Char(new UnicodeSet(new UnicodeSpan(min, max)));
}

export function zeroOrOne(node) {
  return new Alternation(EMPTY, node);
}

export function zeroOrMore(node) {
  return new Repetition(node);
}

export function oneOrMore(node) {
  return new Concatination(node, zeroOrMore(node));
}

export function oneOf(first, ...rest) {
  assert(first instanceof Pattern);
  if (rest.length === 0) {
    return first;
  }
  return new Alternation(first, oneOf(...rest));
}

export function seq(first, ...rest) {
  assert(first instanceof Pattern);
  if (rest.length === 0) {
    return first;
  }
  return first.concat(seq(...rest));
}

class Token {
  constructor(name, expr) {
    assert(typeof name === 'string');
    assert(name.length > 0);
    assert(expr instanceof Pattern);
    this.name = name;
    this.expr = expr;
  }

  toString() {
    return `Token(${this.name})`;
  }
}

export function token(name, expr) {
  return new Token(name, expr);
}
