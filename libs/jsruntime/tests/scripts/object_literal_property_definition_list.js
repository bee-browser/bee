let a = {
  undefined: undefined,
  null: null,
  true: true,
  false: false,
  1: 1,
  NaN: NaN,
  Infinity: Infinity,
  function: function() { return 0; },
  arrowFunction: () => 1,
};

print(a); ///=object
print(a.undefined); ///=undefined
print(a[undefined]); ///=undefined
print(a.null); ///=null
print(a[null]); ///=null
print(a.true); ///=true
print(a[true]); ///=true
print(a.false); ///=false
print(a[false]); ///=false
// TODO(test): print(a[1]); ///=1
print(a.NaN); ///=NaN
// TODO(test): print(a[NaN]); ///=NaN
print(a.Infinity); ///=Infinity
// TODO(test): print(a[Infinity]); ///=Infinity
print(a.function); ///=function
print(a['function']); ///=function
print(a.function()); ///=0
print(a['function']()); ///=0
print(a.arrowFunction); ///=function
print(a['arrowFunction']); ///=function
print(a.arrowFunction()); ///=1
print(a['arrowFunction']()); ///=1
