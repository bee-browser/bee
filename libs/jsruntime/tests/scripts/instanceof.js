print({} instanceof Object); ///=true
print((function(){}) instanceof Function); ///=true
print((function(){}) instanceof Object); ///=true

print(1 instanceof Object); ///=false
print((function(){}) instanceof (function(){})); ///=false

// TODO(test): (new A) instanceof A

try {
  print(1 instanceof 1);
} catch (e) {
  print(e instanceof TypeError); ///=true
}
