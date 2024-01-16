use std::collections::VecDeque;
use std::io;

fn bfs(start_x: usize, start_y: usize, end_x: usize, end_y: usize, l: usize, matrix: &mut Vec<Vec<i32>>) -> i32 {
    let dx = [-1, 1, 2, 2, 1, -1, -2, -2];
    let dy = [2, 2, 1, -1, -2, -2, -1, 1];

    let mut q = VecDeque::new();
    q.push_back((start_x, start_y));

    while let Some((x, y)) = q.pop_front() {
        if x == end_x && y == end_y {
            return matrix[x][y] - 1;
        }

        for i in 0..8 {
            let nx = x as isize + dx[i];
            let ny = y as isize + dy[i];

            if nx >= 0 && ny >= 0 && nx < l as isize && ny < l as isize && matrix[nx as usize][ny as usize] == 0 {
                matrix[nx as usize][ny as usize] = matrix[x][y] + 1;
                q.push_back((nx as usize, ny as usize));
            }
        }
    }

    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let l: usize = input.trim().parse().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.trim().split_whitespace();
        let start_x: usize = iter.next().unwrap().parse().unwrap();
        let start_y: usize = iter.next().unwrap().parse().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.trim().split_whitespace();
        let end_x: usize = iter.next().unwrap().parse().unwrap();
        let end_y: usize = iter.next().unwrap().parse().unwrap();

        let mut matrix = vec![vec![0; l]; l];
        matrix[start_x][start_y] = 1;

        println!("{}", bfs(start_x, start_y, end_x, end_y, l, &mut matrix));
    }
}
