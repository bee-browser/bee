print(typeof Function.prototype.apply); ///="function"
print(Function.prototype.apply.length); ///=2

function a(x) {
  print(x);
}

a.apply(this); ///=undefined
a.apply(this, [1, 2]); ///=1
