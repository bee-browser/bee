let middle = outer();

// `print(middle)` will print:
// [closure(<middle.lambda>, [capture(escaped: <addr of escaped>)])]
let inner = middle();

// `print(inner)` will print:
// [closure(<inner.lambda>, [capture(escaped: <same addr>)])]
print(inner()); ///=1

function outer() {
  let a = 1;
  return function() {
    return function() {
      return a;
    };
  };
}
