use std::collections::HashMap;
use std::io;

macro_rules! readline {
  () => {{
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    input_line
  }};
}

macro_rules! parse_input {
  ($x:expr, $t:ident) => {
    $x.trim().parse::<$t>().unwrap()
  };
}

const MAX_DAYS: i32 = 24;

fn sun_direction(day: i32) -> usize {
  (day % 6) as usize
}

#[derive(Debug)]
struct Cell {
  index: i32,
  richness: i32,
  neighbours: [i32; 6],
  shadow: i32,
}

impl Cell {
  pub fn parse(line: &str) -> Self {
    let inputs = line.split(" ").collect::<Vec<_>>();
    return Self {
      index: parse_input!(inputs[0], i32),
      richness: parse_input!(inputs[1], i32),
      neighbours: [
        parse_input!(inputs[2], i32),
        parse_input!(inputs[3], i32),
        parse_input!(inputs[4], i32),
        parse_input!(inputs[5], i32),
        parse_input!(inputs[6], i32),
        parse_input!(inputs[7], i32),
      ],
      shadow: 0,
    };
  }

  pub fn next(&self, direction: usize) -> Option<i32> {
    let next = self.neighbours[direction];
    if next == -1 {
      None
    } else {
      Some(next)
    }
  }
}

struct CellMap {
  map: HashMap<i32, Cell>,
}

impl CellMap {
  pub fn with_capacity(capacity: usize) -> Self {
    Self {
      map: HashMap::with_capacity(capacity),
    }
  }

  pub fn push(&mut self, line: &str) {
    let cell = Cell::parse(line);
    self.map.insert(cell.index, cell);
  }

  pub fn cast_shadows(&mut self, direction: usize, trees: &Vec<Tree>) {
    for (_, v) in self.map.iter_mut() {
      v.shadow = 0;
    }

    for tree in trees.iter() {
      let mut size = tree.size;
      let mut current = tree.cell_index;
      while size > 0 && current != -1 {
        if let Some(cell) = self.map.get_mut(&current) {
          cell.shadow = tree.size;
          current = cell.neighbours[direction];
          size -= 1;
        }
      }
    }
  }
}

#[derive(Debug)]
struct Tree {
  cell_index: i32,
  size: i32,
  is_mine: bool,
  is_dormant: bool,
}
impl Tree {
  pub fn parse(line: &str) -> Self {
    let inputs = line.split(" ").collect::<Vec<_>>();
    Self {
      cell_index: parse_input!(inputs[0], i32),
      size: parse_input!(inputs[1], i32), // size of this tree: 0-3
      is_mine: parse_input!(inputs[2], i32) == 1, // 1 if this is your tree
      is_dormant: parse_input!(inputs[3], i32) == 1, // 1 if this tree is dormant
    }
  }
}

#[derive(Debug)]
enum ActionCategory {
  GROW(i32),
  SEED(i32, i32),
  COMPLETE(i32),
  WAIT,
}
#[derive(Debug)]
struct Action {
  hash: String,
  category: ActionCategory,
  value: i32,
}

impl Action {
  pub fn parse(hash: String) -> Self {
    let inputs = hash.split(" ").collect::<Vec<_>>();

    let category = match inputs[0] {
      "GROW" => ActionCategory::GROW(parse_input!(inputs[1], i32)),
      "SEED" => ActionCategory::SEED(parse_input!(inputs[1], i32), parse_input!(inputs[2], i32)),
      "COMPLETE" => ActionCategory::COMPLETE(parse_input!(inputs[1], i32)),
      _ => ActionCategory::WAIT,
    };
    return Self {
      hash,
      category,
      value: -1,
    };
  }
}

#[derive(Debug)]
struct DayData {
  index: i32,
  nutrients: i32,
  sun: i32,
  score: i32,
  opp_sun: i32,
  opp_score: i32,
  opp_is_waiting: i32,
  trees: Vec<Tree>,
  actions: Vec<Action>,
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
pub fn main() {
  let number_of_cells = parse_input!(readline!(), i32); // 37
  let mut cell_map = CellMap::with_capacity(number_of_cells as usize);
  for _ in 0..number_of_cells as usize {
    cell_map.push(&readline!());
  }

  // game loop
  loop {
    let day = parse_input!(readline!(), i32); // the game lasts 24 days: 0-23
    if day >= MAX_DAYS {
      return;
    }
    let nutrients = parse_input!(readline!(), i32); // the base score you gain from the next COMPLETE action

    let input_line = readline!();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let sun = parse_input!(inputs[0], i32); // your sun points
    let score = parse_input!(inputs[1], i32); // your current score

    let input_line = readline!();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let opp_sun = parse_input!(inputs[0], i32); // opponent's sun points
    let opp_score = parse_input!(inputs[1], i32); // opponent's score
    let opp_is_waiting = parse_input!(inputs[2], i32); // whether your opponent is asleep until the next day

    let mut day = DayData {
      index: day,
      nutrients,
      sun,
      score,
      opp_sun,
      opp_score,
      opp_is_waiting,
      trees: Vec::new(),
      actions: Vec::new(),
    };

    let number_of_trees = parse_input!(readline!(), i32); // the current amount of trees
    for _ in 0..number_of_trees as usize {
      let input_line = readline!();
      day.trees.push(Tree::parse(&input_line))
    }

    let number_of_possible_actions = parse_input!(readline!(), i32); // all legal actions
    for _ in 0..number_of_possible_actions as usize {
      let input_line = readline!();
      day
        .actions
        .push(Action::parse(input_line.trim().to_string()))
    }
    eprintln!("{:#?}", day);
    // Write an action using println!("message...");
    // To debug: eprintln!("Debug message...");

    // GROW cellIdx | SEED sourceIdx targetIdx | COMPLETE cellIdx | WAIT <message>
    println!("WAIT");
  }
}
