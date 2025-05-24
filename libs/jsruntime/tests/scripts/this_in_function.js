var a = 1;

function b() {
  return this;
}

print(b().a); ///=1
print(b() === globalThis); ///=true
