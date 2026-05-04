try {
  print(A);
} catch (e) {
  print(e.name); ///="ReferenceError"
}

class A {}

print(typeof A); ///="function"
print(A.name); ///="A"

print(typeof A.prototype); ///="object"
print(A.prototype.constructor === A); ///=true
