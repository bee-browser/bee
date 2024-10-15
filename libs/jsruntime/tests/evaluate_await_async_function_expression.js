print(0); ///#0=0
let a = async function() {
  print(1); ///#1=1
  await 0;
  print(3); ///#3=3
};
a();
print(2); ///#2=2
