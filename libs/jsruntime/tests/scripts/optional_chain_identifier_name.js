let a = {
  b: { c: 1 },
};

print(a?.b.c); ///=1
print(a.b?.c); ///=1
print(a?.b?.c); ///=1
print(a.x?.y); ///=undefined
