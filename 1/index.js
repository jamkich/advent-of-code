'use strict';

// const file = Bun.file('./test2.txt');
const file = Bun.file('./input2.txt');
const text = await file.text();

const NUM_ENUM = {
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

const convertDigits = (input) => {
  const regex = new RegExp(Object.keys(NUM_ENUM).join('|'), 'i');

  const reversedEnum = Object.keys(NUM_ENUM).map((i) => {
    return i.split('').reverse().join('');
  });

  const lastRegex = new RegExp(reversedEnum.join('|'), 'i');

  const firstMatch = input.split('\n').map((i) => {
    return i.replace(regex, (match) => {
      return match.toLowerCase() in NUM_ENUM
        ? String(NUM_ENUM[match.toLowerCase()])
        : match;
    });
  });

  const lastMatch = input.split('\n').map((i) => {
    return i
      .split('')
      .reverse()
      .join('')
      .replace(lastRegex, (match) => {
        const reverseMatch = match.split('').reverse().join('');
        return reverseMatch.toLowerCase() in NUM_ENUM
          ? String(NUM_ENUM[reverseMatch])
          : reverseMatch;
      });
  });

  const firstDigitMatch = (arr) => {
    return arr.map((str) => {
      if (str.length) {
        const match = str.match(/\d/);
        if (match) {
          return parseInt(match[0], 10);
        }
      }
    });
  };

  return firstDigitMatch(firstMatch).map(
    (e, index) => String(e) + firstDigitMatch(lastMatch)[index],
  );
};

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
    }
  });

  return sum;
};

const task2 = convertDigits(text);
console.log('-------------------');
console.log(output(task2));
