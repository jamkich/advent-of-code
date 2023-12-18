// const file = Bun.file('./test.txt');
const file = Bun.file('./input.txt');
const text = await file.text();

const output = (input) => {
  let sum = 0;
  input.split('\n').map((i) => {
    let a = '';
    for (const j of i) {
      if (isNaN(parseInt(j))) continue;
      else {
        a = a.concat(j);
      }
    }
    if (a.length) {
      a = a[0] + a.charAt(a.length - 1);
      sum += parseInt(a);
    }
  });

  return sum;
};

console.log(output(text));
