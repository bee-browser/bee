let a = {
  b: function() { return this },
};

print(a.b() === a); ///=true
print(a.b.call(null) === null); ///=true
