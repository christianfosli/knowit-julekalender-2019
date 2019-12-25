fn main() {
    let alphabet = vec![2, 3, 5, 7, 11];
    let length = 217532235;
    let seq = alvesekvens(alphabet, length);
    println!("{}", seq.iter().filter(|&n| *n == 7).count() * 7);
}

fn alvesekvens(alphabet: Vec<u8>, length: usize) -> Vec<u8> {
    let mut seq = Vec::new();
    let mut index = 0;
    let mut push_cnt = alphabet[0];
    loop {
        let n = alphabet[index % alphabet.len()];
        for _ in 0..push_cnt {
            seq.push(n);
            if seq.len() == length {
                return seq;
            }
        }
        index += 1;
        push_cnt = seq[index];
    }
}

#[test]
fn test_alvesekvens() {
    let alphabet = vec![2, 3];
    assert_eq!(
        alvesekvens(alphabet, 16),
        vec![2, 2, 3, 3, 2, 2, 2, 3, 3, 3, 2, 2, 3, 3, 2, 2]
    );
}
