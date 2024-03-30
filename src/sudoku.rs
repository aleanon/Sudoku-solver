

#[derive(Debug, Clone)]
pub struct Sudoku { 
    pub size: u8,
    pub puzzle: Vec<Vec<u8>>,
    pub conflicts: Vec<(usize, usize)>,
}

impl<'a> Sudoku {
    pub fn new(size: u8) -> Self {
        Self{
            size,
            puzzle: vec![vec![0;size as usize];size as usize],
            conflicts: Vec::new(),
        }
    }

    pub fn from_puzzle(puzzle: Vec<Vec<u8>>) -> Self {
        assert!(puzzle.len() <= u8::MAX as usize);
        let mut sudoku = Self {
            size: puzzle.len() as u8,
            puzzle: puzzle,
            conflicts: Vec::new(),
        };

        if let Some(conflicts) = sudoku.validate() {
            sudoku.conflicts = conflicts;
        }
        sudoku
    }

    pub fn solve(&self) -> Option<Vec<Vec<u8>>> {
        let nmax = (self.size - 1) as usize;
        let nmax_sqr = (self.size as f32).sqrt() as usize;
        let mut puzzle = self.puzzle.clone();

        Self::solver(&mut puzzle, nmax, nmax_sqr, (0,0)).then(||puzzle)
    }

    fn solver(puzzle: &mut Vec<Vec<u8>>, nmax:usize,nmax_sqr: usize, coord @ (x,y):(usize,usize)) -> bool {
    
        if x == nmax + 1 {
        return true
        } 

        if puzzle[x][y] == 0 {
            //Determines the upper left corner coordinates of whatever square you are in
            let (sx,sy) = (x - (x % nmax_sqr), y - (y % nmax_sqr));  

            'Increment: for n in 1..=nmax + 1 {
                let n = n as u8;

                //validate current row and column for number "n"
                for i in 0..=nmax {
                    if (i,y) != coord {
                    if puzzle[i][y] == n {
                        continue 'Increment;
                    }
                    }
                    if (x,i) != coord {
                    if puzzle[x][i] == n {
                        continue 'Increment;
                    }
                    }
                }
                    
                //Validate the square you are in for number "n"
                for i in 0..nmax_sqr {
                    for ii in 0..nmax_sqr {
                    if (sx + i, sy + ii) != coord {
                        if puzzle[sx + i][sy + ii] == n {
                        continue 'Increment
                        }
                    }
                    }
                }
                puzzle[x][y] = n as u8;
                    
                if y == nmax {
                    if Self::solver(puzzle, nmax,nmax_sqr, (x + 1, 0)) {
                    return true
                    }
                } else if Self::solver(puzzle, nmax, nmax_sqr, (x, y + 1)) {
                    return true
                }
            // Continues to try the next number for the current position
            }

        } else {
        if y == nmax {
            return Self::solver(puzzle, nmax, nmax_sqr, (x + 1, 0))
        }
        return Self::solver(puzzle, nmax, nmax_sqr, (x, y + 1)) 
        } 
        puzzle[x][y] = 0;
        false
    }

    pub fn validate(&self) -> Option<Vec<(usize, usize)>> {
        let nmax = (self.size - 1) as usize;
        let nmax_sqr = (self.size as f32).sqrt() as usize;
        let mut conflicts:Vec<(usize, usize)> = Vec::new();

        Self::validator(&self.puzzle, &mut conflicts, nmax, nmax_sqr, (0,0));

        if conflicts.len() == 0 {None} else {Some(conflicts)}        
    }

    fn validator(puzzle: &Vec<Vec<u8>>, conflicts: &mut Vec<(usize, usize)>, nmax: usize, nmax_sqr: usize, coord@(x, y): (usize, usize)) -> () {
        if x == nmax + 1 {
        return ()
        } 
          
        //Determines the upper left corner coordinates of whatever square you are in
        let (sx,sy) = (x - (x % nmax_sqr), y - (y % nmax_sqr));  

        let n = puzzle[x][y];
        if n != 0 {
            //validate current row and column for number "n"
            for i in 0..=nmax {
                if (i,y) != coord {
                    if puzzle[i][y] == n {
                        conflicts.push((i, y))
                    }
                }
                if (x,i) != coord {
                    if puzzle[x][i] == n {
                        conflicts.push((x,i))
                    }
                }
            }
                
            //Validate the square you are in for number "n"
            for i in 0..nmax_sqr {
                for ii in 0..nmax_sqr {
                if (sx + i, sy + ii) != coord {
                    if puzzle[sx + i][sy + ii] == n {
                        conflicts.push((sx + i, sy + ii))
                    }
                }
                }
            }
        }

        if y == nmax {
            Self::validator(puzzle, conflicts, nmax,nmax_sqr, (x + 1, 0)) 
        } else {
            Self::validator(puzzle, conflicts, nmax, nmax_sqr, (x, y + 1))
        }        
    }
}
