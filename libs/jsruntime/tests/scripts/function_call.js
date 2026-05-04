print(typeof Function.prototype.call); ///="function"
print(Function.prototype.call.length); ///=2

function a(x, y) {
  print(y);
}

a.call(this); ///=undefined
a.call(this, 1, 2); ///=2
