'use strict';

import {
  assert,
  assertEquals,
  assertExists,
  assertInstanceOf,
  unreachable,
} from 'https://deno.land/std@0.193.0/testing/asserts.ts';
import * as log from 'https://deno.land/std@0.193.0/log/mod.ts';
import * as yaml from 'https://deno.land/std@0.193.0/yaml/mod.ts';
import { parseCommand, readAllText } from '../../../../tools/lib/cli.js';
import { setup } from '../../../../tools/lib/log.js';

const PROGNAME = 'lalrgen.js';

const DOC = `
Generate LALR parsing tables from an ECMA syntactic grammar.

Usage:
  ${PROGNAME} [options] <goal-symbol>
  ${PROGNAME} -h | --help

Options:
  -d, --debug
    Enable debug logs.

Arguments:
  <goal-symbol>
    A goal symbol.
`.trim();

const recursionCheck = new Set();

async function run(options, { goalSymbol }) {
  if (options.debug) {
    setup(PROGNAME, 'DEBUG');
  } else {
    setup(PROGNAME, 'INFO');
  }
  const rules = yaml.parse(await readAllText(Deno.stdin));
  const grammar = {};
  for (const rule of rules) {
    if (grammar[rule.name] === undefined) {
      grammar[rule.name] = {
        type: 'one-of',
        data: [],
      };
    }
    grammar[rule.name].data.push({
      type: 'sequence',
      data: rule.production,
    });
  }
  const builder = new LalrBuilder(goalSymbol, grammar, 1);
  //builder.buildNullableTable();
  builder.buildFirstSetTable();
  builder.buildLr0ItemSets();
  builder.buildLookaheadTable();
  builder.buildLalrTables();
  console.log(JSON.stringify({
    goalSymbol,
    nonTerminals: Array.from(Object.keys(grammar)),
    states: builder.states_.map((state) => {
      return {
        actionTable: state.actionTable,
        gotoTable: state.gotoTable,
      };
    }),
  }));
}

class LalrBuilder {
  constructor(goalSymbol, grammar, k = 1) {
    this.grammar_ = grammar;
    this.goalSymbol_ = goalSymbol;
    this.k_ = k;
    this.initialProduction_ = [{ type: 'non-terminal', data: goalSymbol }];
    this.items_ = [];
    this.itemMap_ = new Map();  // item.repr -> item.id
    this.states_ = [];
    this.stateMap_ = new Map();  // state.repr -> state.id
    this.nonKernelItemSetCache_ = new Map();  // item.id -> set of non-kernel items
    this.closureCache2_ = {};  // repr of kernel items -> set of items
    this.closureCache_ = new Map();  // item.id -> set of item.id
  }

  buildFirstSetTable(n = 1) {
    log.debug('Building first-set table...');
    const cache = {
      firstSetTable: {},
      completes: new Set(),
    };
    const remaining = Object.keys(this.grammar_);
    remaining.push('$');  // sentinel
    let numIterations = 0;
    let numChanged = 0;
    while (remaining.length > 0) {
      let complete = false;
      const nonTerminal = remaining.shift();
      if (nonTerminal === '$') {
        log.debug(`  Finished Iteration#${numIterations}: ${numChanged} entries were changed`);
        if (numChanged === 0) {
          break;
        }
        remaining.push(nonTerminal);
        numIterations++;
        numChanged = 0;
        continue;
      }
      const rule = this.rule(nonTerminal);
      log.debug(`  ${nonTerminal}...`);
      const firstSet = this.computeFirstSet_(n, rule, cache);
      if (firstSet !== undefined) {
        if (cache.firstSetTable[nonTerminal] !== undefined) {
          complete = cache.firstSetTable[nonTerminal].equals(firstSet);
          if (!complete) {
            numChanged++;
          }
        } else {
          numChanged++;
        }
        cache.firstSetTable[nonTerminal] = firstSet;
      }
      if (!complete) {
        remaining.push(nonTerminal);
      }
    }
    this.firstSetTable_ = cache.firstSetTable;
  }

  buildLr0ItemSets() {
    const startItemId = this.createAugmentedStartItem_(0);  // LR(0)
    const startStateId = this.createState_(this.computeClosure_(startItemId));
    const remaining = [startStateId];
    const processed = new Set();
    while (remaining.length > 0) {
      const numRemaining = remaining.length;
      const stateId = remaining.shift();
      if (processed.has(stateId)) {
        continue;
      }
      processed.add(stateId);
      const state = this.state(stateId);
      log.info(`Processing ${state.toString()} (Remaining: ${numRemaining})...`);
      const nextKernelTable = new Map();  // symbol -> [item.id]
      for (const itemId of state.itemIds) {
        const item = this.item(itemId);
        const nextSymbol = item.nextSymbol;
        if (nextSymbol === null) {
          continue;
        }
        let symbol;
        switch (nextSymbol.type) {
        case 'empty':
        case 'lookahead':
          continue;
        case 'token':
          symbol = nextSymbol.data;
          break;
        case 'non-terminal':
          symbol = nextSymbol.data;
          break;
        case 'no-line-terminator':
          continue;  // TODO
        default:
          log.error(`unexpected type: ${nextSymbol.type}`);
          unreachable();
        }
        if (nextKernelTable.has(symbol)) {
          nextKernelTable.get(symbol).push(this.shift(itemId));
        } else {
          nextKernelTable.set(symbol, [this.shift(itemId)]);
        }
      }
      for (const [symbol, itemIds] of nextKernelTable) {
        const nextId = this.createState_(this.computeClosure_(itemIds));
        log.debug(`${state.toString()}: ${symbol} -> ${this.state(nextId).toString()}`);
        state.addTransition(symbol, nextId);
        if (!processed.has(nextId)) {
          remaining.push(nextId);
        }
      }
    }
  }

  buildLookaheadTable() {
    this.state(0).lookaheadTable.set(0, new TokenSeqSet(new TokenSeq('$')));

    let count = 0;
    for (;;) {
      log.info(`buildLookaheadTable: Iteration#${count}...`);
      let changed = false;
      for (const state of this.states_) {
        for (const itemId of state.itemIds) {
          const item = this.item(itemId);
          if (!item.isKernel) {
            continue;
          }
          if (item.isReducible) {
            continue;
          }
          const testItemId = this.createLrItem_(item.symbol, item.seq, item.dot, new TokenSeq('#'));
          const testItemSet = this.computeClosure_(testItemId);
          for (const id of testItemSet) {
            const testItem = this.item(id);
            if (testItem.isEmpty) {
              const kernelId = this.createLrItem_(testItem.symbol, testItem.seq, testItem.dot);
              assert(state.itemSet.has(kernelId));
              const kernelItem = this.item(kernelId);
              if (testItem.lookahead.repr === '#') {
                if (state.lookaheadTable.has(itemId)) {
                  log.debug(`buildLookaheadTable#${count}: ${state.toString()}: ${item.repr}: ${testItem.repr}: progagate ${state.lookaheadTable.get(itemId).repr} to ${kernelItem.repr} in ${state.toString()}`);
                  if (state.lookaheadTable.has(kernelId)) {
                    const old = state.lookaheadTable.get(kernelId);
                    state.lookaheadTable.set(kernelId, old.merge(state.lookaheadTable.get(itemId)));
                    if (!old.equals(state.lookaheadTable.get(kernelId))) {
                      changed = true;
                    }
                  } else {
                    state.lookaheadTable.set(kernelId, state.lookaheadTable.get(itemId).clone());
                    changed = true;
                  }
                }
              } else {
                log.debug(`buildLookaheadTable#${count}: ${state.toString()}: ${item.repr}: ${testItem.repr}: generate ${testItem.lookahead.repr} to ${kernelItem.repr} in ${state.toString()}`);
                if (state.lookaheadTable.has(kernelId)) {
                  const old = state.lookaheadTable.get(kernelId);
                  state.lookaheadTable.set(kernelId, old.merge(new TokenSeqSet(testItem.lookahead)));
                  if (!old.equals(state.lookaheadTable.get(kernelId))) {
                    changed = true;
                  }
                } else {
                  state.lookaheadTable.set(kernelId, new TokenSeqSet(testItem.lookahead));
                  changed = true;
                }
              }
              continue;
            }  // testItem.isEmpty

            const nextSymbol = testItem.nextSymbol;
            if (nextSymbol === null) {
              continue;
            }
            if (nextSymbol.type === 'empty') {
              continue;
            }
            if (nextSymbol.type === 'lookahead') {
              continue;
            }
            if (nextSymbol.type === 'no-line-terminator') {
              continue;  // TODO
            }
            let symbol = nextSymbol.data;
            const kernelId = this.shift(this.createLrItem_(
              testItem.symbol, testItem.seq, testItem.dot));
            const kernelItem = this.item(kernelId);
            const nextId = state.transitions[symbol];
            assert(nextId !== undefined);
            const nextState = this.state(nextId);
            assert(nextState.itemSet.has(kernelId));
            if (testItem.lookahead.repr === '#') {
              if (state.lookaheadTable.has(itemId)) {
                const lookaheadSet = state.lookaheadTable.get(itemId);
                log.debug(`buildLookaheadTable#${count}: ${state.toString()}: ${item.repr}: ${testItem.repr}: progagate ${lookaheadSet.repr} to ${kernelItem.repr} in ${nextState.toString()}`);
                if (nextState.lookaheadTable.has(kernelId)) {
                  const old = nextState.lookaheadTable.get(kernelId);
                  nextState.lookaheadTable.set(kernelId, old.merge(lookaheadSet));
                  if (!old.equals(nextState.lookaheadTable.get(kernelId))) {
                    changed = true;
                  }
                } else {
                  nextState.lookaheadTable.set(kernelId, lookaheadSet.clone());
                  changed = true;
                }
              }
            } else {
              log.debug(`buildLookaheadTable#${count}: ${state.toString()}: ${item.repr}: ${testItem.repr}: generate ${testItem.lookahead.repr} to ${kernelItem.repr} in ${nextState.toString()}`);
              if (nextState.lookaheadTable.has(kernelId)) {
                const old = nextState.lookaheadTable.get(kernelId);
                nextState.lookaheadTable.set(kernelId, old.merge(new TokenSeqSet(testItem.lookahead)));
                if (!old.equals(nextState.lookaheadTable.get(kernelId))) {
                  changed = true;
                }
              } else {
                nextState.lookaheadTable.set(kernelId, new TokenSeqSet(testItem.lookahead));
                changed = true;
              }
            }
          }
        }
      }
      if (!changed) {
        break;
      }
      count += 1;
    }
  }

  buildLalrTables() {
    log.info('Building LALR tables...');
    for (const state of this.states_) {
      for (const [symbol, nextId] of Object.entries(state.transitions)) {
        if (symbol in this.grammar_) {
          // non-terminal
          log.debug(`buildLalrTables: ${state.toString()}: ${symbol}: goto ${nextId}`);
          assertEquals(state.gotoTable[symbol], undefined);
          state.gotoTable[symbol] = nextId;
        } else {
          // token
          log.debug(`buildLalrTables: ${state.toString()}: ${symbol}: shift ${nextId}`);
          assertEquals(state.actionTable[symbol], undefined);
          state.actionTable[symbol] = {
            type: 'shift',
            nextId,
          };
        }
      }
      for (const itemId of state.itemIds) {
        const item = this.item(itemId);
        if (item.isReducible) {
          const lookaheadSet = state.lookaheadTable.get(itemId);
          assertExists(lookaheadSet, `No entry exists for ${item.repr} in lookahead table`);
          for (const lookahead of lookaheadSet.values()) {
            log.debug(`buildLalrTables: ${state.toString()}: ${lookahead.repr}: reduce ${item.repr}`);
            assertEquals(lookahead.length, 1);
            const token = lookahead.tokens[0];
            if (token in state.actionTable) {
              switch (state.actionTable[token].type) {
              case 'shift':
                log.error(`buildLalrTables: ${state.toString()}: ${token}: shift/reduce conflict`);
                log.error(`  ${reprProduction(item.symbol, item.seq)}`);
                break;
              case 'reduce':
                log.error(`buildLalrTables: ${state.toString()}: ${token}: reduce/reduce conflict`);
                log.error(`  ${reprProduction(state.actionTable[token].symbol, state.actionTable[token].production)}`);
                log.error(`  ${reprProduction(item.symbol, item.seq)}`);
                break;
              }
              unreachable();
            }
            state.actionTable[token] = {
              type: 'reduce',
              symbol: item.symbol,
              production: item.seq,
            };
          }
        }
      }
    }
  }

  item(id) {
    return this.items_[id];
  }

  state(id) {
    return this.states_[id];
  }

  itemFromRepr(repr) {
    return this.itemMap_.get(repr);
  }

  stateFromRepr(repr) {
    return this.stateMap_.get(repr);
  }

  rule(nonTerminal) {
    assertExists(this.grammar_[nonTerminal], `${nonTerminal} must exist`);
    return this.grammar_[nonTerminal];
  }

  shift(id) {
    const item = this.item(id);
    assert(item.nextSymbol !== null);
    return this.createLrItem_(
      item.symbol, item.seq, item.dot + 1, item.lookahead);
  }

  createAugmentedStartItem_(k) {
    // '^' is the goal symbol of the augmented grammar.
    // '$' is the symbol representing the end of token stream.
    const lookahead = [];
    for (let i = 0; i < k; ++i) {
      lookahead.push('$');
    }
    return this.createLrItem_('^', this.initialProduction_, 0, new TokenSeq(...lookahead));
  }

  createLrItem_(symbol, seq, dot, lookahead) {
    const id = this.items_.length;
    const item = new LrItem(id, symbol, seq, dot, lookahead);
    const repr = item.repr
    if (this.itemMap_.has(repr)) {
      return this.itemMap_.get(repr);
    }
    log.debug(`Created item#${id}: ${repr}`);
    this.items_.push(item);
    this.itemMap_.set(repr, id);
    return id;
  }

  createState_(itemSet) {
    const id = this.states_.length;
    const state = new State(id, itemSet);
    const repr = state.repr;
    if (this.stateMap_.has(repr)) {
      return this.stateMap_.get(repr);
    }
    log.debug(`Created state#${id}: ${repr}`);
    this.states_.push(state);
    this.stateMap_.set(repr, id);
    return id;
  }

  // nullable

  buildNullableTable() {
    log.debug('Building nullable table...');
    this.nullableTable_ = {};
    const remaining = Object.keys(this.grammar_);
    while (remaining.length > 0) {
      const nonTerminal = remaining.shift();
      const rule = this.rule(nonTerminal);
      const nullable = this.isNullable_(rule);
      log.debug(`  ${nonTerminal}: ${nullable}`);
      if (nullable === undefined) {
        remaining.push(nonTerminal);
      } else {
        this.nullableTable_[nonTerminal] = nullable;
      }
    }
  }

  isNullable_(rule) {
    switch (rule.type) {
    case 'empty':
      return true;
    case 'token':
    case 'no-line-terminator':
      return false;
    case 'non-terminal':
      return this.isNullableNonTerminal_(rule);
    case 'sequence':
      return this.isNullableSequence_(rule);
    case 'one-of':
      return this.hasNullable_(rule);
    default:
      log.error(`unexpected type: ${rule.type}`);
      unreachable();
    }
  }

  isNullableNonTerminal_(rule) {
    assertExists(this.rule(rule.data));
    if (rule.prefix?.includes) {
      return false;
    }
    return this.nullableTable_[rule.data];
  }

  isNullableSequence_(rule) {
    const nullables = rule.data.map((symbol) => this.isNullable_(symbol));
    if (nullables.some((nullable) => nullable === false)) {
      return false;
    }
    if (nullables.every((nullable) => nullable === true)) {
      return true;
    }
    return undefined;
  }

  hasNullable_(rule) {
    if (rule.data.some((prod) => this.isNullable_(prod) === true)) {
      return true;
    }
    if (rule.data.every((prod) => this.isNullable_(prod) === false)) {
      return false;
    }
    return undefined;
  }

  // first set

  computeFirstSet_(n, rule, cache, indent = 2) {
    let firstSet;
    switch (rule.type) {
    case 'empty':
      log.debug(`${''.padStart(indent * 2, ' ')}empty`);
      return new TokenSeqSet(new TokenSeq());
    case 'token':
      log.debug(`${''.padStart(indent * 2, ' ')}token: ${rule.data}`);
      return new TokenSeqSet(new TokenSeq(rule.data));
    case 'no-line-terminator':
      log.debug(`${''.padStart(indent * 2, ' ')}no-line-terminator`);
      return new TokenSeqSet(new TokenSeq());  // TODO
    case 'non-terminal':
      log.debug(`${''.padStart(indent * 2, ' ')}non-terminal: ${rule.data}`);
      assertExists(this.grammar_[rule.data]);
      firstSet = cache.firstSetTable[rule.data];
      log.debug(`${''.padStart(indent * 2, ' ')}  -> ${firstSet?.repr}`);
      return firstSet;
    case 'sequence':
      log.debug(`${''.padStart(indent * 2, ' ')}sequence`);
      const results = rule.data.map((sym) => {
        if (sym.type === 'lookahead') {
          log.debug(`${''.padStart((indent + 1) * 2, ' ')}lookahead`);
          return sym;
        }
        return this.computeFirstSet_(n, sym, cache, indent + 1);
      });
      let lookahead;
      let pendingSet;
      for (const result of results) {
        if (result?.type === 'lookahead') {
          firstSet = pendingSet;
          pendingSet = undefined;
          lookahead = result;
          continue;
        }
        if (result === undefined) {
          pendingSet = undefined;
          break;
        }
        if (pendingSet === undefined) {
          pendingSet = result;
          continue;
        }
        pendingSet = pendingSet.concat(result);
        if (pendingSet.minLength > 2) {
          break;
        }
      }
      if (lookahead !== undefined) {
        pendingSet = pendingSet?.filter((seq) => {
          if (rule.prefix === undefined) {
            return true;
          }
          return this.checkPrefixCondition_(lookahead, seq.repr);
        });
        if (pendingSet) {
          if (firstSet) {
            firstSet = firstSet.merge(pendingSet);
          } else {
            firstSet = pendingSet;
          }
        }
      } else {
        firstSet = pendingSet;
      }
      firstSet = firstSet?.shrink(n);
      log.debug(`${''.padStart(indent * 2, ' ')}  -> ${firstSet?.repr}`);
      return firstSet;
    case 'one-of':
      log.debug(`${''.padStart(indent * 2, ' ')}one-of`);
      firstSet = rule.data.map((prod) => {
        return this.computeFirstSet_(n, prod, cache, indent + 1);
      }).reduce((result, firstSet) => {
        if (firstSet === undefined) {
          return result;
        }
        if (result === undefined) {
          return firstSet;
        }
        return result.merge(firstSet);
      }, undefined);
      log.debug(`${''.padStart(indent * 2, ' ')}  -> ${firstSet?.repr}`);
      return firstSet;
    default:
      log.error(`unexpected type: ${rule.type}`);
      unreachable();
    }
  }

  // closure

  computeClosure_(args, recursion = []) {
    if (typeof args !== 'number') {
      return Array
        .from(args)
        .sort((a, b) => a - b)
        .map((itemId) => this.computeClosure_(itemId, recursion))
        .reduce((itemSet, closure) => mergeSet(itemSet, closure), new Set());
    }

    const itemId = args
    const item = this.item(itemId);
    log.debug(`Computing closure of ${item.toString()}...`);

    if (this.closureCache_.has(itemId)) {
      return this.closureCache_.get(itemId);
    }

    let itemSet = new Set([itemId]);

    if (recursion.includes(itemId)) {
      // Recursion detected.
      return itemSet;
    }

    recursion = recursion.concat([itemId]);

    const [nextSymbol, ...followerSymbols] = item.followerSymbols;
    if (nextSymbol === undefined || nextSymbol.type === 'token') {
      this.closureCache_.set(itemId, itemSet);
      return itemSet;
    }

    if (nextSymbol.type === 'empty') {
      const shiftedId = this.shift(itemId);
      itemSet = mergeSet(itemSet, this.computeClosure_(shiftedId, recursion));
      this.closureCache_.set(itemId, itemSet);
      return itemSet;
    }

    if (nextSymbol.type === 'lookahead') {
      const shiftedId = this.shift(itemId);
      itemSet = mergeSet(itemSet, this.computeClosure_(shiftedId, recursion));
      this.closureCache_.set(itemId, itemSet);
      return itemSet;
    }

    if (nextSymbol.type === 'no-line-terminator') {
      const shiftedId = this.shift(itemId);
      itemSet = mergeSet(itemSet, this.computeClosure_(shiftedId, recursion));
      this.closureCache_.set(itemId, itemSet);
      return itemSet;
    }

    const symbol = nextSymbol.data;
    const rule = this.rule(symbol);
    let lookaheadSet = new TokenSeqSet(new TokenSeq);
    if (item.k > 0) {
      lookaheadSet = this.computeFirstSetOfFollowerSymbols_(followerSymbols).shrink(item.k);
    }
    const nonKernelItemSet = this.computeNonKernelItemSetFromRule_(symbol, rule, lookaheadSet);
    itemSet = mergeSet(itemSet, this.computeClosure_(nonKernelItemSet, recursion));

    if (item.condition !== undefined) {
      const condition = item.condition;
      itemSet = filterSet(itemSet, (itemId) => {
        const item = this.item(itemId);
        function symbolsToRepr(symbols) {
          assert(Array.isArray(symbols));
          return symbols
            .filter((sym) => sym.type !== 'no-line-terminator')  // TODO
            .map((sym) => sym.data)
            .join(' ');
        }
        const repr = symbolsToRepr(item.seq);
        return this.checkPrefixCondition_(condition, repr);
      });
    }

    this.closureCache_.set(itemId, itemSet);
    return itemSet;
  }

  computeClosure2_(itemIds) {
    log.debug(`Computing closure of { ${itemIds.map((id) => this.item(id).repr).join(', ')} }...`);
    const itemSet = new Set(itemIds);
    let remaining = Array.from(itemIds).sort((a, b) => a - b);
    const repr = remaining.join(',');
    if (repr in this.closureCache2_) {
      return this.closureCache2_[repr];
    }
    while (remaining.length > 0) {
      const id = remaining.shift();
      const item = this.item(id);
      const [nextSymbol, ...followerSymbols] = item.followerSymbols;
      if (nextSymbol === undefined || nextSymbol.type === 'token') {
        continue;
      }
      if (nextSymbol.type === 'no-line-terminator') {
        const shiftedId = this.shift(id);
        itemSet.add(shiftedId);
        remaining.push(shiftedId);
        continue;
      }
      let nonKernelItemSet = this.nonKernelItemSetCache_.get(id);
      if (nonKernelItemSet === undefined) {
        const symbol = nextSymbol.data;
        const rule = this.rule(symbol);
        let lookaheadSet = new TokenSeqSet(new TokenSeq);
        if (item.k > 0) {
          lookaheadSet = this.computeFirstSetOfFollowerSymbols_(followerSymbols).shrink(item.k);
        }
        nonKernelItemSet = this.computeNonKernelItemSetFromRule_(symbol, rule, lookaheadSet);
        this.nonKernelItemSetCache_.set(id, nonKernelItemSet);
      }
      for (const nonKernelItemId of nonKernelItemSet) {
        if (itemSet.has(nonKernelItemId)) {
          continue;
        }
        itemSet.add(nonKernelItemId);
        remaining.push(nonKernelItemId);
      }
    }
    this.closureCache2_[repr] = itemSet;
    return itemSet;
  }

  computeNonKernelItemSetFromRule_(symbol, rule, lookaheadSet) {
    if (rule.type === 'one-of') {
      return rule.data.reduce((itemSet, production) => {
        const set = this.computeNonKernelItemSetFromProduction_(
          symbol, production, lookaheadSet);
        for (const id of set) {
          itemSet.add(id);
        }
        return itemSet;
      }, new Set());
    }
    return this.computeNonKernelItemSetFromProduction_(symbol, rule, lookaheadSet);
  }

  computeNonKernelItemSetFromProduction_(symbol, production, lookaheadSet) {
    const itemIds = lookaheadSet.values().map((lookahead) => {
      let itemId;
      switch (production.type) {
      case 'empty':
        return this.createLrItem_(symbol, [], 0, lookahead);
      case 'token':
      case 'non-terminal':
        return this.createLrItem_(symbol, [production], 0, lookahead);
      case 'sequence':
        return this.createLrItem_(symbol, production.data, 0, lookahead);
      default:
        log.error(`${symbol}: unexpected type: ${production.type}`);
        unreachable();
      }
    });
    return new Set(itemIds);
  }

  computeFirstSetOfFollowerSymbols_(symbols) {
    return symbols
      .map((symbol) => {
        switch (symbol.type) {
        case 'empty':
          return new TokenSeqSet(new TokenSeq());
        case 'token':
          return new TokenSeqSet(new TokenSeq(symbol.data));
        case 'no-line-terminator':
          return new TokenSeqSet(new TokenSeq());  // TODO
        case 'non-terminal':
          assertExists(this.grammar_[symbol.data]);
          return this.firstSetTable_[symbol.data];
        case 'lookahead':
          return new TokenSeqSet(new TokenSeq());
        default:
          log.error(`unexpected type: ${symbol.type}`);
          unreachable();
        }
      })
      .reduce((result, firstSet) => {
        const x = result.concat(firstSet, this.k_);
        return x;
      }, new TokenSeqSet(new TokenSeq()));
  }

  checkPrefixCondition_(lookahead, repr) {
    function symbolsToRepr(symbols) {
      assert(Array.isArray(symbols));
      return symbols
        .filter((sym) => sym.type !== 'no-line-terminator')  // TODO
        .map((sym) => sym.data)
        .join(' ');
    }

    if (lookahead.negate) {
      return lookahead.data.every((cond) => {
        switch (cond.type) {
        case 'token':
          return !repr.startsWith(cond.data);
        case 'sequence':
          return !repr.startsWith(symbolsToRepr(cond.data));
        default:
          log.error(`not implemented: ${cond.type}`);
          Deno.exit(1);
        }
      });
    } else {
      return lookahead.data.some((cond) => {
        switch (cond.type) {
        case 'token':
          return repr.startsWith(cond.data);
        case 'sequence':
          return repr.startsWith(symbolsToRepr(cond.data));
        default:
          log.error(`not implemented: ${cond.type}`);
          Deno.exit(1);
        }
      });
    }
  }

  checkSuffixCondition_(condition, symbols) {
    assert(symbols.every((sym) => sym.type === 'token'));
    if (condition.includes) {
      return condition.includes.some((cond) => {
        switch (cond.type) {
        case 'token':
          return cond.data === symbols[0].data;
        default:
          log.error(`not implemented: ${cond.type}`);
          Deno.exit(1);
        }
      });
    } else {
      assertExists(condition.excludes);
      return condition.excludes.every((cond) => {
        switch (cond.type) {
        case 'token':
          return cond.data !== symbols[0].data;
        default:
          log.error(`not implemented: ${cond.type}`);
          Deno.exit(1);
        }
      });
    }
  }
}

// The State class holds an LR item set and provides helper functions used for generating LALR
// tables.
class State {
  constructor(id, itemSet) {
    this.id = id;
    this.itemSet = itemSet;
    this.transitions = {};  // symbol -> state.id
    this.lookaheadTable = new Map();  // item.id -> TokenSeqSet
    this.actionTable = {};  // terminal -> action
    this.gotoTable = {};  // non-terminal -> state.id
  }

  get repr() {
    const sorted = Array.from(this.itemSet).sort((a, b) => a - b);
    return `[${sorted.join(',')}]`;
  }

  get itemIds() {
    return this.itemSet.values();
  }

  addTransition(symbol, stateId) {
    assertEquals(this.transitions[symbol], undefined);
    this.transitions[symbol] = stateId;
  }

  toString() {
    return `State#${this.id}`;
  }
}

function reprProduction(symbol, seq) {
  seq = seq.map((sym) => {
    if (sym.type === 'no-line-terminator') {
      return '+';
    }
    return sym.data;
  });

  return `${symbol} -> ${seq.join(' ')}`;
}

class LrItem {
  constructor(id, symbol, seq, dot, lookahead = new TokenSeq) {
    assert(dot >= 0);
    assert(dot <= seq.length);
    assertInstanceOf(seq, Array);
    this.id = id;
    this.symbol = symbol;
    this.seq = seq;
    this.dot = dot;
    this.lookahead = lookahead;
  }

  get k() {
    return this.lookahead.length;
  }

  get isEmpty() {
    return this.seq.length === 0 || (this.seq.length === 1 && this.seq[0].type === 'empty');
  }

  get isKernel() {
    return this.symbol === '^' || this.dot > 0;
  }

  get isReducible() {
    return this.nextSymbol === null;
  }

  get condition() {
    if (this.isEmpty) {
      return undefined;
    }
    if (this.dot === 0) {
      return undefined;
    }
    if (this.seq[this.dot - 1].type === 'lookahead') {
      return this.seq[this.dot - 1];
    }
    return undefined;
  }

  get repr() {
    const seq = this.seq.map((sym) => {
      if (sym.type === 'empty') {
        return '()';
      }
      if (sym.type === 'lookahead') {
        return '/';
      }
      if (sym.type === 'no-line-terminator') {
        return '+';
      }
      return sym.data;
    });
    seq.splice(this.dot, 0, '*');
    let s = `${this.symbol} -> ${seq.join(' ')}`;  // LR(0)
    if (!this.lookahead.isEmpty) {
      s = `${s}, ${this.lookahead.repr}`;  // LR(k)
    }
    return `[${s}]`;
  }

  get nextSymbol() {
    if (this.dot === this.seq.length) {
      return null;
    }
    return this.seq[this.dot];
  }

  get followerSymbols() {
    const syms = this.seq.slice(this.dot);
    for (const token of this.lookahead.tokens) {
      syms.push({ type: 'token', data: token });
    }
    return syms;
  }

  toString() {
    return `${this.repr}@${this.id}`
  }
}

class TokenSeq {
  constructor(...tokens) {
    assert(Array.isArray(tokens));
    assert(tokens.every((token) => typeof token === 'string'));
    this.tokens_ = tokens;
  }

  get isEmpty() {
    return this.tokens_.length === 0;
  }

  get tokens() {
    return this.tokens_;
  }

  get length() {
    return this.tokens_.length;
  }

  get repr() {
    if (this.tokens_.length === 0) {
      return '_';
    }
    return this.tokens_.join(' ');
  }

  concat(other) {
    if (other instanceof TokenSeq) {
      return new TokenSeq(...this.tokens_, ...other.tokens_);
    }
    return new TokenSeq(...this.tokens_, other);
  }

  shrink(n) {
    if (n === 0) {
      return new TokenSeq();
    }
    if (this.length <= n) {
      return this;
    }
    return new TokenSeq(...this.tokens_.slice(0, n));
  }
}

class TokenSeqSet {
  constructor(...values) {
    this.map_ = new Map();
    this.addValues_(values);
  }

  get length() {
    return this.map_.size;
  }

  get minLength() {
    if (this.map_.size === 0) {
      return 0;
    }
    return this
      .values()
      .map((seq) => seq.tokens.length)
      .reduce((min, n) => Math.min(min, n), Number.MAX_SAFE_INTEGER);
  }

  get repr() {
    return `{${Array.from(this.map_.keys()).sort().join(', ')}}`;
  }

  values() {
    return this.map_.values();
  }

  equals(other) {
    if (this.map_.size !== other.map_.size) {
      return false;
    }
    for (const repr of this.map_.keys()) {
      if (!other.map_.has(repr)) {
        return false;
      }
    }
    return true;
  }

  add(value) {
    assertInstanceOf(value, TokenSeq);
    const repr = value.repr;
    if (this.map_.has(repr)) {
      return;
    }
    this.map_.set(repr, value);
  }

  values() {
    return Array.from(this.map_.values());
  }

  clone() {
    const cloned = new TokenSeqSet();
    cloned.addValues_(this.values());
    return cloned;
  }

  concat(other, n) {
    const result = new TokenSeqSet();
    for (const a of this.values()) {
      for (const b of other.values()) {
        result.add(a.concat(b).shrink(n));
      }
    }
    return result;
  }

  merge(other) {
    const merged = new TokenSeqSet();
    merged.addValues_(this.values());
    merged.addValues_(other.values());
    return merged;
  }

  shrink(n) {
    const shrinked = new TokenSeqSet();
    for (const value of this.values()) {
      shrinked.add(value.shrink(n));
    }
    return shrinked;
  }

  filter(filter) {
    const filtered = new TokenSeqSet();
    for (const value of this.values()) {
      if (filter(value)) {
        filtered.add(value);
      }
    }
    return filtered;
  }

  addValues_(values) {
    for (const value of values) {
      this.add(value);
    }
  }
}

// helper functions for Set

function mergeSet(a, b) {
  return new Set(Array.from(a).concat(Array.from(b)));
}

function filterSet(set, fn) {
  return new Set(Array.from(set).filter(fn));
}

// entry point

if (import.meta.main) {
  const { options, args } = await parseCommand({
    doc: DOC,
  });
  Deno.exit(await run(options, args));
}

// tests

import {
  assertThrows,
} from 'https://deno.land/std@0.193.0/testing/asserts.ts';
import {
  beforeEach,
  describe,
  it,
} from 'https://deno.land/std@0.193.0/testing/bdd.ts';
import { dedent } from 'npm:ts-dedent@2.2.0';

setup(PROGNAME, 'DEBUG');

describe('LalrBuilder', () => {
  let builder;

  beforeEach(() => {
    builder = new LalrBuilder('S', yaml.parse(dedent`
      # Took from the example 4.34 (and 4.45) in the Dragon Book (2nd).
      S:
        type: one-of
        data:
          - type: sequence
            data:
              - type: non-terminal
                data: L
              - type: token
                data: EQ
              - type: non-terminal
                data: R
          - type: non-terminal
            data: R
      L:
        type: one-of
        data:
          - type: sequence
            data:
              - type: token
                data: MUL
              - type: non-terminal
                data: R
          - type: token
            data: ID
      R:
        type: non-terminal
        data: L
    `));
    //builder.buildNullableTable();
    builder.buildFirstSetTable();
  });

  it('rule', () => {
    assertExists(builder.rule('S'));
    assertThrows(() => builder.rule('X'));
  });

  it('buildLr0ItemSets', () => {
    builder.buildLr0ItemSets();
    assertEquals(builder.states_.length, 10);
  });

  it('buildLookaheadTable', () => {
    builder.buildLr0ItemSets();
    builder.buildLookaheadTable();
    {
      // I0
      const itemId = builder.itemFromRepr('[^ -> * S]');
      const stateId = builder.createState_(builder.computeClosure_([itemId]));
      const state = builder.state(stateId);
      assertEquals(state.lookaheadTable.size, 1);
      assert(state.lookaheadTable.has(itemId));
      assertEquals(state.lookaheadTable.get(itemId).repr, '{$}');
    }
    {
      // I1
      const itemId = builder.itemFromRepr('[^ -> S *]');
      const stateId = builder.createState_(builder.computeClosure_([itemId]));
      const state = builder.state(stateId);
      assertEquals(state.lookaheadTable.size, 1);
      assert(state.lookaheadTable.has(itemId));
      assertEquals(state.lookaheadTable.get(itemId).repr, '{$}');
    }
    {
      // I2
      const itemId1 = builder.itemFromRepr('[S -> L * EQ R]');
      const itemId2 = builder.itemFromRepr('[R -> L *]');
      const stateId = builder.createState_(builder.computeClosure_([itemId1, itemId2]));
      const state = builder.state(stateId);
      assertEquals(state.lookaheadTable.size, 2);
      assert(state.lookaheadTable.has(itemId1));
      assertEquals(state.lookaheadTable.get(itemId1).repr, '{$}');
      assert(state.lookaheadTable.has(itemId2));
      assertEquals(state.lookaheadTable.get(itemId2).repr, '{$}');
    }
    {
      // I3
      const itemId = builder.itemFromRepr('[S -> R *]');
      const stateId = builder.createState_(builder.computeClosure_([itemId]));
      const state = builder.state(stateId);
      assertEquals(state.lookaheadTable.size, 1);
      assert(state.lookaheadTable.has(itemId));
      assertEquals(state.lookaheadTable.get(itemId).repr, '{$}');
    }
    {
      // I4
      const itemId = builder.itemFromRepr('[L -> MUL * R]');
      const stateId = builder.createState_(builder.computeClosure_([itemId]));
      const state = builder.state(stateId);
      assertEquals(state.lookaheadTable.size, 1);
      assert(state.lookaheadTable.has(itemId));
      assertEquals(state.lookaheadTable.get(itemId).repr, '{$, EQ}');
    }
    {
      // I5
      const itemId = builder.itemFromRepr('[L -> ID *]');
      const stateId = builder.createState_(builder.computeClosure_([itemId]));
      const state = builder.state(stateId);
      assertEquals(state.lookaheadTable.size, 1);
      assert(state.lookaheadTable.has(itemId));
      assertEquals(state.lookaheadTable.get(itemId).repr, '{$, EQ}');
    }
    {
      // I6
      const itemId = builder.itemFromRepr('[S -> L EQ * R]');
      const stateId = builder.createState_(builder.computeClosure_([itemId]));
      const state = builder.state(stateId);
      assertEquals(state.lookaheadTable.size, 1);
      assert(state.lookaheadTable.has(itemId));
      assertEquals(state.lookaheadTable.get(itemId).repr, '{$}');
    }
    {
      // I7
      const itemId = builder.itemFromRepr('[L -> MUL R *]');
      const stateId = builder.createState_(builder.computeClosure_([itemId]));
      const state = builder.state(stateId);
      assertEquals(state.lookaheadTable.size, 1);
      assert(state.lookaheadTable.has(itemId));
      assertEquals(state.lookaheadTable.get(itemId).repr, '{$, EQ}');
    }
    {
      // I8
      const itemId = builder.itemFromRepr('[R -> L *]');
      const stateId = builder.createState_(builder.computeClosure_([itemId]));
      const state = builder.state(stateId);
      assertEquals(state.lookaheadTable.size, 1);
      assert(state.lookaheadTable.has(itemId));
      assertEquals(state.lookaheadTable.get(itemId).repr, '{$, EQ}');
    }
    {
      // I9
      const itemId = builder.itemFromRepr('[S -> L EQ R *]');
      const stateId = builder.createState_(builder.computeClosure_([itemId]));
      const state = builder.state(stateId);
      assertEquals(state.lookaheadTable.size, 1);
      assert(state.lookaheadTable.has(itemId));
      assertEquals(state.lookaheadTable.get(itemId).repr, '{$}');
    }
  });

  it('createLrItem_', () => {
    const rule = builder.rule('S');
    const seq = rule.data;

    const id = builder.createLrItem_('S', seq, 0);
    assertEquals(id, 0);
    assertExists(builder.item(id));

    const alreadyExists = builder.createLrItem_('S', seq, 0);
    assertEquals(alreadyExists, id);
  });

  it('computeClosure_', () => {
    builder = new LalrBuilder('S', yaml.parse(dedent`
      # Took from (4.16) in the Dragon Book (2nd).
      S:
        type: sequence
        data:
          - type: non-terminal
            data: C
          - type: non-terminal
            data: C
      C:
        type: one-of
        data:
          - type: sequence
            data:
              - type: token
                data: c
              - type: non-terminal
                data: C
          - type: token
            data: d
    `));
    builder.buildFirstSetTable();

    // I0
    const i0 = [
      builder.createAugmentedStartItem_(1),  // LR(1)
    ];
    {
      const itemSet = builder.computeClosure_(i0);
      assertEquals(itemSet.size, 6);
      // kernel items
      assert(itemSet.has(builder.itemFromRepr('[^ -> * S, $]')));
      // non-kernel items
      assert(itemSet.has(builder.itemFromRepr('[S -> * C C, $]')));
      assert(itemSet.has(builder.itemFromRepr('[C -> * c C, c]')));
      assert(itemSet.has(builder.itemFromRepr('[C -> * c C, d]')));
      assert(itemSet.has(builder.itemFromRepr('[C -> * d, c]')));
      assert(itemSet.has(builder.itemFromRepr('[C -> * d, d]')));
    }

    // I1
    const i1 = [
      builder.shift(i0),
    ];
    {
      const itemSet = builder.computeClosure_(i1);
      assertEquals(itemSet.size, 1);
      // kernel items
      assert(itemSet.has(builder.itemFromRepr('[^ -> S *, $]')));
    }

    // I2
    const i2 = [
      builder.shift(builder.itemFromRepr('[S -> * C C, $]')),
    ];
    {
      const itemSet = builder.computeClosure_(i2);
      assertEquals(itemSet.size, 3);
      // kernel items
      assert(itemSet.has(builder.itemFromRepr('[S -> C * C, $]')));
      // non-kernel items
      assert(itemSet.has(builder.itemFromRepr('[C -> * c C, $]')));
      assert(itemSet.has(builder.itemFromRepr('[C -> * d, $]')));
    }

    // I3
    const i3 = [
      builder.shift(builder.itemFromRepr('[C -> * c C, c]')),
      builder.shift(builder.itemFromRepr('[C -> * c C, d]')),
    ];
    {
      const itemSet = builder.computeClosure_(i3);
      assertEquals(itemSet.size, 6);
      // kernel items
      assert(itemSet.has(builder.itemFromRepr('[C -> c * C, c]')));
      assert(itemSet.has(builder.itemFromRepr('[C -> c * C, d]')));
      // non-kernel items
      assert(itemSet.has(builder.itemFromRepr('[C -> * c C, c]')));
      assert(itemSet.has(builder.itemFromRepr('[C -> * c C, d]')));
      assert(itemSet.has(builder.itemFromRepr('[C -> * d, c]')));
      assert(itemSet.has(builder.itemFromRepr('[C -> * d, d]')));
    }

    // I4
    const i4 = [
      builder.shift(builder.itemFromRepr('[C -> * d, c]')),
      builder.shift(builder.itemFromRepr('[C -> * d, d]')),
    ];
    {
      const itemSet = builder.computeClosure_(i4);
      assertEquals(itemSet.size, 2);
      // kernel items
      assert(itemSet.has(builder.itemFromRepr('[C -> d *, c]')));
      assert(itemSet.has(builder.itemFromRepr('[C -> d *, d]')));
    }

    // I5
    const i5 = [
      builder.shift(builder.itemFromRepr('[S -> C * C, $]')),
    ];
    {
      const itemSet = builder.computeClosure_(i5);
      assertEquals(itemSet.size, 1);
      // kernel items
      assert(itemSet.has(builder.itemFromRepr('[S -> C C *, $]')));
    }

    // I6
    const i6 = [
      builder.shift(builder.itemFromRepr('[C -> * c C, $]')),
    ];
    {
      const itemSet = builder.computeClosure_(i6);
      assertEquals(itemSet.size, 3);
      // kernel items
      assert(itemSet.has(builder.itemFromRepr('[C -> c * C, $]')));
      // kernel items
      assert(itemSet.has(builder.itemFromRepr('[C -> * c C, $]')));
      assert(itemSet.has(builder.itemFromRepr('[C -> * d, $]')));
    }

    // I7
    const i7 = [
      builder.shift(builder.itemFromRepr('[C -> * d, $]')),
    ];
    {
      const itemSet = builder.computeClosure_(i7);
      assertEquals(itemSet.size, 1);
      // kernel items
      assert(itemSet.has(builder.itemFromRepr('[C -> d *, $]')));
    }

    // I8
    const i8 = [
      builder.shift(builder.itemFromRepr('[C -> c * C, c]')),
      builder.shift(builder.itemFromRepr('[C -> c * C, d]')),
    ];
    {
      const itemSet = builder.computeClosure_(i8);
      assertEquals(itemSet.size, 2);
      // kernel items
      assert(itemSet.has(builder.itemFromRepr('[C -> c C *, c]')));
      assert(itemSet.has(builder.itemFromRepr('[C -> c C *, d]')));
    }

    // I9
    const i9 = [
      builder.shift(builder.itemFromRepr('[C -> c * C, $]')),
    ];
    {
      const itemSet = builder.computeClosure_(i9);
      assertEquals(itemSet.size, 1);
      // kernel items
      assert(itemSet.has(builder.itemFromRepr('[C -> c C *, $]')));
    }
  });
});

describe('LrItem', () => {
  it('isKernel', () => {
    const seq = [{ type: 'token', data: 'b' }];

    assert(!(new LrItem(0, 'A', seq, 0)).isKernel);
    assert((new LrItem(0, 'A', seq, 1)).isKernel);
    assert((new LrItem(0, '^', seq, 1)).isKernel);
  });

  it('repr', () => {
    const seq = [{ type: 'token', data: 'b' }];
    const lookahead = new TokenSeq('c');

    // LR(0) item
    assertEquals((new LrItem(0, 'A', seq, 0)).repr, '[A -> * b]');
    assertEquals((new LrItem(0, 'A', seq, 1)).repr, '[A -> b *]');

    // LR(1) item
    assertEquals((new LrItem(0, 'A', seq, 0, lookahead)).repr, '[A -> * b, c]');
    assertEquals((new LrItem(0, 'A', seq, 1, lookahead)).repr, '[A -> b *, c]');

    // no-line-terminator
    assertEquals(
      new LrItem(0, 'A', [
        { type: 'token', data: 'b' },
        { type: 'no-line-terminator' },
        { type: 'non-terminal', data: 'C' },
      ], 0).repr,
      '[A -> * b + C]');
  });

  it('followerSymbols', () => {
    const seq = [{type: 'token', data: 'b' }];
    const lookahead = new TokenSeq('c');

    // LR(0) item
    assertEquals((new LrItem(0, 'A', seq, 0)).followerSymbols.length, 1);
    assertEquals((new LrItem(0, 'A', seq, 0)).followerSymbols[0].data, 'b');
    assertEquals((new LrItem(0, 'A', seq, 1)).followerSymbols.length, 0);

    // LR(1) item
    assertEquals((new LrItem(0, 'A', seq, 0, lookahead)).followerSymbols.length, 2);
    assertEquals((new LrItem(0, 'A', seq, 0, lookahead)).followerSymbols[0].data, 'b');
    assertEquals((new LrItem(0, 'A', seq, 0, lookahead)).followerSymbols[1].data, 'c');
    assertEquals((new LrItem(0, 'A', seq, 1, lookahead)).followerSymbols.length, 1);
    assertEquals((new LrItem(0, 'A', seq, 1, lookahead)).followerSymbols[0].data, 'c');
  });
});

describe('TokenSeq', () => {
  it('constructor', () => {
    assertEquals((new TokenSeq).repr, '_');
    assertEquals((new TokenSeq('a')).repr, 'a');
    assertEquals((new TokenSeq('a', 'b')).repr, 'a b');
  });

  it('isEmpty', () => {
    assert((new TokenSeq).isEmpty);
    assert(!(new TokenSeq('a')).isEmpty);
  });
});

describe('TokenSeqSet', () => {
  it('minLength', () => {
    assertEquals((new TokenSeqSet).minLength, 0);
    assertEquals((new TokenSeqSet(new TokenSeq())).minLength, 0);
    assertEquals((new TokenSeqSet(new TokenSeq('a'))).minLength, 1);
    assertEquals((new TokenSeqSet(
      new TokenSeq('a', 'b'),
      new TokenSeq('c'),
    )).minLength, 1);
  });

  it('concat', () => {
    const empty = new TokenSeqSet();
    const epsilon = new TokenSeqSet(new TokenSeq());
    const a = new TokenSeqSet(new TokenSeq('a'));
    const ab = new TokenSeqSet(new TokenSeq('a'), new TokenSeq('b'));

    assertEquals(empty.concat(empty).length, 0);
    assertEquals(empty.concat(epsilon).length, 0);
    assertEquals(empty.concat(a).length, 0);
    assertEquals(empty.concat(ab).length, 0);

    assertEquals(epsilon.concat(empty).length, 0);
    assertEquals(epsilon.concat(epsilon).length, 1);
    assertEquals(epsilon.concat(epsilon).repr, '{_}');
    assertEquals(epsilon.concat(a).length, 1);
    assertEquals(epsilon.concat(a).repr, '{a}');
    assertEquals(epsilon.concat(ab).length, 2);
    assertEquals(epsilon.concat(ab).repr, '{a, b}');

    assertEquals(a.concat(empty).length, 0);
    assertEquals(a.concat(epsilon).length, 1);
    assertEquals(a.concat(epsilon).repr, '{a}');
    assertEquals(a.concat(a).length, 1);
    assertEquals(a.concat(a).repr, '{a a}');
    assertEquals(a.concat(ab).length, 2);
    assertEquals(a.concat(ab).repr, '{a a, a b}');

    assertEquals(ab.concat(empty).length, 0);
    assertEquals(ab.concat(epsilon).length, 2);
    assertEquals(ab.concat(epsilon).repr, '{a, b}');
    assertEquals(ab.concat(a).length, 2);
    assertEquals(ab.concat(a).repr, '{a a, b a}');
    assertEquals(ab.concat(ab).length, 4);
    assertEquals(ab.concat(ab).repr, '{a a, a b, b a, b b}');
  });
});
