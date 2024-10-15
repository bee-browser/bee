print(0); ///#0=0
a();
print(3); ///#3=3

async function a() {
  print(1); ///#1=1
  await b();
  print(5); ///#5=5
}

async function b() {
  print(2); ///#2=2
  await 0;
  print(4); ///#4=4
}
