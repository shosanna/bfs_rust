#![feature(vec_resize)]
use std::collections::VecDeque;

fn print_map(map:&Vec<char>) {
    for i in 0..10 {
        for j in 0..10 {
            print!("{}", map[10*i + j]);
        }
        println!("");
    }
    println!("");
}

fn main() {
    let mut mapik = Vec::new();
    mapik.resize(100, '.');

    print_map(&mapik);

    let mut queue = VecDeque::new();

    let mut path: Vec<(i32, i32)> = Vec::new();

    mapik[0] = 'o';
    path.push((0,0));

    mapik[15] = '@';

    queue.push_back(path);

    'mimo: while !queue.is_empty() {
        std::thread::sleep_ms(20);
        let curr_path = queue.pop_front().unwrap();
        let curr_coords = curr_path[curr_path.len() - 1];
        let curr_index = curr_coords.0 * 10 + curr_coords.1;
        let curr_item = mapik[curr_index as usize].clone();

        if curr_item == 'z' { continue };
        if curr_item == '.' { panic!(". not supposed to be in queue")};
        if curr_item == 'o' {
            mapik[curr_index as usize] = 'z';

            for i in (curr_coords.0 - 1)..(curr_coords.0 + 2) {
                for j in (curr_coords.1 - 1)..(curr_coords.1 + 2) {
                    if i >= 0 && i < 10 && j >= 0 && j < 10 {
                        let next_index = i*10 +j;

                        if mapik[next_index as usize] == '@' {
                            print_map(&mapik);
                            for breadcrumb in &curr_path {
                                println!("{} {}", breadcrumb.1, breadcrumb.0);
                            }
                            break 'mimo;
                        }


                        if mapik[next_index as usize] == '.' {
                            mapik[next_index as usize] = 'o';

                            let mut next_path = curr_path.clone();
                            next_path.push((i,j));
                            queue.push_back(next_path); 
                        }
                    }
                }
            }
        }

        print_map(&mapik);
    }
}
