let x = (function() {
  let a;

  // `print(b)` will print:
  // [closure(<b.lambda>, [capture(onstack: <addr of `a`>)])]
  return b();

  function b() {
    a = 1;
    return () => a = 2;
  }
})();

// `print(x)` will print:
// [closure(<lambda>, [capture(escaped: <addr of captured `a`>)])]
print(x());
