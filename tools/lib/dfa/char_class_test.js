import {
  assert,
  assertEquals,
  assertExists,
  assertThrows,
} from 'https://deno.land/std@0.184.0/testing/asserts.ts';

import {
  describe,
  it,
} from 'https://deno.land/std@0.184.0/testing/bdd.ts';

import { CharClass, CharClassListBuilder } from './char_class.js';
import { UnicodeSpan } from './unicode.js';

describe('CharClass', () => {
  it('has EMPTY', () => {
    assertExists(CharClass.EMPTY);
    assert(CharClass.EMPTY.isEmpty);
  });

  it('has ANY', () => {
    assertExists(CharClass.ANY);
    assert(!CharClass.ANY.isEmpty);
    assertEquals(CharClass.ANY.amount, 0x110000);
  });

  it('constructor', () => {
    const empty = new CharClass();
    assert(empty.isEmpty);

    const single = new CharClass(new UnicodeSpan('a'));
    assertEquals(single.amount, 1);

    const list = new CharClass([new UnicodeSpan('a'), new UnicodeSpan('b')]);
    assertEquals(list.amount, 2);

    assertThrows(() => new CharClass({}));
  });

  it('list', () => {
    const cc = new CharClass(new UnicodeSpan('a'));
    assertExists(cc.list);
    assertEquals(cc.list.length, 1);
    assertThrows(() => cc.list = []);  // immutable
  });

  it('amount', () => {
    const cc = new CharClass(new UnicodeSpan('a'));
    assertExists(cc.amount);
    assertEquals(cc.amount, 1);
    assertThrows(() => cc.amount = 0);  // immutable
  });

  it('isEmpty', () => {
    const cc = new CharClass(new UnicodeSpan('a'));
    assert(!cc.isEmpty);
    assert(CharClass.EMPTY.isEmpty);
    assertThrows(() => cc.isEmpty = true);  // immutable
  });

  it('includes', () => {
    const digit = new CharClass(new UnicodeSpan('0', '9'));
    const upper = new CharClass(new UnicodeSpan('A', 'Z'));
    const lower = new CharClass(new UnicodeSpan('a', 'z'));
    const upperAlnum = new CharClass([
      new UnicodeSpan('0', '9'),
      new UnicodeSpan('A', 'Z'),
    ]);
    const lowerAlnum = new CharClass([
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
    const digit = new CharClass(new UnicodeSpan('0', '9'));
    const alpha = new CharClass(new UnicodeSpan('a', 'z'));
    const alnum = new CharClass([
      new UnicodeSpan('0', '9'),
      new UnicodeSpan('a', 'z'),
    ]);
    assertEquals(digit.merge(CharClass.EMPTY), digit);
    assertEquals(digit.merge(alpha), alnum);
    assertEquals(alpha.merge(digit), alnum);
    assertEquals(
      alnum.merge(new CharClass(new UnicodeSpan('5', 'k'))),
      new CharClass(new UnicodeSpan('0', 'z')),
    );
  });

  it('intersect', () => {
    const digit = new CharClass(new UnicodeSpan('0', '9'));
    const alpha = new CharClass(new UnicodeSpan('a', 'z'));
    const alnum = new CharClass([
      new UnicodeSpan('0', '9'),
      new UnicodeSpan('a', 'z'),
    ]);
    assertEquals(digit.intersect(CharClass.EMPTY), CharClass.EMPTY);
    assertEquals(CharClass.EMPTY.intersect(digit), CharClass.EMPTY);
    assertEquals(digit.intersect(digit), digit);
    assertEquals(digit.intersect(alpha), CharClass.EMPTY);
    assertEquals(alnum.intersect(digit), digit);
    assertEquals(
      alnum.intersect(new CharClass(new UnicodeSpan('5', 'k'))),
      new CharClass([
        new UnicodeSpan('5', '9'),
        new UnicodeSpan('a', 'k'),
      ]),
    );
  });

  it('exclude', () => {
    const digit = new CharClass(new UnicodeSpan('0', '9'));
    const alpha = new CharClass(new UnicodeSpan('a', 'z'));
    const alnum = new CharClass([
      new UnicodeSpan('0', '9'),
      new UnicodeSpan('a', 'z'),
    ]);
    assertEquals(digit.exclude(CharClass.EMPTY), digit);
    assertEquals(digit.exclude(alpha), digit);
    assertEquals(alnum.exclude(digit), alpha);
    assertEquals(
      digit.exclude(new CharClass(new UnicodeSpan('4', '6'))),
      new CharClass([
        new UnicodeSpan('0', '3'),
        new UnicodeSpan('7', '9'),
      ]),
    );
    assertEquals(
      alnum.exclude(new CharClass(new UnicodeSpan('5', 'k'))),
      new CharClass([
        new UnicodeSpan('0', '4'),
        new UnicodeSpan('l', 'z'),
      ]),
    );
  });

  it('toString', () => {
    const digit = new CharClass(new UnicodeSpan('0', '9'));
    const alnum = new CharClass([
      new UnicodeSpan('0', '9'),
      new UnicodeSpan('a', 'z'),
    ]);
    assertEquals(CharClass.EMPTY.toString(), '[]');
    assertEquals(digit.toString(), '[0..9]');
    assertEquals(alnum.toString(), '[0..9, a..z]');
  });
});

describe('CharClassListBuilder', () => {
  it('includes', () => {
    let builder = new CharClassListBuilder();
    const space = new CharClass(new UnicodeSpan(' '));
    assert(!builder.includes(space));
    builder.add(space);
    assert(builder.includes(space));
    assert(!builder.includes(new CharClass(new UnicodeSpan('A'))));
  });

  it('add', () => {
    let builder = new CharClassListBuilder();

    const alnum = new CharClass([
      new UnicodeSpan('0', '9'),
      new UnicodeSpan('A', 'Z'),
      new UnicodeSpan('a', 'z'),
    ]);
    builder.add(alnum);
    assertEquals(builder.list_.length, 1);
    assert(builder.includes(alnum));

    builder.add(CharClass.EMPTY);
    assertEquals(builder.list_.length, 1);
    assert(builder.includes(alnum));

    const space = new CharClass(new UnicodeSpan(' '));
    builder.add(space);
    assertEquals(builder.list_.length, 2);
    assert(builder.includes(space));
    assert(builder.includes(alnum));

    const digit = new CharClass(new UnicodeSpan('0', '9'));
    builder.add(digit);
    assertEquals(builder.list_.length, 3);
    assert(builder.includes(space));
    assert(builder.includes(digit));
    assert(builder.includes(new CharClass([
      new UnicodeSpan('A', 'Z'),
      new UnicodeSpan('a', 'z'),
    ])));

    builder.add(new CharClass(new UnicodeSpan('5', 'K')));
    assertEquals(builder.list_.length, 6);
    assert(builder.includes(space));
    assert(builder.includes(new CharClass(new UnicodeSpan('0', '4'))));
    assert(builder.includes(new CharClass(new UnicodeSpan('5', '9'))));
    assert(builder.includes(new CharClass(new UnicodeSpan(':', '@'))));
    assert(builder.includes(new CharClass(new UnicodeSpan('A', 'K'))));
    assert(builder.includes(new CharClass([
      new UnicodeSpan('L', 'Z'),
      new UnicodeSpan('a', 'z'),
    ])));
  });

  it('build', () => {
    let builder = new CharClassListBuilder();
    builder.add(new CharClass(new UnicodeSpan(0)));
    builder.build();
    assertEquals(builder.list_.length, 0);
  });

  it('toString', () => {
    let builder = new CharClassListBuilder();
    assertEquals(builder.toString(), '[]');
    builder.add(new CharClass(new UnicodeSpan('a')));
    assertEquals(builder.toString(), '[[a]]');
  });
});
