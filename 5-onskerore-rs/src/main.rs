
fn main() {
    let wishlist = "tMlsioaplnKlflgiruKanliaebeLlkslikkpnerikTasatamkDpsdakeraBeIdaegptnuaKtmteorpuTaTtbt\
    sesOHXxonibmksekaaoaKtrssegnveinRedlkkkroeekVtkekymmlooLnanoKtlstoepHrpeutdynfSneloietbol";
    println!("{}",
        cut_in_half_and_swap(
            &swap_every_other(
                &swap_first_three_last_tree_and_so_on(wishlist))));
}

fn cut_in_half_and_swap(input: &str) -> String {
    let (first, last) = input.split_at(input.len()/2);
    let switched = [last, first].concat();
    return switched;
}

fn swap_every_other(input: &str) -> String {
    let mut swapped = String::from("");
    let mut iter = input.chars().peekable();
    while iter.peek().is_some() {
        let first = iter.next();
        let sec = iter.next();
        match (first, sec) {
            (Some(first), Some(sec)) => {
                swapped.push(sec);
                swapped.push(first);
            },
            _ => (),
        };
    }
    return swapped;
}

fn swap_first_three_last_tree_and_so_on(input: &str) -> String {
    let mut new_front = String::from("");
    let mut new_back = String::from("");
    let mut iter = input.chars().peekable();
    let mut cnt = 0;
    while iter.peek().is_some() && cnt < input.len() {
        let chunk: String = iter.by_ref().take(3).collect();
        new_front = [chunk, new_front].concat();
        cnt += 3;
    }
    while iter.peek().is_some() && cnt >= input.len() {
        let chunk: String = iter.by_ref().take(3).collect();
        new_back = [new_back, chunk].concat();
        cnt += 3;
    }
    return [new_front, new_back].concat();
}

#[test]
fn test_cut_in_half_and_swap() {
    let original = "PonnistallHoppeslottTrommesett";
    assert_eq!(cut_in_half_and_swap(original), "slottTrommesettPonnistallHoppe");
}

#[test]
fn test_swap_every_other() {
    let original = "slottTrommesettPonnistallHoppe";
    assert_eq!(swap_every_other(original), "lstoTtormmsetePtnointslaHlpoep");
}

#[test]
fn test_swap_first_three_last_three_and_so_on() {
    let original = "lstoTtormmsetePtnointslaHlpoep";
    assert_eq!(swap_first_three_last_tree_and_so_on(original), "oepHlpslainttnotePmseormoTtlst")
}
