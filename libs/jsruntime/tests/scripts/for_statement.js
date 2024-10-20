let i = 0;
print(i); ///=0
for (let j = 0; j < 2; ++j) {
  i = j;
  ///=0
  ///=1
  print(i);
}
print(i); ///=1
