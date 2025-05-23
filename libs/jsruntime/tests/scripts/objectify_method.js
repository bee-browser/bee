let a = {};

a.b = function() {
  print(a.b.x); ///=2
}

// A `Closure` bound to `a.b` will be converted into an `Object`.
// The `Object` will be bound to `a.b`.  Then `a.b.x` will work in the body of `a.b()`.
a.b.x = 2;

a.b();
