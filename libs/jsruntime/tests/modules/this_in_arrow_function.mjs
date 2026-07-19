let a = {
  b: () => this,
};

print(a.b() === undefined); ///=true
print(a.b.call(null) === undefined); ///=true

const o = {};
print(a.b.call(o) === undefined); ///=true
