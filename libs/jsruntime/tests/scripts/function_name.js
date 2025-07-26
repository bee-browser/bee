print((function() {}).name); ///=""
print((function a() {}).name); ///="a"

function b() {}
print(b.name); ///="b"

const c = function x() {};
print(c.name); ///="x"

const d = function() {};
print(d.name); ///="d"

let e = function x() {};
print(e.name); ///="x"

let f = function() {};
print(f.name); ///="f"

var g = function x() {};
print(g.name); ///="x"

let h = function() {};
print(h.name); ///="h"

print(({ i: function() {} }).i.name); ///="i"

const y = {};
y.j = function () {};
print(y.j.name); ///=""

(function(k) {
  print(k.name); ///=""
})(function() {});
