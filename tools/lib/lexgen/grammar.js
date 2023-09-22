import * as log from 'https://deno.land/std@0.202.0/log/mod.ts';
import * as yaml from 'https://deno.land/std@0.202.0/yaml/mod.ts';
import { assert } from 'https://deno.land/std@0.202.0/testing/asserts.ts';
import Handlebars from 'npm:handlebars@4.7.8'
import { parseRegExpLiteral, validateRegExpLiteral, visitRegExpAST, } from 'npm:regexpp@3.2.0';
import { readAllText } from '../cli.js';
import { Pattern, EMPTY } from '../dfa/pattern.js';
import { compile } from '../dfa/compiler.js';

export class Grammar {
  static async fromYaml(file) {
    let data;
    if (file) {
      data = yaml.parse(await Deno.readTextFile(file));
    } else {
      data = yaml.parse(await readAllText(Deno.stdin));
    }
    data = Grammar.resolveVariables_(data);
    Grammar.validate_(data);
    return new Grammar(data);
  }

  static resolveVariables_(data) {
    const variables = {};
    for (const [name, regexp] of Object.entries(data.variables)) {
      variables[name] = render(regexp, data.variables);
    }
    const tokens = data
      .tokens
      .map(({ name, regexp }) => {
        if (regexp) {
          return {
            name,
            regexp: render(regexp, variables),
          };
        }
        return { name };
      });
    return { tokens, variables }
  }

  static validate_(data) {
    let abort = false;
    for (const [name, regexp] of Object.entries(data.variables)) {
      try {
        validateRegExpLiteral(`/${regexp}/u`);
      } catch (e) {
        log.error(`variables.${name}: ${e.message}`);
        abort = true;
      }
    }
    for (const { name, regexp } of data.tokens) {
      try {
        validateRegExpLiteral(`/${regexp}/u`);
      } catch (e) {
        log.error(`tokens.${name}: ${e.message}`);
        abort = true;
      }
    }
    if (abort) {
      Deno.exit(1);
    }
  }

  constructor(data) {
    this.data_ = data;
  }

  compile() {
    const tokens = this.data_.tokens
      .map(({ name, regexp }) => createToken(name, regexp));
    return compile(tokens);
  }
}

function render(regexp, variables) {
  for (;;) {
    const template = Handlebars.compile(regexp, {
      noEscape: true,
      strict: true,
    });
    const result = template(variables);
    if (result === regexp) {
      break;
    }
    regexp = result;
  }
  return regexp;
}

function createToken(name, regexp) {
  assert(typeof name === 'string');
  assert(name.length > 0);
  if (regexp === undefined) {
    regexp = name;
  } else if (regexp instanceof RegExp) {
    // Any flags are dropped.
    regexp = regexp.source;
  }
  assert(typeof regexp === 'string');
  log.debug(`${name}: ${regexp}`);
  // Always enable the unicode flag.
  const ast = parseRegExpLiteral(`/${regexp}/u`);
  const gen = new PatternTreeGenerator();
  visitRegExpAST(ast, gen);
  const expr = gen.result;
  return { name, regexp, expr };
}

class PatternTreeGenerator {
  constructor() {
    this.stack_ = [];
  }

  get result() {
    assert(this.stack_.length === 1);
    return this.stack_[0];
  }

  // Implementation of RegExpVisitor
  //
  // We implement on.*Leave methods because we traverse the tree in post-order.

  onAlternativeLeave(node) {
    if (node.elements.length === 1) {
      // Nothing to do.
    } else {
      let pat = this.stack_.pop();
      for (let i = 1; i < node.elements.length; ++i) {
        pat = this.stack_.pop().concat(pat);
      }
      this.stack_.push(pat);
    }
    //log.debug('onAlternativeLeave', this.stack_.toString());
  }

  onAssertionLeave(node) {
    log.debug('TODO: onAssertionLeave');
    Deno.exit(2);
  }

  onBackreferenceLeave(node) {
    log.error('TODO: onBackreferenceLeave');
    Deno.exit(2);
  }

  onCapturingGroupLeave(node) {
    //log.debug('onCapturingGroupLeave', node);
    if (node.alternatives.length === 1) {
      // Nothing to do.
    } else {
      let pat = this.stack_.pop();
      for (let i = 1; i < node.alternatives.length; ++i) {
        pat = this.stack_.pop().or(pat);
      }
      this.stack_.push(pat);
    }
  }

  onCharacterLeave(node) {
    //log.debug('onCharacterLeave', node);
    this.stack_.push(Pattern.unicodeSet([node.value]));
  }

  onCharacterClassLeave(node) {
    //log.debug('onCharacterClassLeave', node);
    let pat;
    if (node.elements.length === 1) {
      pat = this.stack_.pop();
    } else {
      pat = this.stack_.pop();
      for (let i = 1; i < node.elements.length; ++i) {
        pat = pat.merge(this.stack_.pop());
      }
    }
    if (node.negate) {
      pat = pat.invert();
    }
    this.stack_.push(pat);
  }

  onCharacterClassRangeLeave(node) {
    //log.debug('onCharacterClassRangeLeave', node);
    // Remove the min and max Char nodes from the stack.
    const max = this.stack_.pop();
    const min = this.stack_.pop();
    // And then add a new Char node.
    this.stack_.push(Pattern.unicodeSet([[node.min.value, node.max.value]]));
  }

  onCharacterSetLeave(node) {
    //log.debug('onCharacterSetLeave', node);
    let pat;
    switch (node.kind) {
    case 'any':
      this.stack_.push(Pattern.unicodeSet([], true));
      break;
    case 'space':
      this.stack_push(Pattern.unicodeSet([
        '\f', '\n', '\r', '\t', '\v',
        '\u0020', '\u00a0', '\u1680', ['\u2000', '\u200a'],
        '\u2028', '\u2029', '\u202f', '\u205f', '\u3000', '\ufeff',
      ], node.negate));
      break;
    case 'digit':
      this.stack_.push(Pattern.unicodeSet([['0', '9']], node.negate));
      break;
    case 'word':
      this.stack_push(Pattern.unicodeSet([
        ['A', 'Z'], ['a', 'z'], ['0', '9'], '_',
      ], node.negate));
      break;
    case 'property':
      log.error('onCharacterSetLeave', node)
      assert(false, 'unsupported');
      break;
    default:
      log.error('onCharacterSetLeave', node);
      break;
    }
  }

  onFlagsLeave(node) {
    assert(node.unicode);
    //log.debug('onFlagsLeave', node);
  }

  onGroupLeave(node) {
    log.error('onGroupLeave', node);
    Deno.exit(2);
  }

  onPatternLeave(node) {
    //log.debug('onPatternLeave', node);
    if (node.alternatives.length === 1) {
      // Nothing to do.
    } else {
      let pat = this.stack_.pop();
      for (let i = 1; i < node.alternatives.length; ++i) {
        pat = this.stack_.pop().or(pat);
      }
      this.stack_.push(pat);
    }
  }

  onQuantifierLeave(node) {
    const pat = this.stack_.pop();
    switch (`${node.min}..${node.max}`) {
    case '0..1':
      this.stack_.push(pat.zeroOrOne());
      break;
    case '0..Infinity':
      this.stack_.push(pat.zeroOrMore());
      break;
    case '1..Infinity':
      this.stack_.push(pat.oneOrMore());
      break;
    default:
      this.stack_.push(pat.repeat(node.min, node.max));
      break;
    }
    //log.debug('onQuantifierLeave', this.stack_.toString());
  }

  onRegExpLiteralLeave(node) {
    //log.debug('TODO: onRegExpLiteralLeave');
  }
}
