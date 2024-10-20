print(0); ///#0=0
if (false) {
  await 0;
} else {
  print(1); ///#1=1
  await 0;
  print(2); ///#2=2
}
print(3); ///#3=3
