'use strict';

import * as path from '@std/path';
import { pascalCase } from '@luca/cases';

const testFile = Deno.args[0];
const name = path.basename(testFile, '.test');
const original = JSON.parse(await Deno.readTextFile(testFile));

function isString(v) {
  return typeof v === 'string' || v instanceof String;
}

let data = [];

// https://github.com/html5lib/html5lib-tests/blob/master/tokenizer/README.md
for (let testIndex = 0; testIndex < original.tests.length; ++testIndex) {
  const test = original.tests[testIndex];
  if (test.doubleEscaped) {
    test.input = JSON.parse(`"${test.input}"`);
    test.output = test.output.map((output) => {
      return output.map((v) => {
        return isString(v) ? JSON.parse(`"${v}"`) : v;
      });
    });
  }

  const inputUtf16 = [];
  for (let i = 0; i < test.input.length; ++i) {
    inputUtf16.push(test.input.charCodeAt(i));
  }
  test.inputUtf16 = inputUtf16;

  if (test.initialStates) {
    test.initialStates = test.initialStates.map((state) => {
      // 'CDATA section state' -> 'CdataSection'
      return pascalCase(state.split(' ').slice(0, -1).join(' '));
    });
  } else {
    test.initialStates = ['Data'];
  }

  // Normalize output data.
  test.output = test.output.map((output) => {
    switch (output[0]) {
      case 'StartTag':
        return {
          StartTag: {
            name: output[1],
            attrs: output[2],
            self_closing: !!output[3],
          },
        };
      case 'EndTag':
        return {
          EndTag: {
            name: output[1],
          },
        };
      case 'Character':
        return {
          Character: {
            data: output[1],
          },
        };
      case 'Comment':
        return {
          Comment: {
            data: output[1],
          },
        };
      case 'DOCTYPE':
        return {
          Doctype: {
            name: output[1],
            public_id: output[2],
            system_id: output[3],
            force_quirks: !output[4],
          },
        };
      default:
        throw new Error(`unknown output: ${output[0]}`);
    }
  });

  if (test.errors) {
    test.errors = test.errors.map((error) => {
      return {
        code: error.code,
        location: {
          line: error.line,
          column: error.col,
        },
      };
    });
  } else {
    test.errors = [];
  }

  if (name === 'test3') {
    switch (testIndex) {
      case 67:
      case 139:
      case 160:
      case 228:
      case 248:
      case 268:
      case 399:
      case 474:
      case 542:
      case 613:
      case 701:
      case 706:
      case 804:
      case 875:
      case 941:
      case 1010:
      case 1098:
      case 1103:
      case 1589:
        // '\uDBC0\uDC00' should be treat as a single character.
        test.errors.at(-1).location.column -= 1;
        break;
    }
  }
  if (name === 'unicodeCharsProblematic') {
    if (test.input.includes('\uD800') || test.input.includes('\uDFFF')) {
      // Required for avoiding a parse error in serde_json.
      test.input = '';
    }
    for (let output of test.output) {
      if (output.Character) {
        // Invalid characters will be replaced with U+FFFD by a tokenizer.
        output.Character.data = output.Character.data.replace('\uD800', '\uFFFD');
        output.Character.data = output.Character.data.replace('\uDFFF', '\uFFFD');
      }
    }
  }

  for (let i = 0; i < test.initialStates.length; ++i) {
    data.push({
      description: test.description,
      initialState: test.initialStates[i],
      lastStartTag: test.lastStartTag,
      input: test.input,
      inputUtf16: test.inputUtf16,
      output: test.output,
      errors: test.errors,
    });
  }
}

console.log(JSON.stringify(data));
