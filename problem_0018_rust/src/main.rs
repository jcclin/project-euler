const DATA: &str = "
75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23
";

#[derive(Clone,Debug)]
struct MyCoord {
    r: usize,
    c: usize,
}


#[derive(Debug)]
struct MyPath {
    path: Vec<MyCoord>,
    sum_so_far: i32,
}


fn main() {

    let triangle: Vec<Vec<i32>> = DATA.split("\n")
    .filter(|r| r.len() > 0)
    .map(|r| { r
        .split(' ')
        .map(|f| f.parse::<i32>().unwrap() )
        .collect() 
    } )
    .collect();

    let depth: usize = triangle.len();
    let mut paths: Vec<MyPath> = vec![MyPath{ path: vec![MyCoord{ r: 0, c: 0 }], sum_so_far: 75 }];
    for d in 1..depth {
        let mut next_level_paths: Vec<MyPath> = vec![];
        for p in paths {

            let coord = p.path.last().unwrap();

            assert_eq!(coord.r, d - 1);
            let coord_l = MyCoord { r: coord.r + 1, c: coord.c };
            let coord_r = MyCoord { r: coord.r + 1, c: coord.c + 1 };

            let value_l = triangle.get(coord_l.r).unwrap().get(coord_l.c).unwrap();
            let value_r = triangle.get(coord_r.r).unwrap().get(coord_r.c).unwrap();

            let mut path_l = p.path.clone();
            path_l.push(coord_l);
            let sum_so_far_l = p.sum_so_far + value_l;
            
            let mut path_r = p.path.clone();
            path_r.push(coord_r);
            let sum_so_far_r = p.sum_so_far + value_r;

            let Path_l = MyPath { path: path_l, sum_so_far: sum_so_far_l };
            let Path_r = MyPath { path: path_r, sum_so_far: sum_so_far_r };

            next_level_paths.push(Path_l);
            next_level_paths.push(Path_r);
        }
        paths = next_level_paths;
    }

    for d in 0..depth {
        println!("{:?}", triangle.get(d));
    }

    /*for p in paths
    {
        println!("{:?}", p);
    }
    */

    let max_sum_path = paths
        .into_iter()
        .max_by(|a, b| (a.sum_so_far).cmp(&(b.sum_so_far)) ).unwrap();
    println!("max_sum_path = {:?}", max_sum_path);
}
