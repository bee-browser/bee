'use strict';

import {
  assert,
  assertEquals,
  assertExists,
} from 'https://deno.land/std@0.186.0/testing/asserts.ts';
import * as log from 'https://deno.land/std@0.186.0/log/mod.ts';
import * as yaml from 'https://deno.land/std@0.186.0/yaml/mod.ts';
import { parseCommand, readAllText } from '../../../tools/lib/cli.js';
import { setup } from '../../../tools/lib/log.js';

const PROGNAME = 'transpile.js';

const DOC = `
Transpile an ECMA lexical grammer from the text format to the YAML format.

Usage:
  ${PROGNAME} [options]
  ${PROGNAME} -h | --help

Options:
  -d, --debug
    Enable debug logs.
`.trim();

const HEADER = `
# DO NOT EDIT THIS FILE BY HAND.
#
# This file was automagically generated by ${PROGNAME}.
`.trim();

async function run(options) {
  if (options.debug) {
    setup(PROGNAME, 'DEBUG');
  } else {
    setup(PROGNAME, 'INFO');
  }
  const esgrammar = await readAllText(Deno.stdin);
  let rules = readRules(esgrammar);
  rules = rewriteReservedWord(rules);
  rules = rewritePunctuator(rules);
  rules = expandOptionals(rules);
  rules = expandParameterizedRules(rules);
  rules = translateRules(rules);
  rules = mergeCharClasses(rules);
  rules = simplify(rules);
  printYaml(rules.reduce((grammar, rule) => {
    grammar[rule.name] = {
      type: rule.type,
      data: rule.data,
    };
    return grammar;
  }, {
    SourceCharacter: { type: 'any' },
  }));
}

function readRules(esgrammar) {
  const STATE_RULE = 0;
  const STATE_MEMBER = 1;

  let state = STATE_RULE;
  let name;
  let values;
  let oneOf;

  const rules = [];
  for (const line of esgrammar.split('\n')) {
    if (state === STATE_RULE) {
      if (line.trim().length === 0) {
        continue;
      }
      const parts = line.trim().split(/\s+/u);
      name = parts.shift();
      if (parts[1] === 'one' && parts[2] === 'of') {
        oneOf = true;
      } else {
        oneOf = false;
      }
      values = [];
      state = STATE_MEMBER;
    } else if (state === STATE_MEMBER) {
      const trimed = line.trim();
      if (trimed.length === 0) {
        if (name === 'CodePoint') {
          // Special case: CodePoint
          rules.push({
            name,
            values: [
              'HexDigit',
              'HexDigit HexDigit',
              'HexDigit HexDigit HexDigit',
              'Hex4Digits',
              'HexDigit Hex4Digits',
              '`10` Hex4Digits',
              '`0` CodePoint',
            ],
          });
        } else if (name === 'NotCodePoint') {
          // Special case: NotCodePoint
          rules.push({
            name,
            values: [
              'NonZeroHexDigit NonZeroHexDigit Hex4Digits',
              'HexDigit NotCodePoint',
            ],
          });
        } else {
          rules.push({ name, values });
        }
        state = STATE_RULE;
        continue;
      }
      if (oneOf) {
        values = values.concat(trimed.split(/\s+/u));
      } else {
        values.push(trimed);
      }
    }
  }

  // Additional rules.
  rules.push({
    name: 'NonZeroHexDigit',
    values: ['HexDigit but not `0`'],
  });
  rules.push({
    name: 'WhiteSpaceSequence',
    values: ['WhiteSpace WhiteSpaceSequence?'],
  });

  return rules;
}

function rewriteReservedWord(rules) {
  log.info('Rewriting ReservedWord...');
  const rule = rules.find((rule) => rule.name === 'ReservedWord');
  assert(rule !== undefined);
  const values = rule.values;
  rule.values = [];
  for (const reserved of values) {
    rule.values.push(reserved.slice(1, -1).toUpperCase());
    rules.push({
      name: reserved.slice(1, -1).toUpperCase(),
      values: [reserved],
    });
  }
  const ADDITIONAL = [
    'async', 'from', 'get', 'meta', 'of', 'set', 'target',
    // For the strict mode
    'let', 'static', 'implements', 'interface', 'packege', 'private', 'protected', 'public',
  ];
  for (const word of ADDITIONAL) {
    rule.values.push(word.toUpperCase());
    rules.push({
      name: word.toUpperCase(),
      values: [`\`${word}\``],
    });
  }
  return rules;
}

function rewritePunctuator(rules) {
  const optionalChaining = rules.find((rule) => rule.name === 'OptionalChainingPunctuator');
  rules.push({
    name: 'OPTIONAL_CHAINING',
    values: optionalChaining.values,
  });
  optionalChaining.values = ['OPTIONAL_CHAINING'];

  const otherPunctuator = rules.find((rule) => rule.name === 'OtherPunctuator');
  const PUNCTUATORS = {
    '{': 'LBRACE',
    '}': 'RBRACE',
    '[': 'LBRACK',
    ']': 'RBRACK',
    '(': 'LPAREN',
    ')': 'RPAREN',
    '.': 'DOT',
    '...': 'ELLIPSIS',
    ';': 'SEMI_COLON',
    ',': 'COMMA',
    '<': 'LT',
    '>': 'GT',
    '<=': 'LTE',
    '>=': 'GTE',
    '==': 'EQ',
    '!=': 'NE',
    '===': 'EQ_STRICT',
    '!==': 'NE_STRICT',
    '+': 'ADD',
    '-': 'SUB',
    '*': 'MUL',
    '/': 'DIV',
    '%': 'MOD',
    '**': 'EXP',
    '++': 'INC',
    '--': 'DEC',
    '<<': 'SHL',
    '>>': 'SAR',
    '>>>': 'SHR',
    '&': 'BIT_AND',
    '|': 'BIT_OR',
    '^': 'BIT_XOR',
    '!': 'NOT',
    '~': 'BIT_NOT',
    '&&': 'AND',
    '||': 'OR',
    '??': 'NULLISH',
    '?': 'CONDITIONAL',
    ':': 'COLON',
    '=': 'ASSIGN',
    '+=': 'ADD_ASSIGN',
    '-=': 'SUB_ASSIGN',
    '*=': 'MUL_ASSIGN',
    '/=': 'DIV_ASSIGN',
    '%=': 'MOD_ASSIGN',
    '**=': 'EXP_ASSIGN',
    '<<=': 'SHL_ASSIGN',
    '>>=': 'SAR_ASSIGN',
    '>>>=': 'SHR_ASSIGN',
    '&=': 'BIT_AND_ASSIGN',
    '|=': 'BIT_OR_ASSIGN',
    '^=': 'BIT_XOR_ASSIGN',
    '&&=': 'AND_ASSIGN',
    '||=': 'OR_ASSIGN',
    '??=': 'NULLISH_ASSIGN',
    '=>': 'ARROW',
  };
  otherPunctuator.values.forEach((value) => {
    rules.push({
      name: PUNCTUATORS[value.slice(1, -1)],
      values: [value]
    });
  });
  otherPunctuator.values = otherPunctuator.values.map((value) => {
    return PUNCTUATORS[value.slice(1, -1)];
  });

  const divPunctuator = rules.find((rule) => rule.name === 'DivPunctuator');
  divPunctuator.values.forEach((value) => {
    rules.push({
      name: PUNCTUATORS[value.slice(1, -1)],
      values: [value],
    });
  });
  divPunctuator.values = divPunctuator.values.map((value) => {
    return PUNCTUATORS[value.slice(1, -1)];
  });

  const rightBracePunctuator = rules.find((rule) => rule.name === 'RightBracePunctuator');
  rightBracePunctuator.values.forEach((value) => {
    rules.push({
      name: PUNCTUATORS[value.slice(1, -1)],
      values: [value],
    });
  });
  rightBracePunctuator.values = rightBracePunctuator.values.map((value) => {
    return PUNCTUATORS[value.slice(1, -1)];
  });

  return rules;
}

function expandOptionals(rules) {
  log.info('Expanding optionals...');
  const expanded = [];
  for (const rule of rules) {
    const values = [];
    let hasOptionals = false;
    for (const value of rule.values) {
      const parts = value.split(/\s+/u);
      let patterns = [[]];
      for (let part of parts) {
        if (part.endsWith('?')) {
          hasOptionals = true;
          const clone = patterns.map((pattern) => Array.from(pattern));
          part = part.slice(0, -1);
          clone.forEach((pattern) => pattern.push(part));
          patterns = patterns.concat(clone);
        } else {
          patterns.forEach((pattern) => pattern.push(part));
        }
      }
      patterns.forEach((pattern) => values.push(pattern.join(' ')));
    }
    if (hasOptionals) {
      log.debug(`  ${rule.name}`);
    }
    expanded.push({
      name: rule.name,
      values,
    });
  }
  return expanded;
}

function expandParameterizedRules(rules) {
  log.info('Expanding parameterized rules...');
  const expanded = [];

  for (const rule of rules) {
    if (rule.name.endsWith(']')) {
      log.debug(`  ${rule.name}`);
      // TODO: multiple parameters.
      const [name, param] = rule.name.split(/[\]\[]/u);
      // Append name without param
      const valuesWithoutParam = [];
      for (let value of rule.values) {
        if (value.startsWith(`[+${param}]`)) {
          continue;
        }
        if (value.startsWith(`[~${param}]`)) {
          value = value.slice(param.length + 3).trim();
        }
        valuesWithoutParam.push(
          value
            .replaceAll(`[?${param}]`, '')
            .replaceAll(`[~${param}]`, '')
            .replaceAll(`[+${param}]`, `_${param}`));
      }
      expanded.push({
        name,
        values: valuesWithoutParam,
      });
      // Append name with param.
      const valuesWithParam = [];
      for (let value of rule.values) {
        if (value.startsWith(`[~${param}]`)) {
          continue;
        }
        if (value.startsWith(`[+${param}]`)) {
          value = value.slice(param.length + 3).trim();
        }
        valuesWithParam.push(
          value
            .replaceAll(`[?${param}]`, `_${param}`)
            .replaceAll(`[~${param}]`, '')
            .replaceAll(`[+${param}]`, `_${param}`));
      }
      expanded.push({
        name: `${name}_${param}`,
        values: valuesWithParam,
      });
    } else {
      expanded.push({
        name: rule.name,
        values: rule.values.map((value) => {
          value = value.replaceAll(/\[\+(\w+)\]/g, '_$1');
          value = value.replaceAll(/\[~\w+\]/g, '');
          return value;
        }),
      });
    }
  }
  return expanded;
}

function translateRules(rules) {
  const grammar = [];
  for (const rule of rules) {
    log.info(`Translating ${rule.name}...`);
    grammar.push({
      type: 'one-of',
      name: rule.name,
      data: rule.values.map((value) => translateProduction(value)),
    });
  }
  return grammar;
}

function translateProduction(production) {
  log.debug(`  ${production}`);

  // Special case: ID_Start
  if (production === '> any Unicode code point with the Unicode property “ID_Start”') {
    // TODO:
    // return {
    //   type: 'unicode-set',
    //   data: [{ type: 'property', data: 'ID_Start' }],
    // };
    return {
      type: 'unicode-set',
      data: [
        { type: 'span', data: ['a', 'z'] },
        { type: 'span', data: ['A', 'Z'] },
        { type: 'char', data: '$' },
        { type: 'char', data: '_' },
      ]
    };
  }

  // Special case: ID_Continue
  if (production === '> any Unicode code point with the Unicode property “ID_Continue”') {
    // TODO:
    // return {
    //   type: 'unicode-set',
    //   data: [{ type: 'property',  data: 'ID_Continue' }],
    // };
    return {
      type: 'unicode-set',
      data: [
        { type: 'span', data: ['0', '9'] },
        { type: 'span', data: ['a', 'z'] },
        { type: 'span', data: ['A', 'Z'] },
        { type: 'char', data: '$' },
        { type: 'char', data: '_' },
      ]
    };
  }

  // Special case: X but not one of ...
  if (production.includes('but not one of')) {
    production = production.replace('but not one of', '');
    production = production.replaceAll(' or', '');
    const [base, ...excludes] = production.split(/\s+/u);
    return {
      type: 'unicode-set',
      data: [
        { type: 'non-terminal', data: base },
        ...excludes.map((exclude) => {
          if (exclude.startsWith('`')) {
            return { type: 'exclude', data: exclude.slice(1, -1) };
          }
          return { type: 'exclude', data: exclude };
        }),
      ]
    };
  }

  // Special case: X but not ...
  if (production.includes('but not')) {
    production = production.replace('but not', '');
    const [base, ...excludes] = production.split(/\s+/u);
    return {
      type: 'unicode-set',
      data: [
        { type: 'non-terminal', data: base },
        ...excludes.map((exclude) => {
          if (exclude.startsWith('`')) {
            return { type: 'exclude', data: exclude.slice(1, -1) };
          }
          return { type: 'exclude', data: exclude };
        }),
      ]
    };
  }

  let seq = [];
  const items = production.split(/\s+/u);
  while (items.length > 0) {
    let item = items.shift();
    if (item.startsWith('`')) {
      const str = item.slice(1, -1);
      // We assume that `str` contains only ASCII characters.
      if (str.length === 1) {
        seq.push({
          type: 'unicode-set',
          data: [{ type: 'char', data: str }],
        });
      } else {
        seq.push({ type: 'word', data: str });
      }
    } else if (item.startsWith('<')) {
      seq.push({
        type: 'unicode-set',
        data: [{ type: 'built-in', data: item.slice(1, -1) }],
      });
    } else if (item === '[lookahead') {
      seq = seq.concat(translateLookahead(items));
    } else if (item === '[empty]') {
      seq.push({ type: 'empty' });
    } else {
      seq.push({ type: 'non-terminal', data: item });
    }
  }

  if (seq.length === 1) {
    return seq[0];
  }
  return { type: 'sequence', data: seq };
}

function translateLookahead(items) {
  let op = items.shift();
  let target = items.shift();
  let value;
  if (target === '{') {
    value = [];
    target = items.shift();
    while (target !== '}]') {
      if (target.endsWith(',')) {
        target = target.slice(0, -1);
      }
      value.push(target);
      target = items.shift();
    }
  } else {
    value = target.slice(0, -1);  // remove the last ']'
  }
  switch (op) {
  case '=':
    return translateLookaheadSet([value]);
  case '!=':
    return translateLookaheadSet([value], true/* negate */);
  case '\u2208':
    return translateLookaheadSet(value);
  case '\u2209':
    return translateLookaheadSet(value, true/* negate */);
  default:
    log.error(`translateLookahead: Unknown op: U+${op.codePointAt(0).toString(16)}`);
    Deno.exit(1);
  }
}

function translateLookaheadSet(values, negate = false) {
  const set = [];
  if (Array.isArray(values)) {
    for (let value of values) {
      if (value.startsWith('`')) {
        set.push({
          type: 'char',
          data: value.slice(1, -1),
        });
      } else if (value.startsWith('<')) {
        set.push({
          type: 'built-in',
          data: value.slice(1, -1),
        });
      } else {
        set.push({
          type: 'non-terminal',
          data: value,
        });
      }
    }
  } else {
    set.push({
      type: 'non-terminal',
      data: values,
    });
  }
  return {
    type: 'lookahead',
    data: set,
    negate,
  };
}

function mergeCharClasses(rules) {
  for (const rule of rules) {
    if (rule.type === 'one-of' &&
        rule.data.every((item) => item.type === 'unicode-set')) {
      log.info(`Merging character classes in ${rule.name}...`);
      let data = rule.data.reduce((data, item) => data.concat(item.data), []);
      rule.type = 'unicode-set';
      rule.data = data;
    }
  }
  return rules;
}

function simplify(rules) {
  for (const rule of rules) {
    if (rule.type === 'one-of' && rule.data.length === 1) {
      log.info(`Simplify ${rule.name}...`);
      rule.type = rule.data[0].type;
      rule.data = rule.data[0].data;
    }
  }
  return rules;
}

function printYaml(rules) {
  console.log(HEADER);
  console.log('');
  console.log(yaml.stringify(rules).trim());
}

const { options, args } = await parseCommand({
  doc: DOC,
});

Deno.exit(await run(options));
