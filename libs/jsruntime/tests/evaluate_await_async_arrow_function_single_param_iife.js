print(0); ///#0=0
(async x => {
  print(1); ///#1=1
  await x;
  print(3); ///#3=3
})(0);
print(2); ///#2=2
