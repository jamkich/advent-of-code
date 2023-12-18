const file = Bun.file('./test.txt');
// const file = Bun.file('./input.txt');
const text = await file.text();

const output = (input) => {
  let sum = 0;
  const words = input.split('\n').map((i) => {
    let a = '';
    for (const j of i) {
      if (isNaN(parseInt(j))) continue;
      else {
        // get first number
        // get last number
        a = a.concat(j);
        if (j === a.slice(-1)) {
          if (a.length === 1) {
            console.log(a);
            a = a.concat(a);
            sum += parseInt(a);
          } else sum += parseInt(a);
        }
      }
    }
  });
  return sum;
};

console.log(output(text));
