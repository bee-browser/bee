class A extends null {}

try {
  const o = new A;
} catch (e) {
  print(e instanceof TypeError); ///=true
}

class B extends null {
  constructor() {
    return Object.create(new.target.prototype);
  }
}

const o = new B;
print(o instanceof B); ///=true
print(o instanceof Object); ///=false
