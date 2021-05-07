const SUN_COST = {
  "COMPLETE": 4,
};

const MAX_DAYS = 24;

function richnessBonus(richness: number) {
  return (richness - 1) * 2;
}

console.assert(richnessBonus(1) === 0, "bonus(1) should be 0");
console.assert(richnessBonus(2) === 2, "bonus(2) should be 2");
console.assert(richnessBonus(3) === 4, "bonus(3) should be 4");

interface Cell {
  index: number;
  richness: number;
  neighbours: number[];
}

interface Player {
  isMe: boolean;
  sun: number;
  score: number;
  isWaiting: boolean;
}

interface Tree {
  cellIndex: number;
  size: number;
  isMine: boolean;
  isDormant: boolean;
}

interface Action {
  hash: string;
  command: string;
  args: string[];
}
function parseAction(hash: string): Action {
  const list = hash.split(" ");
  const [command, ...args] = list;
  return { hash, command, args };
}
interface Day {
  index: number;
  nutrients: number;
  me: Player;
  enemy: Player;
  trees: Map<number, Tree>;
  possibleActions: Action[];
}

const cells = new Map<number, Cell>();
const days = new Map<number, Day>();

function Richness(cells: Map<number, Cell>, day: Day) {
  return (tree: Tree) => {
    const cell = cells.get(tree.cellIndex);
    return day.nutrients + richnessBonus(cell.richness);
  };
}

function ByRichness(cells: Map<number, Cell>, day: Day) {
  const cellRichness = Richness(cells, day);
  return (a: Action, b: Action) => {
    if (a.command == "WAIT" && b.command == "WAIT") {
      return 0;
    } else if (a.command == "WAIT") {
      return 1;
    } else if (b.command == "WAIT") {
      return -1;
    }
    const aTree = day.trees.get(+a.args[0]);
    const bTree = day.trees.get(+b.args[0]);
    return cellRichness(bTree) - cellRichness(aTree);
  };
}

function take_turn(cells: Map<number, Cell>, day: Day) {
  day.possibleActions.sort(ByRichness(cells, day));
  console.error(day.possibleActions);
  return day.possibleActions[0].hash;
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/

const numberOfCells: number = parseInt(readline()); // 37
for (let i = 0; i < numberOfCells; i++) {
  var inputs: string[] = readline().split(" ");
  // 0 is the center cell, the next cells spiral outwards
  const index: number = parseInt(inputs[0]);
  // 0 if the cell is unusable, 1-3 for usable cells
  const richness: number = parseInt(inputs[1]);

  // the index of the neighbouring cell for each direction
  const neighbours = [
    parseInt(inputs[2]),
    parseInt(inputs[3]),
    parseInt(inputs[4]),
    parseInt(inputs[5]),
    parseInt(inputs[6]),
    parseInt(inputs[7]),
  ];
  cells.set(index, { index, richness, neighbours });
}

// game loop
while (true) {
  const index: number = parseInt(readline()); // the game lasts 24 days: 0-23
  const nutrients: number = parseInt(readline()); // the base score you gain from the next COMPLETE action

  var inputs: string[] = readline().split(" ");
  const sun: number = parseInt(inputs[0]); // your sun points
  const score: number = parseInt(inputs[1]); // your current score
  const me: Player = { isMe: true, sun, score, isWaiting: false };

  var inputs: string[] = readline().split(" ");
  const oppSun: number = parseInt(inputs[0]); // opponent's sun points
  const oppScore: number = parseInt(inputs[1]); // opponent's score
  const oppIsWaiting: boolean = inputs[2] !== "0"; // whether your opponent is asleep until the next day
  const enemy: Player = { isMe: false, sun, score, isWaiting: oppIsWaiting };
  const numberOfTrees: number = parseInt(readline()); // the current amount of trees

  const trees = new Map<number, Tree>();
  for (let i = 0; i < numberOfTrees; i++) {
    var inputs: string[] = readline().split(" ");
    const cellIndex: number = parseInt(inputs[0]); // location of this tree
    const size: number = parseInt(inputs[1]); // size of this tree: 0-3
    const isMine: boolean = inputs[2] !== "0"; // 1 if this is your tree
    const isDormant: boolean = inputs[3] !== "0"; // 1 if this tree is dormant
    trees.set(cellIndex, { cellIndex, size, isMine, isDormant });
  }
  const numberOfPossibleMoves: number = parseInt(readline());
  const possibleActions: Action[] = [];
  for (let i = 0; i < numberOfPossibleMoves; i++) {
    const possibleMove: string = readline();
    possibleActions.push(parseAction(possibleMove));
  }

  const day: Day = { index, nutrients, me, enemy, trees, possibleActions };
  days.set(index, day);
  console.log(take_turn(cells, day));
  // Write an action using console.log()
  // To debug: console.error('Debug messages...');

  // GROW cellIdx | SEED sourceIdx targetIdx | COMPLETE cellIdx | WAIT <message>
  //console.log('WAIT');
}
