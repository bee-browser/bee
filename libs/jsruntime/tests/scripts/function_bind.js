print(typeof Function.prototype.bind); ///="function"
print(Function.prototype.bind.length); ///=2

const o = {
  x: 1,
};

function sum(a, b, c) {
  return a + b + c + this.x;
};

print(sum()); ///=NaN

const bound = sum.bind(o, 2, 3, 4);
print(bound()); ///=10
