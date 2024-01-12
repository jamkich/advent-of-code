const file = Bun.file('./input');
const text = await file.text();
// const file = Bun.file('./test');
// const text = await file.text();

// Task 1
// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue

// TODO: only 12 red cubes, 13 green cubes, and 14 blue cubes?

const solution = (input) => {
  // make condition to win
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
      // const [...redMatches] = s.matchAll(/(\d+) red/g);
      // const [...greenMatches] = s.matchAll(/(\d+) green/g);
      // const [...blueMatches] = s.matchAll(/(\d+) blue/g);
      // console.log(s, redMatches, greenMatches, blueMatches);
      const sets = s.split(';');

      // TODO: iterate sets with matches and winCondition
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

      // for (const match of redMatches) {
      //   acc.red = parseInt(match[1]);
      // }
      // for (const match of greenMatches) {
      //   acc.green = parseInt(match[1]);
      // }
      // for (const match of blueMatches) {
      //   acc.blue = parseInt(match[1]);
      // }
    })
    .filter((e) => Number(e))
    .reduce((acc, cur) => acc + cur, 0);

  return a;
};

console.log(solution(text));
