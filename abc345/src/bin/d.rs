#[derive(Debug, Clone)]
struct Grid {
    h: usize,
    w: usize,
    grid: Vec<Vec<bool>>
}
impl Grid {
    fn new(h: usize, w: usize) -> Self {
        Self {
            h,
            w,
            grid: vec![vec![false; w]; h]
        }
    }
    fn can_place(&self, a: usize, b: usize, i: usize, j: usize) -> bool {
        if i + a > self.h || j + b > self.w {
            return false;
        }
        for ii in i..i + a {
            for jj in j..j + b {
                if self.grid[ii][jj] {
                    return false;
                }
            }
        }
        true
    }
    fn place(&mut self, a: usize, b: usize, i: usize, j: usize) {
        for ii in i..i + a {
            for jj in j..j + b {
                self.grid[ii][jj] = true;
            }
        }
    }
    fn delete(&mut self, a: usize, b: usize, i: usize, j: usize) {
        for ii in i..i + a {
            for jj in j..j + b {
                self.grid[ii][jj] = false;
            }
        }
    }
    fn is_ok(&self) -> bool {
        for i in 0..self.h {
            for j in 0..self.w {
                if !self.grid[i][j] {
                    return false;
                }
            }
        }
        true
    }
}

fn dfs(grid: &mut Grid, now: usize, pieces: &Vec<(usize, usize)>) -> bool {
    // for row in grid.grid.iter() {
    //     eprintln!("{:?}", row.iter().map(|e| if *e { "1" } else { "0" }).join(""));
    // }
    // eprintln!("");
    if grid.is_ok() {
        return true;
    }
    if now == pieces.len() {
        return false;
    }
    let (a, b) = pieces[now];
    if a <= grid.h && b <= grid.w {
        for i in 0..grid.h+1-a {
            for j in 0..grid.w+1-b {
                if grid.can_place(a, b, i, j) {
                    grid.place(a, b, i, j);
                    if dfs(grid, now + 1, pieces) {
                        return true;
                    }
                    grid.delete(a, b, i, j);
                }
            }
        }
    }
    if b <= grid.h && a <= grid.w {
        for i in 0..grid.h+1-b {
            for j in 0..grid.w+1-a {
                if grid.can_place(b, a, i, j) {
                    grid.place(b, a, i, j);
                    if dfs(grid, now + 1, pieces) {
                        return true;
                    }
                    grid.delete(b, a, i, j);
                }
                
            }
        }
    }
    dfs(grid, now + 1, pieces)
}

fn main() {
    proconio::input! {
        n: usize,
        h: usize,
        w: usize,
        pieces: [(usize, usize); n]
    }
    let mut grid = Grid::new(h, w);
    if dfs(&mut grid, 0, &pieces) {
        println!("Yes");
    } else {
        println!("No");
    }
}
