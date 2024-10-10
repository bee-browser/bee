import * as io from '@std/io';

const decoder = new TextDecoder('utf-8');
const json = await decoder.decode(await io.readAll(Deno.stdin));
const entries = JSON.parse(json);

const nodes = [{
  buffer: '',
  next: Array(63).fill(0),
  chars: '',
  end: true,
}];

const DIGIT_0 = '0'.codePointAt(0);
const DIGIT_9 = '9'.codePointAt(0);
const NUM_DIGITS = DIGIT_9 - DIGIT_0 + 1;

const UPPER_A = 'A'.codePointAt(0);
const UPPER_Z = 'Z'.codePointAt(0);
const NUM_UPPERS = UPPER_Z - UPPER_A + 1;

const LOWER_A = 'a'.codePointAt(0);
const LOWER_Z = 'z'.codePointAt(0);
const NUM_LOWERS = LOWER_Z - LOWER_A + 1;

const SEMICOLON = ';'.codePointAt(0);

function char_index(cp) {
  if (cp >= DIGIT_0 && cp <= DIGIT_9) {
    return cp - DIGIT_0;
  }
  if (cp >= UPPER_A && cp <= UPPER_Z) {
    return cp - UPPER_A + NUM_DIGITS;
  }
  if (cp >= LOWER_A && cp <= LOWER_Z) {
    return cp - LOWER_A + NUM_DIGITS + NUM_UPPERS;
  }
  if (cp === SEMICOLON) {
    return NUM_DIGITS + NUM_UPPERS + NUM_LOWERS;
  }
  console.error(`unexpected codepoint: ${cp}`);
}

for (const charref of Object.keys(entries).sort()) {
  let node_index = 0;
  let node = nodes[node_index];
  for (let i = 1; i < charref.length; ++i) {
    const cp = charref.codePointAt(i);
    const ci = char_index(cp);
    node_index = node.next[ci];
    if (node_index === 0) {
      node_index = nodes.length;
      node.next[ci] = node_index;
      node.end = false;
      nodes.push({
        buffer: charref.substring(1, i + 1),
        next: Array(63).fill(0),
        chars: '',
        end: true,
      });
    }
    node = nodes[node_index];
  }
  node.chars = entries[charref].characters;
}

// Validate nodes
for (let node of nodes) {
  if (node.end) {
    for (let next of node.next) {
      if (next !== 0) {
        console.error(`leaf node must not has next: ${node.buffer}`);
        Deno.exit(1);
      }
    }
    if (!node.buffer.endsWith(';')) {
      console.error(`leaf node must be ended with ';': ${node.buffer}`);
      Deno.exit(1);
    }
  }
}

console.log(JSON.stringify({ nodes }, null, 2));
