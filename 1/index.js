'use strict';

const file = Bun.file('./test3.txt');
// const file = Bun.file('./input2.txt');
const text = await file.text();

function convertDigits(input) {
  const num = {
    one: 1,
    two: 2,
    three: 3,
    four: 4,
    five: 5,
    six: 6,
    seven: 7,
    eight: 8,
    nine: 9,
  };

  const regex = new RegExp(Object.keys(num).join('|'), 'ig');

  const arr = input.split('\n').map((e) => {
    let replacedWord = e;

    replacedWord = replacedWord.replace(
      regex,
      (match) => num[match.toLowerCase()] || match,
    );

    // Replace from right to left
    replacedWord = replacedWord.split('').reverse().join('');
    replacedWord = replacedWord.replace(
      regex,
      (match) => num[match.toLowerCase()] || match,
    );
    replacedWord = replacedWord.split('').reverse().join('');

    return replacedWord;
  });

  return arr;
}

const output = (input) => {
  let sum = 0;
  input.map((i) => {
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
      console.log(a);
    }
  });

  return sum;
};

const task2 = convertDigits(text);
console.log('-------------------');
console.log(output(task2));
