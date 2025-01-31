let a = {
  nested: {
    x: 1,
  },
};
print(a); ///=object
print(a.nested); ///=object
print(a['nested']); ///=object
print(a.nested.x); ///=1
print(a.nested['x']); ///=1
print(a['nested'].x); ///=1
print(a['nested']['x']); ///=1
