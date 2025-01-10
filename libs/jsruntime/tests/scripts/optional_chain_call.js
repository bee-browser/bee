let a = {
  b: () => () => 1,
  c: () => {},
};

print(a.b?.()()); ///=1
print(a.b()?.()); ///=1
print(a.b?.()?.()); ///=1
print(a.c()?.()); ///=undefined
print(a.x?.()); ///=undefined
