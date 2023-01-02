

fn partial_sum_p200_to_p100(p200: u32, p100: u32) -> u32 {
    p200 * 200 + p100 * 100
}

fn partial_sum_p200_to_p50(p200: u32, p100: u32, p50: u32) -> u32 {
    p200 * 200 + p100 * 100 + p50 * 50
}

fn partial_sum_p200_to_p20(p200: u32, p100: u32, p50: u32, p20: u32) -> u32 {
    p200 * 200 + p100 * 100 + p50 * 50 + p20 * 20
}

fn partial_sum_p200_to_p10(p200: u32, p100: u32, p50: u32, p20: u32, p10: u32) -> u32 {
    p200 * 200 + p100 * 100 + p50 * 50 + p20 * 20 + p10 * 10
}

fn partial_sum_p200_to_p5(p200: u32, p100: u32, p50: u32, p20: u32, p10: u32, p5: u32) -> u32 {
    p200 * 200 + p100 * 100 + p50 * 50 + p20 * 20 + p10 * 10 + p5 * 5
}


fn partial_sum_p200_to_p2(p200: u32, p100: u32, p50: u32, p20: u32, p10: u32, p5: u32, p2: u32) -> u32 {
    p200 * 200 + p100 * 100 + p50 * 50 + p20 * 20 + p10 * 10 + p5 * 5 + p2 * 2
}


fn partial_sum_p200_to_p1(p200: u32, p100: u32, p50: u32, p20: u32, p10: u32, p5: u32, p2: u32, p1: u32) -> u32 {
    p200 * 200 + p100 * 100 + p50 * 50 + p20 * 20 + p10 * 10 + p5 * 5 + p2 * 2 + p1 * 1
}


fn main() {
    
    let mut ways: Vec<Vec<u32>> = vec![];


    for p200 in 0..=1 {

        for p100 in 0..=2 {

            if partial_sum_p200_to_p100(p200, p100) > 200 {
                break
            }

            for p50 in 0..=4 {

                if partial_sum_p200_to_p50(p200, p100, p50) > 200 {
                    break
                }

                for p20 in 0..=10 {

                    if partial_sum_p200_to_p20(p200, p100, p50, p20) > 200 {
                        break
                    }

                    for p10 in 0..=20 {

                        if partial_sum_p200_to_p10(p200, p100, p50, p20, p10) > 200 {
                            break
                        }

                        for p5 in 0..=40 {

                            if partial_sum_p200_to_p5(p200, p100, p50, p20, p10, p5) > 200 {
                                break
                            }

                            for p2 in 0..=100 {

                                if partial_sum_p200_to_p2(p200, p100, p50, p20, p10, p5, p2) > 200 {
                                    break
                                }

                                for p1 in 0..=200 {

                                    let s = partial_sum_p200_to_p1(p200, p100, p50, p20, p10, p5, p2, p1);
                                    if s > 200 {
                                        break;
                                    } else if s == 200 {
                                        let way = vec![p200, p100, p50, p20, p10, p5, p2, p1];
                                        println!("........ {:?}", way);
                                        ways.push(way);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Number of ways = {}", ways.len());

}
