let b = a();

// `print(b)` will print:
// [closure(<a.lambda>, [capture(escaped: <addr of escaped>)])]
print(b()); ///=1

function a() {
  let x = 1;
  return () => x;
}
