print(0); ///#0=0
(async () => {
  print(1); ///#1=1
  await 0;
  print(3); ///#3=3
})();
print(2); ///#2=2
