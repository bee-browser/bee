let a = { x: 1 };
let b = {
  y: 1,
  ...a
};
print(a); ///=object
// TODO(test): print(a.x); ///=1
print(b); ///=object
// TODO(test): print(b.y); ///=1
// TODO(test): print(b.x); ///=1
