// TODO(234): replace with the following code:
//
// let b = a();
// print(b());
// function a() {
//   let x = 1;
//   return () => x;
// }
let a;
{
  let x = 1;
  a = () => x;
}
// `print(a)` will print:
// [closure(<a.lambda>, [capture(escaped: <addr of escaped>)])]
print(a());
