let a = { x: 1 };
print(a); ///=object
print(a.x); ///=1
print(a['x']); ///=1

let b = {
  y: 1,
  ...a
};
print(b); ///=object
print(b.y); ///=1
print(b['y']); ///=1
print(b.x); ///=1
print(b['x']); ///=1
