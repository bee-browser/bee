let a = function() {
  print(a.x); ///=2
}

// A `Closure` bound to `a` will be converted into an `Object`.
// The `Object` will be bound to `a`.  Then `a.x` will work in the body of `a()`.
a.x = 2;

a();
