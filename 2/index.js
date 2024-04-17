const file = Bun.file('./input');
const text = await file.text();
// const file = Bun.file('./test');
// const text = await file.text();

const part1Solution = (input) => {
  const winCondition = {
    red: 12,
    green: 13,
    blue: 14,
  };

  // parse input
  const a = input
    .split('\n')
    .filter((e) => e.trim())
    .map((s) => {
      const gameNumber = parseInt(s.substr(s.indexOf(' '), s.indexOf(':')));
      const acc = {
        red: 0,
        green: 0,
        blue: 0,
      };

      const sets = s.split(';');

      for (const set of sets) {
        const [...redMatches] = set.matchAll(/(\d+) red/g);
        const [...greenMatches] = set.matchAll(/(\d+) green/g);
        const [...blueMatches] = set.matchAll(/(\d+) blue/g);

        for (const match of redMatches) {
          acc.red = parseInt(match[1]);
        }
        for (const match of greenMatches) {
          acc.green = parseInt(match[1]);
        }
        for (const match of blueMatches) {
          acc.blue = parseInt(match[1]);
        }

        if (
          acc.red > winCondition.red ||
          acc.green > winCondition.green ||
          acc.blue > winCondition.blue
        ) {
          break;
        } else {
          if (set === sets.at(-1)) return gameNumber;
        }
      }
    })
    .filter((e) => Number(e))
    .reduce((acc, cur) => acc + cur, 0);

  return a;
};

// console.log(part1Solution(text));

const part2Solution = (input) => {
  // parse input
  const a = input
    .split('\n')
    .filter((e) => e.trim())
    .map((s) => {
      const acc = {
        red: 0,
        green: 0,
        blue: 0,
        power() {
          return this.red * this.green * this.blue;
        },
      };
      const sets = s.split(';');

      for (const set of sets) {
        const [...redMatches] = set.matchAll(/(\d+) red/g);
        const [...greenMatches] = set.matchAll(/(\d+) green/g);
        const [...blueMatches] = set.matchAll(/(\d+) blue/g);

        for (const match of redMatches) {
          if (parseInt(match[1]) > acc.red) acc.red = parseInt(match[1]);
        }
        for (const match of greenMatches) {
          if (parseInt(match[1]) > acc.green) acc.green = parseInt(match[1]);
        }
        for (const match of blueMatches) {
          if (parseInt(match[1]) > acc.blue) acc.blue = parseInt(match[1]);
        }
      }
      return acc.power();
    })
    .filter((e) => Number(e))
    .reduce((acc, cur) => acc + cur, 0);

  return a;
};

console.log(part2Solution(text));
