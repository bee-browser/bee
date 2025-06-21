let a = {
  b: function() { return this },
};

print(a.b() === a); ///=true
// TODO(feat): Function.prototype.call
// print(a.b.call(null) === null); ///=true
