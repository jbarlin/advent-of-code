use crate::AoCDay;

type Cups = Vec<u32>;

pub struct Code;

impl AoCDay for Code {

    fn part1(&self, _input: &mut dyn std::io::Read, _extra_argss: &[String]) -> String {
        let mut cups = init(9);
        run_moves(&mut cups, 100, 0);
        return collect_cups_after_1(cups);
    }

    fn part2(&self, _input: &mut dyn std::io::Read, _extra_args: &[String]) -> String {
        let mut cups = init(1000000);
        run_moves(&mut cups, 10000000, 0);
        return get_two_after_1(cups);
    }
}

fn init(array_len: u32) -> Cups{
    let mut arr: Cups = (0..array_len).collect();
    for i in 1..arr.len() {
        arr[i - 1] += 1;
    }
    arr.shrink_to_fit();
    arr[0] = 5;
    arr[5] = 6;
    arr[6] = 1;
    arr[1] = 3;
    arr[3] = 7;
    arr[7] = 2;
    arr[2] = 4;
    arr[4] = 8;
    if array_len == 9 {
        arr[8] = 0;
    }else{
        arr[8] = 9;
        arr[(array_len as usize) - 1] = 0;
    }
    return arr;
}

fn perform_move(cups: &mut Cups, current_cup: u32) -> u32{
    let mut vls: Cups = Vec::new();
    let mut active_cup = cups[current_cup as usize];
    vls.push(active_cup);
    vls.push(cups[active_cup as usize]); // So, the next value in the chain
    active_cup = vls[1];
    vls.push(cups[active_cup as usize]);
    active_cup = vls[2];
    let next_cup = cups[active_cup as usize];
    // OK, update the original links?
    let mut selector_cup = (if current_cup != 0 { current_cup} else {cups.len() as u32}) - 1;
    while vls.contains(&(selector_cup as u32)) {
        if selector_cup == 0 {
            selector_cup += cups.len() as u32;
        }
        selector_cup -= 1;
    }
    cups[current_cup as usize] = vls[2];
    let tmp_cup = cups[selector_cup as usize];
    cups[vls[2] as usize] = tmp_cup;
    cups[selector_cup as usize] = vls[0];
    cups[current_cup as usize] = next_cup;
    return next_cup;
}

fn run_moves(cups: &mut Cups, num_moves: u128, start_cup: u32) -> u32{
    let mut itr: u128 = 0;
    let mut current_cup = start_cup;
    while itr < num_moves {
        current_cup = perform_move(cups, current_cup);
        itr += 1;
    }
    return current_cup;
}

fn collect_cups_after_1(cups: Cups) -> String{
    let mut cups_list = String::new();
    let mut next_cup = 0;
    loop{
        next_cup = cups[next_cup as usize];
        if next_cup == 0 {
            break;
        }
        cups_list.push_str(&(next_cup + 1).to_string());
    }
    return cups_list;
}

fn get_two_after_1(cups: Cups) -> String{
    let first_num = cups[0];
    let second_num = cups[first_num as usize];
    let final_mult: usize = ((first_num + 1) as usize) * ((second_num + 1) as usize);
    return final_mult.to_string();
}

#[cfg(test)]
mod tests_all{
    use crate::day23::get_two_after_1;
    use crate::day23::collect_cups_after_1; 
    use crate::day23::run_moves;
    use crate::day23::Cups;
    use super::perform_move;

    fn get_test_cups() -> Cups{
        let mut arr: Cups = (0..9).collect();
        arr[2] = 7;
        arr[7] = 8;
        arr[8] = 0;
        arr[0] = 1;
        arr[1] = 4;
        arr[4] = 3;
        arr[3] = 5;
        arr[5] = 6;
        arr[6] = 2;
        return arr;
    }

    #[test]
    fn test_one_move(){
        let mut arr = get_test_cups();
        let curr_cup = 2 as u32;
        let next_cup = perform_move(&mut arr, curr_cup);
        assert_eq!(next_cup, 1);
        assert_eq!(arr[curr_cup as usize], 1);
        assert_eq!(arr[next_cup as usize], 7);
        assert_eq!(arr[7], 8);
        assert_eq!(arr[8], 0);
        assert_eq!(arr[3], 5);
        assert_eq!(arr[2], 1);
    }
    #[test]
    fn test_three_moves(){
        let mut arr = get_test_cups();
        let curr_cup = 2 as u32;
        let next_cupa = perform_move(&mut arr, curr_cup);
        let next_cupb = perform_move(&mut arr, next_cupa);
        let next_cupc = perform_move(&mut arr, next_cupb);
        assert_eq!(next_cupc, 7);
        assert_eq!(arr[next_cupc as usize], 8);
        assert_eq!(arr[5], 6);
        assert_eq!(arr[6], 1);
        assert_eq!(arr[1], 4);
        assert_eq!(arr[4], 7);
        assert_eq!(arr[8], 0);
        assert_eq!(arr[0], 2);
        assert_eq!(arr[2], 3);
        assert_eq!(arr[3], 5);
    }

    #[test]
    fn test_10_moves(){
        let mut arr = get_test_cups();
        let curr_cup = 2 as u32;
        let next_cup = run_moves(&mut arr, 10, curr_cup);
        assert_eq!(next_cup, 7);
    }

    #[test]
    fn test_cups_list(){
        let mut arr = get_test_cups();
        let curr_cup = 2 as u32;
        let next_cup = run_moves(&mut arr, 10, curr_cup);
        let cups_list = collect_cups_after_1(arr);
        assert_eq!(cups_list, "92658374");
    }

    #[test]
    fn test_cups_list_100(){
        let mut arr = get_test_cups();
        let curr_cup = 2 as u32;
        let next_cup = run_moves(&mut arr, 100, curr_cup);
        let cups_list = collect_cups_after_1(arr);
        assert_eq!(cups_list, "67384529");
    }

    #[test]
    fn test_million_run(){
        let mut arr: Cups = (0..1000000).collect();
        let curr_cup = 2 as u32;
        for i in 1..arr.len() {
            arr[i - 1] += 1;
        }
        arr[curr_cup as usize] = 7;
        arr[7] = 8;
        arr[8] = 0;
        arr[0] = 1;
        arr[1] = 4;
        arr[4] = 3;
        arr[3] = 5;
        arr[5] = 6;
        arr[6] = 9;
        arr[(1000000 - 1)] = curr_cup;
        run_moves(&mut arr, 10000000, curr_cup);
        assert_eq!(arr[0], (934001 -1));
        assert_eq!(arr[934001 -1], (159792 -1));
        assert_eq!(get_two_after_1(arr),"149245887792");
    }
}