use crate::AoCDay;
use std::collections::HashMap;
use std::ops::Add;

pub struct Code;

type NUM = isize;
/**
 * Use boolean as indicator true = black, false/undef = white
 */
type XGrid = HashMap<NUM, bool>;
type XYGrid = HashMap<NUM, XGrid>;
type XYZGrid = HashMap<NUM, XYGrid>;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct XYZCoord(NUM, NUM, NUM);

const ORIGIN: XYZCoord = XYZCoord(0, 0, 0);

type FlatGrid = HashMap<XYZCoord, bool>;

struct ConwayFloor {
    buffer_1: FlatGrid,
    buffer_2: FlatGrid,
    using_buffer_1: bool,
}

#[derive(PartialEq, Debug)]
enum DIRECTION {
    NorthEast,
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
}

type ACTIONS = Vec<Vec<DIRECTION>>;

const FL_CONT: &str = include_str!("../../inputs/Day24");

impl AoCDay for Code {    
    fn part1(&self, _input: &mut dyn std::io::Read, _extra_argss: &[String]) -> String {
        let file_conts: String = FL_CONT.to_string();
        let mut grid: XYZGrid = HashMap::new();
        flip_tiles(tokenize(file_conts.to_string()), &mut grid);
        return count_black_tiles(&grid).to_string();
    }
    fn part2(&self, _input: &mut dyn std::io::Read, _extra_args: &[String]) -> String {
        let file_conts: String = FL_CONT.to_string();
        let mut grid: XYZGrid = HashMap::new();
        flip_tiles(tokenize(file_conts.to_string()), &mut grid);
        return ConwayFloor::from_grid(&grid).ticks(100).to_string();
    }
}

impl DIRECTION {
    const fn value(&self) -> XYZCoord {
        match *self {
            DIRECTION::NorthEast => XYZCoord(1, 0, -1),
            DIRECTION::East => XYZCoord(1, -1, 0),
            DIRECTION::SouthEast => XYZCoord(0, -1, 1),
            DIRECTION::SouthWest => XYZCoord(-1, 0, 1),
            DIRECTION::West => XYZCoord(-1, 1, 0),
            DIRECTION::NorthWest => XYZCoord(0, 1, -1),
        }
    }
}

impl<'a, 'b> Add<&'b XYZCoord> for &'a XYZCoord {
    type Output = XYZCoord;

    fn add(self, other: &'b XYZCoord) -> XYZCoord {
        return XYZCoord(self.0 + other.0, self.1 + other.1, self.2 + other.2);
    }
}

impl XYZCoord {
    fn neighbours(self) -> [XYZCoord; 6] {
        return [
            self.add(&DIRECTION::NorthEast.value()),
            self.add(&DIRECTION::East.value()),
            self.add(&DIRECTION::SouthEast.value()),
            self.add(&DIRECTION::SouthWest.value()),
            self.add(&DIRECTION::West.value()),
            self.add(&DIRECTION::NorthWest.value()),
        ];
    }
}

fn flip_tiles(actions: ACTIONS, tiles: &mut XYZGrid) {
    for act in actions {
        flip_tile(act, tiles);
    }
}

fn flip_tile(action: Vec<DIRECTION>, tiles: &mut XYZGrid) {
    let mut tile: XYZCoord = ORIGIN;
    for mv in action {
        tile = tile.add(&mv.value());
    }
    if tile.0 + tile.1 + tile.2 != 0 {
        panic!("Invalid co-ords?")
    }
    tiles
        .entry(tile.0)
        .or_default()
        .entry(tile.1)
        .or_default()
        .entry(tile.2)
        .and_modify(|f| *f = !(*f))
        .or_insert(true);
}

fn count_black_tiles(tiles: &XYZGrid) -> usize {
    return tiles.iter().fold(0, |x, y| {
        x + y.1.iter().fold(0, |x, y| {
            x + y.1.iter().fold(0, |x, y| if *y.1 { x + 1 } else { x })
        })
    });
}

fn tokenize(inline: String) -> ACTIONS {
    #[derive(PartialEq)]
    enum State {
        Normal,
        North,
        South,
    }
    let mut lines: ACTIONS = Vec::new();
    for line in inline.split('\n') {
        let mut lnd = Vec::new();
        //Let's
        let mut state: State = State::Normal;
        for c in line.chars() {
            if c == 'e' {
                match state {
                    State::North => lnd.push(DIRECTION::NorthEast),
                    State::South => lnd.push(DIRECTION::SouthEast),
                    State::Normal => lnd.push(DIRECTION::East),
                }
                state = State::Normal;
            } else if c == 'w' {
                match state {
                    State::North => lnd.push(DIRECTION::NorthWest),
                    State::South => lnd.push(DIRECTION::SouthWest),
                    State::Normal => lnd.push(DIRECTION::West),
                }
                state = State::Normal;
            } else if c == 'n' && state == State::Normal {
                state = State::North;
            } else if c == 's' && state == State::Normal {
                state = State::South;
            } else if c == ' ' {
                //Do nothing?
            } else {
                panic!("Invalid state for tokenizer");
            }
        }
        lines.push(lnd);
    }
    return lines;
}

impl ConwayFloor {
    pub fn new() -> ConwayFloor {
        Self {
            buffer_1: HashMap::new(),
            buffer_2: HashMap::new(),
            using_buffer_1: true,
        }
    }

    pub fn from_grid(grid: &XYZGrid) -> ConwayFloor {
        let mut floor = Self::new();
        grid.into_iter().for_each(|x| {
            x.1.into_iter().for_each(|y| {
                y.1.into_iter().for_each(|v| {
                    floor.buffer_1.insert(XYZCoord(*x.0, *y.0, *v.0), *v.1);
                })
            })
        });
        return floor;
    }
    //Not mutable - no edit
    fn current_buffer(&self) -> &FlatGrid {
        if self.using_buffer_1 {
            &self.buffer_1
        } else {
            &self.buffer_2
        }
    }
    //Mutable to edit
    fn next_buffer(&mut self) -> &mut FlatGrid {
        if self.using_buffer_1 {
            &mut self.buffer_2
        } else {
            &mut self.buffer_1
        }
    }

    fn is_black(&self, xyz: &XYZCoord) -> bool {
        return *self.current_buffer().get(xyz).unwrap_or(&false);
    }

    fn set_next_colour(&mut self, black: bool, xyz: &XYZCoord) {
        let next = self.next_buffer();
        match next.get_mut(xyz) {
            Some(val) => {
                *val = black
            },
            None => {
                //Don't bother inserting a white value ig?
                if black{
                    next.insert(*xyz, black);
                }
            }
        }
        if black {
            for neighbour in xyz.neighbours().iter() {
                if next.get(neighbour).is_none() {
                    //next.insert(*neighbour, false);
                }
            }
        }
    }

    fn ne_black(&mut self, xyz: XYZCoord){
        let black: bool = self.is_black(&xyz);
        let black_neighbours: usize = xyz.neighbours()
            .iter()
            .map(|neighbor| self.is_black(neighbor))
            .filter(|black| *black)
            .count();
        if black && (black_neighbours == 0 || black_neighbours > 2) {
            self.set_next_colour(false, &xyz);
        }else if (!black) && (black_neighbours == 2){
            self.set_next_colour(true, &xyz);
        }else {
            self.set_next_colour(black, &xyz)
        }
    }

    fn tick(&mut self){
        let coords: Vec<XYZCoord> = self.current_buffer().keys().map(|&xyz| xyz).collect();
        for xyz in coords{
            //Need to check self and all neighbours for an alive state!
            self.ne_black(xyz);
            for nb in xyz.neighbours().iter(){
                self.ne_black(*nb);
            }
        }
        self.using_buffer_1 = !self.using_buffer_1;
        self.next_buffer().clear();
    }

    pub fn ticks(&mut self, ticks: usize) -> usize{
        let mut itr = 0;
        while itr < ticks {
            self.tick();
            itr += 1;
        }
        return self.current_buffer()
            .iter()
            .filter(|f| *f.1)
            .count();
    }
}

#[cfg(test)]
mod test_token {
    use crate::day24::{tokenize, DIRECTION};
    #[test]
    fn test__tokenize() {
        let se = tokenize("se".to_string());
        assert_eq!(se.len(), 1);
        assert_eq!(se[0].len(), 1);
        assert_eq!(se[0][0], DIRECTION::SouthEast);

        let nw = tokenize("nw".to_string());
        assert_eq!(nw.len(), 1);
        assert_eq!(nw[0].len(), 1);
        assert_eq!(nw[0][0], DIRECTION::NorthWest);

        let e = tokenize("e".to_string());
        assert_eq!(e.len(), 1);
        assert_eq!(e[0].len(), 1);
        assert_eq!(e[0][0], DIRECTION::East);

        let clock = tokenize("neeseswwnw".to_string());
        assert_eq!(clock.len(), 1);
        assert_eq!(clock[0].len(), 6);
        assert_eq!(clock[0][0], DIRECTION::NorthEast);
        assert_eq!(clock[0][1], DIRECTION::East);
        assert_eq!(clock[0][2], DIRECTION::SouthEast);
        assert_eq!(clock[0][3], DIRECTION::SouthWest);
        assert_eq!(clock[0][4], DIRECTION::West);
        assert_eq!(clock[0][5], DIRECTION::NorthWest);

        let twotone = tokenize("neeseswwnw\nnwwseeeesw".to_string());
        assert_eq!(twotone.len(), 2);
        assert_eq!(twotone[0].len(), 6);
        assert_eq!(twotone[0][0], DIRECTION::NorthEast);
        assert_eq!(twotone[0][1], DIRECTION::East);
        assert_eq!(twotone[0][2], DIRECTION::SouthEast);
        assert_eq!(twotone[0][3], DIRECTION::SouthWest);
        assert_eq!(twotone[0][4], DIRECTION::West);
        assert_eq!(twotone[0][5], DIRECTION::NorthWest);
        assert_eq!(twotone[1].len(), 7);
        assert_eq!(twotone[1][0], DIRECTION::NorthWest);
        assert_eq!(twotone[1][1], DIRECTION::West);
        assert_eq!(twotone[1][2], DIRECTION::SouthEast);
        assert_eq!(twotone[1][3], DIRECTION::East);
        assert_eq!(twotone[1][4], DIRECTION::East);
        assert_eq!(twotone[1][5], DIRECTION::East);
        assert_eq!(twotone[1][6], DIRECTION::SouthWest);
    }
}

#[cfg(test)]
mod test_flip {
    use crate::day24::count_black_tiles;
    use crate::day24::flip_tiles;
    use crate::day24::{flip_tile, tokenize, XYZGrid, DIRECTION};
    use std::collections::HashMap;

    pub const FILE_CONTS: &str = include_str!("../../inputs/day24-test");
    #[test]
    fn test_simple() {
        let mut grid: XYZGrid = HashMap::new();
        flip_tile(tokenize("se".to_string()).pop().unwrap(), &mut grid);
        assert_eq!(grid.len(), 1);
        assert_eq!(grid.contains_key(&DIRECTION::SouthEast.value().0), true);
        let par_x_a = grid.get(&DIRECTION::SouthEast.value().0).unwrap();
        assert_eq!(par_x_a.contains_key(&DIRECTION::SouthEast.value().1), true);
        let par_y_a = par_x_a.get(&DIRECTION::SouthEast.value().1).unwrap();
        assert_eq!(*par_y_a.get(&DIRECTION::SouthEast.value().2).unwrap(), true);
        assert_eq!(count_black_tiles(&grid), 1);
        flip_tile(tokenize("ewewewse".to_string()).pop().unwrap(), &mut grid);
        assert_eq!(grid.len(), 1);
        assert_eq!(grid.contains_key(&DIRECTION::SouthEast.value().0), true);
        let par_x_b = grid.get(&DIRECTION::SouthEast.value().0).unwrap();
        assert_eq!(par_x_b.contains_key(&DIRECTION::SouthEast.value().1), true);
        let par_y_b = par_x_b.get(&DIRECTION::SouthEast.value().1).unwrap();
        assert_eq!(
            *par_y_b.get(&DIRECTION::SouthEast.value().2).unwrap(),
            false
        );
        assert_eq!(count_black_tiles(&grid), 0);
        flip_tile(tokenize("esw".to_string()).pop().unwrap(), &mut grid);
        assert_eq!(grid.len(), 1);
        assert_eq!(grid.len(), 1);
        assert_eq!(grid.contains_key(&DIRECTION::SouthEast.value().0), true);
        let par_x_c = grid.get(&DIRECTION::SouthEast.value().0).unwrap();
        assert_eq!(par_x_c.contains_key(&DIRECTION::SouthEast.value().1), true);
        let par_y_c = par_x_c.get(&DIRECTION::SouthEast.value().1).unwrap();
        assert_eq!(*par_y_c.get(&DIRECTION::SouthEast.value().2).unwrap(), true);
        assert_eq!(count_black_tiles(&grid), 1);
        //Lovely, let's go further!
        flip_tile(tokenize("nenene".to_string()).pop().unwrap(), &mut grid);
        assert_eq!(grid.len(), 2);
        //We will establish that the se one is still true

        assert_eq!(grid.contains_key(&DIRECTION::SouthEast.value().0), true);
        let par_x_d = grid.get(&DIRECTION::SouthEast.value().0).unwrap();
        assert_eq!(par_x_d.contains_key(&DIRECTION::SouthEast.value().1), true);
        let par_y_d = par_x_d.get(&DIRECTION::SouthEast.value().1).unwrap();
        assert_eq!(*par_y_d.get(&DIRECTION::SouthEast.value().2).unwrap(), true);
        //And we shall check the nenene one
        assert_eq!(grid.contains_key(&3), true);
        //XYZCoord(1, 0, -1)
        assert_eq!(grid.contains_key(&3), true);
        let par_x_e = grid.get(&3).unwrap();
        assert_eq!(par_x_e.contains_key(&0), true);
        let par_y_e = par_x_e.get(&0).unwrap();
        assert_eq!(*par_y_e.get(&-3).unwrap(), true);
    }

    #[test]
    fn test_multiple() {
        let mut grid: XYZGrid = HashMap::new();
        flip_tiles(tokenize(FILE_CONTS.to_string()), &mut grid);
        assert_eq!(count_black_tiles(&grid), 10);
    }
}

#[cfg(test)]
mod test_conway{
    use crate::day24::test_flip::FILE_CONTS;
    use crate::day24::ConwayFloor;
    use crate::day24::tokenize;
    use crate::day24::flip_tiles;
    use crate::day24::HashMap;
    use crate::day24::XYZGrid;

    #[test]
    fn test_eg_grid(){
        let mut grid: XYZGrid = HashMap::new();
        flip_tiles(tokenize(FILE_CONTS.to_string()), &mut grid);
        let mut floor = ConwayFloor::from_grid(&grid);
        assert_eq!(floor.current_buffer()
        .iter()
        .filter(|f| *f.1)
        .count(), 10);

        assert_eq!(floor.ticks(1), 15);
        assert_eq!(floor.ticks(1), 12);
        assert_eq!(floor.ticks(1), 25);
        assert_eq!(floor.ticks(1), 14);
        assert_eq!(floor.ticks(1), 23);
        assert_eq!(floor.ticks(1), 28);
        assert_eq!(floor.ticks(1), 41);
        assert_eq!(floor.ticks(1), 37);
        assert_eq!(floor.ticks(1), 49);
        assert_eq!(floor.ticks(1), 37);
        assert_eq!(floor.ticks(10), 132);
        assert_eq!(floor.ticks(10), 259);
        assert_eq!(floor.ticks(10), 406);
        assert_eq!(floor.ticks(10), 566);
        assert_eq!(floor.ticks(10), 788);
        assert_eq!(floor.ticks(10), 1106);
        assert_eq!(floor.ticks(10), 1373);
        assert_eq!(floor.ticks(10), 1844);
        assert_eq!(floor.ticks(10), 2208);
    }
}
