#![allow(dead_code, unused_variables, unused_mut)]

use std::fs::read_to_string;

fn print_plane(plane: &Vec<Vec<char>>) {
    println!();
    for y in 0..plane[0].len() {
        for x in plane {
            print!("{}", x[y]);
        }
        println!();
    }
    println!();
}

fn fold_it(plane: Vec<Vec<char>>, fold: (char, usize)) -> Vec<Vec<char>> {
    //dbg!(fold);
    //print_plane(&plane);
    let mut out_plane: Vec<Vec<char>> = vec![];
    let mut tmp_plane: Vec<Vec<char>> = vec![];
    let plane_size_x: usize = plane.len();
    let plane_size_y: usize = plane[0].len();
    //dbg!(plane_size_x, plane_size_y);
    if fold.0 == 'x' {
        let x_pos: usize = fold.1;
        //dbg!(x_pos);
        for x in 0..((plane.len() - 1) / 2) {
            //dbg!(y);
            let mut tmp_element: Vec<char> = vec![];

            for y in 0..plane[0].len() {
                //println!("{}:{}", x, y);
                //dbg!(plane[x][plane_size_y-y-1]);
                if plane[x][y] == '#' {
                    //dbg!("xy#");
                    tmp_element.push('#');
                } else if plane[plane_size_x - x - 1][y] == '#' {
                    //dbg!("xpsyy#");
                    tmp_element.push('#');
                } else {
                    tmp_element.push('.');
                }
            }

            tmp_plane.push(tmp_element);
        }
        //out_plane = vec![vec!['.'; plane_size_x]; (plane_size_y-1)/2];
        /*out_plane = vec![];
        for x in 0..tmp_plane[0].len() {
            let mut tmp_element: Vec<char> = vec![];
            for y in &tmp_plane {
                tmp_element.push(y[x]);
            }
            out_plane.push(tmp_element)
        }*/
        out_plane = tmp_plane.to_owned();
        //print_plane(&out_plane);
    } else if fold.0 == 'y' {
        let y_pos: usize = fold.1;
        //dbg!(y_pos);
        for y in 0..((plane[0].len() - 1) / 2) {
            //dbg!(y);
            let mut tmp_element: Vec<char> = vec![];

            for x in 0..plane.len() {
                //println!("{}:{}", x, y);
                //dbg!(plane[x][plane_size_y-y-1]);
                if plane[x][y] == '#' {
                    //dbg!("xy#");
                    tmp_element.push('#');
                } else if plane[x][plane_size_y - y - 1] == '#' {
                    //dbg!("xpsyy#");
                    tmp_element.push('#');
                } else {
                    tmp_element.push('.');
                }
            }

            tmp_plane.push(tmp_element);
        }
        //out_plane = vec![vec!['.'; plane_size_x]; (plane_size_y-1)/2];
        out_plane = vec![];
        for x in 0..tmp_plane[0].len() {
            let mut tmp_element: Vec<char> = vec![];
            for y in &tmp_plane {
                tmp_element.push(y[x]);
            }
            out_plane.push(tmp_element)
        }
        //print_plane(&out_plane);
    }
    return out_plane;
}

fn part1(input_file: String) {
    let mut answer: usize = 0;
    let plane_size_x: usize = 1311; //11
    let plane_size_y: usize = 895; //15
    let mut plane: Vec<Vec<char>> = vec![vec!['.'; plane_size_y]; plane_size_x];
    let mut points: Vec<Vec<usize>> = input_file.split("\n\n").collect::<Vec<&str>>()[0]
        .split("\n")
        .map(|l| {
            l.split(",")
                .map(|d| d.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();
    let mut folds_str: Vec<&str> = input_file
        .split("\n\n")
        .map(|x| x.trim())
        .collect::<Vec<&str>>()[1]
        .split("\n")
        .map(|l| l.split(" ").nth(2).unwrap())
        .collect();
    let mut folds: Vec<(char, usize)> = vec![];
    for fold in &folds_str {
        folds.push((
            fold.split("=").nth(0).unwrap().parse::<char>().unwrap(),
            fold.split("=").nth(1).unwrap().parse::<usize>().unwrap(),
        ));
    }
    //dbg!(&points);
    //dbg!(&folds);
    //print_plane(&plane);
    for point in points {
        plane[point[0]][point[1]] = '#';
    }
    //print_plane(&plane);
    for fold in folds {
        if fold.0 == 'x' {
            for x in 0..plane.len() {
                //dbg!(x);
                for y in 0..plane[0].len() {
                    //dbg!(y);
                    if x == fold.1 {
                        plane[x][y] = '|';
                    }
                }
            }
        } else if fold.0 == 'y' {
            for x in 0..plane.len() {
                //dbg!(x);
                for y in 0..plane[0].len() {
                    //dbg!(y);
                    if y == fold.1 {
                        plane[x][y] = '-';
                    }
                }
            }
        }
        //print_plane(&plane);
        plane = fold_it(plane.to_owned(), fold);
        //print_plane(&plane);
        //dbg!(plane);
        for x in plane {
            for y in x {
                if y == '#' {
                    answer += 1;
                }
            }
        }
        break;
    }
    println!("Answer: {}", answer);
}

fn part2(input_file: String) {
    let mut answer: usize = 0;
    let plane_size_x: usize = 1311; //11
    let plane_size_y: usize = 895; //15
    let mut plane: Vec<Vec<char>> = vec![vec!['.'; plane_size_y]; plane_size_x];
    let mut points: Vec<Vec<usize>> = input_file.split("\n\n").collect::<Vec<&str>>()[0]
        .split("\n")
        .map(|l| {
            l.split(",")
                .map(|d| d.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();
    let mut folds_str: Vec<&str> = input_file
        .split("\n\n")
        .map(|x| x.trim())
        .collect::<Vec<&str>>()[1]
        .split("\n")
        .map(|l| l.split(" ").nth(2).unwrap())
        .collect();
    let mut folds: Vec<(char, usize)> = vec![];
    for fold in &folds_str {
        folds.push((
            fold.split("=").nth(0).unwrap().parse::<char>().unwrap(),
            fold.split("=").nth(1).unwrap().parse::<usize>().unwrap(),
        ));
    }
    //dbg!(&points);
    //dbg!(&folds);
    //print_plane(&plane);
    for point in points {
        plane[point[0]][point[1]] = '#';
    }
    //print_plane(&plane);
    for fold in folds {
        if fold.0 == 'x' {
            for x in 0..plane.len() {
                //dbg!(x);
                for y in 0..plane[0].len() {
                    //dbg!(y);
                    if x == fold.1 {
                        plane[x][y] = '|';
                    }
                }
            }
        } else if fold.0 == 'y' {
            for x in 0..plane.len() {
                //dbg!(x);
                for y in 0..plane[0].len() {
                    //dbg!(y);
                    if y == fold.1 {
                        plane[x][y] = '-';
                    }
                }
            }
        }
        //print_plane(&plane);
        plane = fold_it(plane.to_owned(), fold);
        //print_plane(&plane);
        //dbg!(plane);
    }
    print_plane(&plane);
    /*for x in plane {
        for y in x {
            if y == '#' {
                answer += 1;
            }
        }
    }*/
    //println!("Answer: {}", answer);
}

fn main() {
    let part: i32 = 2;

    let input_file: String = read_to_string("input.txt").unwrap();

    if part == 1 {
        part1(input_file);
    } else if part == 2 {
        part2(input_file);
    }
}
