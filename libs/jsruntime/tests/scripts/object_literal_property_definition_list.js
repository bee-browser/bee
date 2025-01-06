let a = {
  undefined: undefined,
  null: null,
  true: true,
  false: false,
  1: 1,
  NaN: NaN,
  Infinity: Infinity,
};

print(a); ///=object
print(a.undefined); ///=undefined
print(a.null); ///=null
print(a.true); ///=true
print(a.false); ///=false
// TODO(test): print(a[1]); ///=1
print(a.NaN); ///=NaN
print(a.Infinity); ///=Infinity
