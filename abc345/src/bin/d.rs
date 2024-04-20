struct Grid {
    h: usize,
    w: usize,
    pieces: Vec<(usize, usize, usize, usize)>,
}
impl Grid {
    fn new(h: usize, w: usize) -> Self {
        Self {
            h,
            w,
            pieces: Vec::new(),
        }
    }
    fn can_place(&self, i: usize, j: usize, a: usize, b: usize) -> bool {
        if i >= self.h || j >= self.w {
            return false;
        }
        if i + a - 1 >= self.h || j + b - 1 >= self.w {
            return false;
        }
        for (ii, jj, aa, bb) in self.pieces.iter() {
            // (i, j)
            if (*ii <= i && i < *ii + aa) && (*jj <= j && j < *jj + bb) {
                return false;
            }
            // (i, j + b - 1)
            if (*ii <= i && i < *ii + aa) && (*jj <= j + b - 1 && j + b - 1 < *jj + bb) {
                return false;
            }
            // (i + a - 1, j)
            if (*ii <= i + a - 1 && i + a - 1 < *ii + aa) && (*jj <= j && j < *jj + bb) {
                return false;
            }
            // (i + a - 1, j + b - 1)
            if (*ii <= i + a - 1 && i + a - 1 < *ii + aa)
                && (*jj <= j + b - 1 && j + b - 1 < *jj + bb)
            {
                return false;
            }
        }
        true
    }
    fn place(&mut self, i: usize, j: usize, a: usize, b: usize) {
        self.pieces.push((i, j, a, b));
    }
    fn delete(&mut self) {
        self.pieces.pop().unwrap();
    }
    fn is_ok(&self) -> bool {
        let mut grid = vec![vec![false; self.w]; self.h];
        for (i, j, a, b) in self.pieces.iter() {
            for ii in *i..*i + *a {
                for jj in *j..*j + *b {
                    grid[ii][jj] = true;
                }
            }
        }
        grid.iter().all(|row| row.iter().all(|e| *e))
    }
}

fn dfs(grid: &mut Grid, now: usize, pieces: &Vec<(usize, usize)>) -> bool {
    if now == pieces.len() {
        return grid.is_ok();
    }
    if dfs(grid, now + 1, pieces) {
        return true;
    }
    let (p0, p1) = pieces[now];
    for (a, b) in [(p0, p1), (p1, p0)] {
        if a - 1 <= grid.h && b - 1 <= grid.w {
            for i in 0..grid.h + 1 - a {
                for j in 0..grid.w + 1 - b {
                    if grid.can_place(i, j, a, b) {
                        grid.place(i, j, a, b);
                        if dfs(grid, now + 1, pieces) {
                            return true;
                        }
                        grid.delete();
                    }
                }
            }
        }
    }
    false
}

fn main() {
    proconio::input! {
        n: usize,
        h: usize,
        w: usize,
        pieces: [(usize, usize); n]
    }

    let mut cnt = 0;
    for (a, b) in pieces.iter() {
        cnt += a * b;
    }
    if cnt < h * w {
        println!("No");
        return;
    }
    
    let mut grid = Grid::new(h, w);
    if dfs(&mut grid, 0, &pieces) {
        println!("Yes");
    } else {
        println!("No");
    }
}
