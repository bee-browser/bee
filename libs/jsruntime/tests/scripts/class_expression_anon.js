const X = class {}
// TODO(feat): print(X.name); ///="X"

const o = new X;
print(o instanceof X); ///=true
print(o instanceof Object); ///=true
