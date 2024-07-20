// TODO(issue#234): replace with the following code:
//
// let middle = outer();
// let inner = middle();
// print(inner());
// function outer() {
//   let a = 1;
//   return function() {
//     return function() {
//       return a;
//     };
//   };
// }
let middle;
{
  let a = 1;
  middle = () => () => a;
}
// `print(middle)` will print:
// [closure(<middle.lambda>, [capture(escaped: <addr of escaped>)])]
let inner = middle();
// `print(inner)` will print:
// [closure(<inner.lambda>, [capture(escaped: <same addr>)])]
print(inner());
