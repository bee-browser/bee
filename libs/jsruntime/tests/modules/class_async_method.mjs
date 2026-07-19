class A {
  async a() {
    return await this;
  }
}

print(typeof A.prototype.a); ///="function"
print(A.prototype.a.name); ///="a"

const o = new A();
print(await o.a() === o); ///=true
print(await o.a.call(null) === null); ///=true
