print('' + undefined); ///="undefined"
print('' + null); ///="null"
print('' + true); ///="true"
print('' + false); ///="false"
print('' + 1); ///="1"
print('' + 'a'); ///="a"
// TODO: print('' + {}); ///="[object Object]"
// TODO: print('' + new String('a')); ///="a"

let a = {};
print('' + a); ///="[object Object]"

a = new String('a');
print('' + a); ///="a"
