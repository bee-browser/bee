import { pascalCase } from "https://deno.land/x/case/mod.ts";

const decoder = new TextDecoder('utf-8');
const json = await decoder.decode(await Deno.readAll(Deno.stdin));
const data = JSON.parse(json);

data.name = Deno.args[0];

function isString(v) {
  return typeof v === 'string' || v instanceof String;
}

// https://github.com/html5lib/html5lib-tests/blob/master/tokenizer/README.md
for (let testIndex = 0; testIndex < data.tests.length; ++testIndex) {
  const test = data.tests[testIndex];
  if (test.doubleEscaped) {
    test.input = JSON.parse(`"${test.input}"`);
    test.output = test.output.map((output) => {
      return output.map((v) => {
        return isString(v) ? JSON.parse(`"${v}"`) : v;
      });
    });
  }

  const input = [];
  for (let i = 0; i < test.input.length; ++i) {
    input.push(test.input.charCodeAt(i));
  }
  test.input = input;

  if (test.initialStates) {
    test.initialStates = test.initialStates.map((state) => {
      // 'CDATA section state' -> 'CdataSection'
      return pascalCase(state.split(' ').slice(0, -1).join(' '));
    });
  } else {
    test.initialStates = ['Data'];
  }

  test.output = test.output.map((output) => {
    switch (output[0]) {
    case 'StartTag':
      return {
        StartTag: {
          name: output[1],
          attrs: output[2],
          self_closing: !!output[3],
        }
      };
    case 'EndTag':
      return {
        EndTag: {
          name: output[1],
        }
      };
    case 'Character':
      return {
        Character: {
          data: output[1],
        }
      };
    case 'Comment':
      return {
        Comment: {
          data: output[1],
        }
      };
    case 'DOCTYPE':
      return {
        Doctype: {
          name: output[1],
          public_id: output[2],
          system_id: output[3],
          force_quirks: !output[4],
        }
      };
    default:
      throw new Error(`unknown output: ${output[0]}`);
    }
  });

  if (test.errors) {
    test.errors = test.errors.map((error) => {
      return {
        code: pascalCase(error.code),
        location: {
          line: error.line,
          column: error.col,
        },
      };
    });
  } else {
    test.errors = [];
  }

  if (data.name === 'test3') {
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
  if (data.name === 'unicodeCharsProblematic') {
    for (let output of test.output) {
      if (output.Character) {
        // Invalid characters will be replaced with U+FFFD by a tokenizer.
        output.Character.data = output.Character.data.replace('\uD800', '\uFFFD');
        output.Character.data = output.Character.data.replace('\uDFFF', '\uFFFD');
      }
    }
  }
}

console.log(JSON.stringify(data, null, 2));
