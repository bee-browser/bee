let x = (function() {
  let c;
  {
    let a;

    let b = function() {
      a = 1;
      return () => a = 2;
    };

    // `print(b)` will print:
    // [closure(<b.lambda>, [capture(onstack: <addr of `a`>)])]
    c = b();
  }

  // `print(c)` will print:
  // [closure(<c.lambda>, [capture(escaped: <addr of captured `a`>)])]
  return c;
})();

print(x()); ///=2
