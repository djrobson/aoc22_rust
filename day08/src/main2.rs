
fn is_visible(x: usize, y: usize, grid: &Vec<Vec<u8>>) -> bool {
    let max = grid.len();
    let my_height = grid[y][x];
    let mut block_left = false;
    let mut block_right = false;
    let mut block_up = false;
    let mut block_down = false;

    if x == 0 || x == max || y == 0 || y == max {
        return true;
    }

    for left in 0..x {
        if grid[y][left] >= my_height {
            block_left = true;
            break;
        }
    }
    for right in x+1..max {
        if grid[y][right] >= my_height {
            block_right = true;
            break;
        }
    }
    for up in 0..y {
        if grid[up][x] >= my_height {
            block_up = true;
            break;
        }
    }
    for down in y+1..max {
        if grid[down][x] >= my_height {
            block_down = true;
            break;
        }
    }

    !(block_left && block_right && block_up && block_down)
}
fn get_visible(x: usize, y: usize, grid: &Vec<Vec<u8>>) -> usize {
    let max = grid.len();
    let my_height = grid[y][x];
    let mut count_left = 0;
    let mut count_right = 0;
    let mut count_up = 0;
    let mut count_down = 0;

    if x==0 || x== max || y == 0 || y == max {
        return 0;
    }

    // 0 1 2 3 4 x 6 7 8 9
    let mut left = x;
    while left > 0
    {
        count_left = count_left +1;
        if grid[y][left-1] >= my_height {
            break;
        }
        left = left -1;
    }

    let mut right = x+1;
    while right < max
    {
        count_right = count_right +1;
        if grid[y][right] >= my_height {
            break;
        }
        right = right +1;
    }
    
    let mut up = y;
    while up > 0
    {
        count_up = count_up +1;
        if grid[up-1][x] >= my_height {
            break;
        }
        up = up -1;
    }

    let mut down = y+1;
    while down < max
    {
        count_down = count_down +1;
        if grid[down][x] < my_height {
            break;
        }
        down = down +1;
    }

    return count_down * count_up * count_left * count_right;
}

fn main() {
    let f = include_str!("../sample.txt");
    //let f = include_str!("../input08.txt");
    let mut grid: Vec<Vec<u8>> = Vec::new();

    for line in f.lines() {
        grid.push(line.as_bytes().to_vec());
    }
    //println!("{:?}", grid);

    let mut count = 0;
    //let edge = grid.len();

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if is_visible(x, y, &grid) {
                count = count +1;
            }
        }
    }
    println!("outside visible trees {}", count);

    let mut max_v = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let v =  get_visible(x, y, &grid);
            //println!("{} {} {}", x, y, v);
            if v > max_v
            {
                max_v = v;
            }
        }
    }
    println!("inside visible trees {}", max_v);

}