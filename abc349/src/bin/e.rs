#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Color {
    Red,
    Blue,
    White,
}

use Color::{Blue, Red, White};

type Board = Vec<Vec<Color>>;

fn check(bd: &Board, score: &Vec<Vec<i32>>) -> Color {
    for i in 0..3 {
        if bd[i][0] != White && bd[i][0] == bd[i][1] && bd[i][1] == bd[i][2] {
            return bd[i][0];
        }
        if bd[0][i] != White && bd[0][i] == bd[1][i] && bd[1][i] == bd[2][i] {
            return bd[0][i];
        }
    }
    if bd[0][0] != White && bd[0][0] == bd[1][1] && bd[1][1] == bd[2][2] {
        return bd[0][0];
    }
    if bd[0][2] != White && bd[0][2] == bd[1][1] && bd[1][1] == bd[2][0] {
        return bd[0][2]
    }
    let mut takahashi = 0;
    let mut aoki = 0;
    for i in 0..3 {
        for j in 0..3 {
            match bd[i][j] {
                Red => takahashi += score[i][j],
                Blue => aoki += score[i][j],
                White => return White,
            }
        }
    }
    if takahashi > aoki {
        Red
    } else {
        Blue
    } 
}

fn dfs(bd: Board, score: &Vec<Vec<i32>>, turn: Color) -> Color {
    println!("{:#?}", bd);
    let c = check(&bd, score);
    if c != White {
        return c;
    }
    for i in 0..3 {
        for j in 0..3 {
            if bd[i][j] == White {
                let mut new_bd = bd.clone();
                new_bd[i][j] = turn;
                if dfs(new_bd, score, match turn {
                    Red => Blue,
                    Blue => Red,
                    White => unreachable!(),
                }) == turn {
                    return turn;
                }           
            }
        }
    }
    match turn {
        Red => Blue,
        Blue => Red,
        White => unreachable!(),
    }
}

fn main() {
    proconio::input! {
        score: [[i32; 3]; 3]
    }
    let bd = vec![vec![White; 3]; 3];
    match dfs(bd, &score, Red) {
        Red => println!("Takahashi"),
        Blue => println!("Aoki"),
        White => unreachable!(),
    }
}
