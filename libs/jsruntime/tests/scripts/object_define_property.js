print(typeof Object.defineProperty); ///="function"
print(Object.defineProperty.length); ///=3

const o = Object.defineProperty({}, 'a', {
  value: 1
});

print(o.a); ///=1
