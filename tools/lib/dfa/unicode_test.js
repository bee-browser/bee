import {
  assert,
  assertEquals,
  assertExists,
  assertThrows,
} from 'https://deno.land/std@0.187.0/testing/asserts.ts';

import {
  describe,
  it,
} from 'https://deno.land/std@0.187.0/testing/bdd.ts';

import { UnicodeSpan, UnicodeSet, UnicodeSetsBuilder } from './unicode.js';

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

describe('UnicodeSet', () => {
  it('has EMPTY', () => {
    assertExists(UnicodeSet.EMPTY);
    assert(UnicodeSet.EMPTY.isEmpty);
  });

  it('has ANY', () => {
    assertExists(UnicodeSet.ANY);
    assert(!UnicodeSet.ANY.isEmpty);
  });

  it('constructor', () => {
    const empty = new UnicodeSet();
    assert(empty.isEmpty);

    const single = new UnicodeSet(new UnicodeSpan('a'));
    assert(!single.isEmpty);

    const unicodeSet = new UnicodeSet([new UnicodeSpan('a'), new UnicodeSpan('b')]);
    assert(!unicodeSet.isEmpty);

    assertThrows(() => new UnicodeSet({}));
  });

  it('spans', () => {
    const unicodeSet = new UnicodeSet(new UnicodeSpan('a'));
    assertExists(unicodeSet.spans);
    assertEquals(unicodeSet.spans.length, 1);
    assertThrows(() => unicodeSet.spans = []);  // immutable
  });

  it('isEmpty', () => {
    const unicodeSet = new UnicodeSet(new UnicodeSpan('a'));
    assert(!unicodeSet.isEmpty);
    assert(UnicodeSet.EMPTY.isEmpty);
    assertThrows(() => unicodeSet.isEmpty = true);  // immutable
  });

  it('includes', () => {
    const digit = new UnicodeSet(new UnicodeSpan('0', '9'));
    const upper = new UnicodeSet(new UnicodeSpan('A', 'Z'));
    const lower = new UnicodeSet(new UnicodeSpan('a', 'z'));
    const upperAlnum = new UnicodeSet([
      new UnicodeSpan('0', '9'),
      new UnicodeSpan('A', 'Z'),
    ]);
    const lowerAlnum = new UnicodeSet([
      new UnicodeSpan('0', '9'),
      new UnicodeSpan('a', 'z'),
    ]);
    assert(lowerAlnum.includes(digit));
    assert(lowerAlnum.includes(lower));
    assert(lowerAlnum.includes(lowerAlnum));
    assert(!lowerAlnum.includes(upper));
    assert(!lowerAlnum.includes(upperAlnum));
  });

  it('merge', () => {
    const digit = new UnicodeSet(new UnicodeSpan('0', '9'));
    const alpha = new UnicodeSet(new UnicodeSpan('a', 'z'));
    const alnum = new UnicodeSet([
      new UnicodeSpan('0', '9'),
      new UnicodeSpan('a', 'z'),
    ]);
    assertEquals(digit.merge(UnicodeSet.EMPTY), digit);
    assertEquals(digit.merge(alpha), alnum);
    assertEquals(alpha.merge(digit), alnum);
    assertEquals(
      alnum.merge(new UnicodeSet(new UnicodeSpan('5', 'k'))),
      new UnicodeSet(new UnicodeSpan('0', 'z')),
    );
  });

  it('intersect', () => {
    const digit = new UnicodeSet(new UnicodeSpan('0', '9'));
    const alpha = new UnicodeSet(new UnicodeSpan('a', 'z'));
    const alnum = new UnicodeSet([
      new UnicodeSpan('0', '9'),
      new UnicodeSpan('a', 'z'),
    ]);
    assertEquals(digit.intersect(UnicodeSet.EMPTY), UnicodeSet.EMPTY);
    assertEquals(UnicodeSet.EMPTY.intersect(digit), UnicodeSet.EMPTY);
    assertEquals(digit.intersect(digit), digit);
    assertEquals(digit.intersect(alpha), UnicodeSet.EMPTY);
    assertEquals(alnum.intersect(digit), digit);
    assertEquals(
      alnum.intersect(new UnicodeSet(new UnicodeSpan('5', 'k'))),
      new UnicodeSet([
        new UnicodeSpan('5', '9'),
        new UnicodeSpan('a', 'k'),
      ]),
    );
  });

  it('exclude', () => {
    const digit = new UnicodeSet(new UnicodeSpan('0', '9'));
    const alpha = new UnicodeSet(new UnicodeSpan('a', 'z'));
    const alnum = new UnicodeSet([
      new UnicodeSpan('0', '9'),
      new UnicodeSpan('a', 'z'),
    ]);
    assertEquals(digit.exclude(UnicodeSet.EMPTY), digit);
    assertEquals(digit.exclude(alpha), digit);
    assertEquals(alnum.exclude(digit), alpha);
    assertEquals(
      digit.exclude(new UnicodeSet(new UnicodeSpan('4', '6'))),
      new UnicodeSet([
        new UnicodeSpan('0', '3'),
        new UnicodeSpan('7', '9'),
      ]),
    );
    assertEquals(
      alnum.exclude(new UnicodeSet(new UnicodeSpan('5', 'k'))),
      new UnicodeSet([
        new UnicodeSpan('0', '4'),
        new UnicodeSpan('l', 'z'),
      ]),
    );
  });

  it('toString', () => {
    const digit = new UnicodeSet(new UnicodeSpan('0', '9'));
    const alnum = new UnicodeSet([
      new UnicodeSpan('0', '9'),
      new UnicodeSpan('a', 'z'),
    ]);
    assertEquals(UnicodeSet.EMPTY.toString(), '[]');
    assertEquals(digit.toString(), '[0..9]');
    assertEquals(alnum.toString(), '[0..9, a..z]');
  });
});

describe('UnicodeSetsBuilder', () => {
  it('includes', () => {
    let builder = new UnicodeSetsBuilder();
    const space = new UnicodeSet(new UnicodeSpan(' '));
    assert(!builder.includes(space));
    builder.add(space);
    assert(builder.includes(space));
    assert(!builder.includes(new UnicodeSet(new UnicodeSpan('A'))));
  });

  it('add', () => {
    let builder = new UnicodeSetsBuilder();

    const alnum = new UnicodeSet([
      new UnicodeSpan('0', '9'),
      new UnicodeSpan('A', 'Z'),
      new UnicodeSpan('a', 'z'),
    ]);
    builder.add(alnum);
    assertEquals(builder.sets_.length, 1);
    assert(builder.includes(alnum));

    builder.add(UnicodeSet.EMPTY);
    assertEquals(builder.sets_.length, 1);
    assert(builder.includes(alnum));

    const space = new UnicodeSet(new UnicodeSpan(' '));
    builder.add(space);
    assertEquals(builder.sets_.length, 2);
    assert(builder.includes(space));
    assert(builder.includes(alnum));

    const digit = new UnicodeSet(new UnicodeSpan('0', '9'));
    builder.add(digit);
    assertEquals(builder.sets_.length, 3);
    assert(builder.includes(space));
    assert(builder.includes(digit));
    assert(builder.includes(new UnicodeSet([
      new UnicodeSpan('A', 'Z'),
      new UnicodeSpan('a', 'z'),
    ])));

    builder.add(new UnicodeSet(new UnicodeSpan('5', 'K')));
    assertEquals(builder.sets_.length, 6);
    assert(builder.includes(space));
    assert(builder.includes(new UnicodeSet(new UnicodeSpan('0', '4'))));
    assert(builder.includes(new UnicodeSet(new UnicodeSpan('5', '9'))));
    assert(builder.includes(new UnicodeSet(new UnicodeSpan(':', '@'))));
    assert(builder.includes(new UnicodeSet(new UnicodeSpan('A', 'K'))));
    assert(builder.includes(new UnicodeSet([
      new UnicodeSpan('L', 'Z'),
      new UnicodeSpan('a', 'z'),
    ])));
  });

  it('build', () => {
    let builder = new UnicodeSetsBuilder();
    builder.add(new UnicodeSet(new UnicodeSpan(0)));
    builder.build();
    assertEquals(builder.sets_.length, 0);
  });

  it('toString', () => {
    let builder = new UnicodeSetsBuilder();
    assertEquals(builder.toString(), '[]');
    builder.add(new UnicodeSet(new UnicodeSpan('a')));
    assertEquals(builder.toString(), '[[a]]');
  });
});
