try {
  print(a);
} catch (e) {
  print(e.name); ///="ReferenceError"
}

// `a` is never captured because it's a global variable.
function x() {
  return () => a + b;
}

a = 1;
print(a); ///=1

const y = x();
const b = 2;
print(y()); ///=3
