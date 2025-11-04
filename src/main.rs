use std::{
    fmt::Display,
    ops::{Index, IndexMut},
    thread::sleep,
    time::Duration,
};

use rand::Rng;

fn main() {
    const STEPS: usize = 100;
    const M: usize = 10;
    const N: usize = 10;

    let mut game = Game::<M, N>::new_random();

    println!("{game}");
    for _ in 0..STEPS {
        game.update();
        print!("\x1B[2J\x1B[1;1H");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        println!("{game}");
        sleep(Duration::from_millis(250));
    }
}

#[derive(Debug)]
struct Grid<const M: usize, const N: usize> {
    inner: [[bool; N]; M],
}

impl<const M: usize, const N: usize> Index<(usize, usize)> for Grid<M, N> {
    type Output = bool;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.inner[i][j]
    }
}

impl<const M: usize, const N: usize> IndexMut<(usize, usize)> for Grid<M, N> {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        &mut self.inner[i][j]
    }
}

impl<const M: usize, const N: usize> Default for Grid<M, N> {
    fn default() -> Self {
        Self {
            inner: [[false; N]; M],
        }
    }
}

impl<const M: usize, const N: usize> Grid<M, N> {
    fn random() -> Self {
        let mut grid = Self::default();

        for i in 0..M {
            for j in 0..N {
                grid[(i, j)] = rand::rng().random();
            }
        }

        grid
    }

    fn iter_cells(&self) -> impl Iterator<Item = ((usize, usize), bool)> {
        (0..M).flat_map(move |i| (0..N).map(move |j| ((i, j), self[(i, j)])))
    }

    fn neighbors(&self, (i, j): (usize, usize)) -> impl Iterator<Item = bool> {
        const DELTAS: [[isize; 2]; 8] = [
            [-1, -1],
            [-1, 0],
            [-1, 1],
            [0, -1],
            [0, 1],
            [1, -1],
            [1, 0],
            [1, 1],
        ];

        let (i, j) = (i as isize, j as isize);

        DELTAS.into_iter().filter_map(move |[di, dj]| {
            let new_i = i + di;
            if !(0..M as isize).contains(&new_i) {
                return None;
            }
            let new_j = j + dj;
            if !(0..N as isize).contains(&new_j) {
                return None;
            }
            Some(self[(new_i as usize, new_j as usize)])
        })
    }

    fn clear(&mut self) {
        todo!()
    }
}

#[derive(Debug)]
struct Game<const M: usize, const N: usize> {
    cur: Grid<M, N>,
    next: Grid<M, N>,
}

impl<const M: usize, const N: usize> Display for Game<M, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const DEAD: char = '░';
        const ALIVE: char = '▓';

        for i in 0..M {
            let row: String = self.cur.inner[i]
                .iter()
                .flat_map(|&x| if x { [ALIVE, ALIVE] } else { [DEAD, DEAD] })
                .collect();
            writeln!(f, "{row}")?;
        }

        Ok(())
    }
}

impl<const M: usize, const N: usize> Game<M, N> {
    fn new(start_grid: Grid<M, N>) -> Self {
        Self {
            cur: start_grid,
            next: Grid::default(),
        }
    }

    fn new_random() -> Self {
        Self::new(Grid::random())
    }

    fn update(&mut self) {
        for (coord, status) in self.cur.iter_cells() {
            let alive_cnt = self.cur.neighbors(coord).filter(|&value| value).count();
            match (status, alive_cnt) {
                (true, cnt) if cnt < 2 || cnt > 3 => {
                    self.next[coord] = false;
                }
                (false, 3) => {
                    self.next[coord] = true;
                }
                _ => {
                    self.next[coord] = self.cur[coord];
                }
            }
        }
        std::mem::swap(&mut self.cur, &mut self.next);
    }
}
