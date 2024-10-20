let a;

{
  let x = 1;
  a = () => x;
}

// `print(a)` will print:
// [closure(<a.lambda>, [capture(escaped: <addr of escaped>)])]
print(a()); ///=1
