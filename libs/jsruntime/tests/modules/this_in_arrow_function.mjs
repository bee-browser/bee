let a = {
  b: () => this,
};

print(a.b() === undefined); ///=true
