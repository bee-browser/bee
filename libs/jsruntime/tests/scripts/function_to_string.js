print(typeof Function.prototype.toString); ///="function"
print(Function.prototype.toString.length); ///=0

function a() {};
try {
  print(a.toString()); // TODO
} catch (e) {
  print(e.name); ///="InternalError"
}
