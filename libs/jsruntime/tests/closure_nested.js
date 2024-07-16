let middle = outer();
let inner = middle();
print(inner());
function outer() {
  let a = 1;
  return function() {
    return function() {
      return a;
    };
  };
}
