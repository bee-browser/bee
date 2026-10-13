const X = class A {};

try {
  print(A); // TODO: 'A;' does NOT work property. Fix dereference.
} catch (e) {
  print(e instanceof ReferenceError); ///=true
}

print(X.name); ///="A"
