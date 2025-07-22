print('' + undefined); ///="undefined"
print(undefined + ''); ///="undefined"

print('' + null); ///="null"
print(null + ''); ///="null"

print('' + true); ///="true"
print(true + ''); ///="true"

print('' + false); ///="false"
print(false + ''); ///="false"

// TODO: print('' + 0); ///="0"
// TODO: print('' + {}); ///="[object Object]"

print('a' + 'b'); ///="ab"

const a = 'a';
print(a + 'b'); ///="ab"
print('b' + a); ///="ba"

const b = 'b';
print(a + b); ///="ab"
