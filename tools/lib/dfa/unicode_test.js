import {
  assert,
  assertEquals,
  assertExists,
  assertThrows,
} from 'https://deno.land/std@0.186.0/testing/asserts.ts';

import {
  describe,
  it,
} from 'https://deno.land/std@0.186.0/testing/bdd.ts';

import { UnicodeSpan } from './unicode.js';

describe('UnicodeSpan', () => {
  it('has EMPTY', () => {
    assertExists(UnicodeSpan.EMPTY);
    assert(UnicodeSpan.EMPTY.isEmpty);
  });

  it('has ANY', () => {
    assertExists(UnicodeSpan.ANY);
  });

  it('constructor', () => {
    const empty = new UnicodeSpan();
    assert(empty.isEmpty);

    const single = new UnicodeSpan('a');
    assert(single.length === 1);

    const span = new UnicodeSpan('0', '9');
    assert(span.length === 10);
  });

  it('length', () => {
    const empty = new UnicodeSpan();
    assertEquals(empty.length, 0);

    const single = new UnicodeSpan('a');
    assertEquals(single.length, 1);

    const span = new UnicodeSpan('0', '9');
    assertEquals(span.length, 10);

    const invalid = new UnicodeSpan('9', '0');
    assertEquals(invalid.length, -8);
  });

  it('isEmpty', () => {
    const empty = new UnicodeSpan();
    assert(empty.isEmpty);

    const single = new UnicodeSpan('a');
    assert(!single.isEmpty);

    const span = new UnicodeSpan('0', '9');
    assert(!span.isEmpty);

    const invalid = new UnicodeSpan('9', '0');
    assert(invalid.isEmpty);
  });

  it('firstCodePoint', () => {
    const span = new UnicodeSpan('0', '9');
    assertEquals(span.firstCodePoint, '0'.codePointAt(0));

    assertThrows(() => UnicodeSpan.EMPTY.firstCodePoint);
  });

  it('lastCodePoint', () => {
    const span = new UnicodeSpan('0', '9');
    assertEquals(span.lastCodePoint, '9'.codePointAt(0));

    assertThrows(() => UnicodeSpan.EMPTY.lastCodePoint);
  });

  it('prevCodePoint', () => {
    const span = new UnicodeSpan('0', '9');
    assertEquals(span.prevCodePoint, '/'.codePointAt(0));

    assertThrows(() => UnicodeSpan.EMPTY.prevCodePoint);
  });

  it('nextCodePoint', () => {
    const span = new UnicodeSpan('0', '9');
    assertEquals(span.nextCodePoint, ':'.codePointAt(0));

    assertThrows(() => UnicodeSpan.EMPTY.nextCodePoint);
  });

  it('equals', () => {
    const span = new UnicodeSpan(0, 10);
    assert(UnicodeSpan.EMPTY.equals(UnicodeSpan.EMPTY));
    assert(UnicodeSpan.EMPTY.equals(new UnicodeSpan()));
    assert(!span.equals(UnicodeSpan.EMPTY));
    assert(!UnicodeSpan.EMPTY.equals(span));
    assert(!span.equals(new UnicodeSpan(1, 10)));
    assert(!span.equals(new UnicodeSpan(0, 9)));
    assert(span.equals(new UnicodeSpan(0, 10)));
  });

  it('has', () => {
    const span = new UnicodeSpan('0', '9');
    assert(span.has('0'));
    assert(span.has('5'));
    assert(span.has('9'));
    assert(!span.has('/'));
    assert(!span.has(':'));

    assert(!UnicodeSpan.EMPTY.has(0));
  });

  it('includes', () => {
    const span = new UnicodeSpan('0', '9');
    assert(!span.includes(UnicodeSpan.EMPTY));
    assert(span.includes(span));
    assert(span.includes(new UnicodeSpan('1', '8')));
    assert(!span.includes(new UnicodeSpan('/', '5')));
    assert(!span.includes(new UnicodeSpan('5', ':')));
    assert(!span.includes(new UnicodeSpan('a', 'z')));
  });

  it('expand', () => {
    assertEquals(
      (new UnicodeSpan('0', '9')).expand(1), new UnicodeSpan('/', ':'));
    assertEquals(
      (new UnicodeSpan(0)).expand(1), new UnicodeSpan(0, 1));
    assertEquals(UnicodeSpan.EMPTY.expand(1), UnicodeSpan.EMPTY);
    assertEquals(UnicodeSpan.ANY.expand(1), UnicodeSpan.ANY);
  });

  it('canMerge', () => {
    assert((new UnicodeSpan(10, 19)).canMerge(new UnicodeSpan(11)));
    assert((new UnicodeSpan(10, 19)).canMerge(new UnicodeSpan(10)));
    assert((new UnicodeSpan(10, 19)).canMerge(new UnicodeSpan(9)));
    assert(!(new UnicodeSpan(10, 19)).canMerge(new UnicodeSpan(8)));
    assert((new UnicodeSpan(10, 19)).canMerge(new UnicodeSpan(10)));
    assert((new UnicodeSpan(10, 19)).canMerge(new UnicodeSpan(9)));
    assert((new UnicodeSpan(10, 19)).canMerge(new UnicodeSpan(18)));
    assert((new UnicodeSpan(10, 19)).canMerge(new UnicodeSpan(19)));
    assert((new UnicodeSpan(10, 19)).canMerge(new UnicodeSpan(20)));
    assert(!(new UnicodeSpan(10, 19)).canMerge(new UnicodeSpan(21)));
  })

  it('merge', () => {
    assertEquals(
      (new UnicodeSpan(10, 19)).merge(new UnicodeSpan(13)),
      new UnicodeSpan(10, 19),
    );
    assertEquals(
      (new UnicodeSpan(10, 19)).merge(new UnicodeSpan(5, 25)),
      new UnicodeSpan(5, 25),
    );
    assertEquals(
      (new UnicodeSpan(10, 19)).merge(new UnicodeSpan(15, 25)),
      new UnicodeSpan(10, 25),
    );
    assertEquals(
      (new UnicodeSpan(10, 19)).merge(new UnicodeSpan(5, 15)),
      new UnicodeSpan(5, 19),
    );
    assertEquals(
      (new UnicodeSpan(10, 19)).merge(new UnicodeSpan(9)),
      new UnicodeSpan(9, 19),
    );
    assertEquals(
      (new UnicodeSpan(10, 19)).merge(new UnicodeSpan(20)),
      new UnicodeSpan(10, 20),
    );
  });

  it('intersect', () => {
    const span = new UnicodeSpan('0', '9');

    assert(span.intersect(UnicodeSpan.EMPTY).isEmpty);

    // span contains other.
    assertEquals(
      span.intersect(new UnicodeSpan('1', '8')), new UnicodeSpan('1', '8'));

    // other contains span.
    assertEquals(
      span.intersect(new UnicodeSpan('/', ':')), new UnicodeSpan('0', '9'));

    // span contains other.firstCodePoint.
    assertEquals(
      span.intersect(new UnicodeSpan('5', ':')), new UnicodeSpan('5', '9'));

    // span contains other.lastCodePoint.
    assertEquals(
      span.intersect(new UnicodeSpan('/', '5')), new UnicodeSpan('0', '5'));

    // no intersection.
    assert(span.intersect(new UnicodeSpan('/')).isEmpty);
    assert(span.intersect(new UnicodeSpan(':')).isEmpty);
  });

  it('exclude', () => {
    const span = new UnicodeSpan('0', '9');

    assertEquals(UnicodeSpan.EMPTY.exclude(span), []);
    assertEquals(span.exclude(UnicodeSpan.EMPTY), [span]);
    assertEquals(span.exclude(span), []);

    // span contains other.
    assertEquals(
      span.exclude(new UnicodeSpan('1', '8')),
      [
        new UnicodeSpan('0'),
        new UnicodeSpan('9'),
      ],
    );

    // other contains span.
    assertEquals(
      span.exclude(new UnicodeSpan('/', ':')),
      [],
    );

    // span contains other.firstCodePoint.
    assertEquals(
      span.exclude(new UnicodeSpan('5', ':')),
      [new UnicodeSpan('0', '4')],
    );

    // span contains other.lastCodePoint.
    assertEquals(
      span.exclude(new UnicodeSpan('/', '5')),
      [new UnicodeSpan('6', '9')],
    );

    // no intersection.
    assertEquals(
      span.exclude(new UnicodeSpan('/')),
      [new UnicodeSpan('0', '9')],
    );
    assertEquals(
      span.exclude(new UnicodeSpan(':')),
      [new UnicodeSpan('0', '9')],
    );
  });

  it('toString', () => {
    assertEquals(UnicodeSpan.EMPTY.toString(), '()');
    assertEquals((new UnicodeSpan('a')).toString(), 'a');
    assertEquals((new UnicodeSpan('0', '9')).toString(), '0..9');
  });
});
