print(undefined ?? 1); ///=1
print(null ?? 1); ///=1
print(0 ?? 1); ///=0

let a;

a = 0;
null ?? (a = 1);
print(a); ///=1

a = 0;
0 ?? (a = 1);
print(a); ///=0
