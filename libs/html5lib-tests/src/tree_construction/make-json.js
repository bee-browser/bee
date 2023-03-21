const name = Deno.args[0];

const decoder = new TextDecoder('utf-8');
const dat = await decoder.decode(await Deno.readAll(Deno.stdin));

const State = {
  NONE: 1,
  DATA: 2,
  ERRORS: 3,
  NEW_ERRORS: 4,
  DOCUMENT: 5,
  DOCUMENT_FRAGMENT: 6,
};

class Test {
  constructor() {
    this.data = null;
    this.errors = [];
    this.newErrors = []
    this.document = [];
    this.scripting = 'Both';
  }

  hasData() {
    return this.data !== null && this.data.length > 0;
  }
}

let state = State.NONE;
let test = new Test();
let tests = [];

for (const line of dat.split('\n')) {
  const trimed = line.replace('\n', '');
  switch (trimed) {
  case '':
    if (test.hasData()) {
      tests.push(test);
      test = new Test();
    }
    state = State.NONE;
    continue;
  case '#data':
    state = State.DATA;
    continue;
  case '#errors':
    state = State.ERRORS;
    continue;
  case '#new-errors':
    state = State.NEW_ERRORS;
    continue;
  case '#document':
    state = State.DOCUMENT;
    continue;
  case '#document-fragment':
    state = State.DOCUMENT_FRAGMENT;
    continue;
  case '#script-on':
    test.scripting = 'On';
    continue;
  case '#script-off':
    test.scripting = 'Off';
    continue;
  default:
    break;
  }
  switch (state) {
  case State.NONE:
    // Nothing to do.
    break;
  case State.DATA:
    if (test.data === null) {
      test.data = trimed;
    } else {
      test.data += '\n' + trimed;
    }
    break;
  case State.ERRORS:
    test.errors.push(trimed);
    break;
  case State.NEW_ERRORS:
    test.newErrors.push(trimed);
    break;
  case State.DOCUMENT:
    if (trimed.startsWith('| ')) {
      test.document.push(parseDocumentLine(trimed));
    } else {
      const last = test.document.pop();
      last[1] += '\n' + trimed;
      test.document.push(last);
    }
    break;
  case State.DOCUMENT_FRAGMENT:
    const qname = trimed.split(' ');
    if (qname.length === 1) {
      test.contextElement = {
        namespace: 'html',
        localName: qname[0],
      };
    } else if (qname.length === 2) {
      test.contextElement = {
        namespace: qname[0],
        localName: qname[1],
      };
    } else {
      console.error(`Invalid format in DOCUMENT_FRAGMENT: ${trimed}`);
      Deno.exit(1);
    }
  }
}

if (test.hasData()) {
  tests.push(test);
}

console.log(JSON.stringify({ name, tests }, null, 2));

// helper functions

function parseDocumentLine(line) {
  let docLine = line;
  if (!docLine.startsWith('| ')) {
    console.error(`Invalid format: ${line}`);
    Deno.exit(1);
  }
  docLine = docLine.slice(2);  // remove '| '
  let depth = 0;
  while (docLine.startsWith('  ')) {
    docLine = docLine.slice(2);
    depth++;
  }
  if (docLine.startsWith(' ')) {
    console.error(`Invalid format: ${line}`);
    Deno.exit(1);
  }
  return [depth, docLine];
}
