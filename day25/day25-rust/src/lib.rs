fn find_loop_size(pub_key: u64) -> usize {
    let mut value = 7;
    let mut counter = 1;
    loop {
        if value == pub_key {
            return counter
        }
        value = value * 7 % 20201227;
        counter += 1
    }
}

fn find_encryption_key(subject_number: u64, loop_size: usize) -> u64 {
    let mut value = 1;
    for _ in 0..loop_size {
        value = value * subject_number % 20201227;
    }
    value
}

pub fn solution(door_pub_key: u64, lock_pub_key: u64) -> u64 {
    find_encryption_key(lock_pub_key, find_loop_size(door_pub_key))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_loop_size() {
        assert_eq!(find_loop_size(5764801), 8);
        assert_eq!(find_loop_size(17807724), 11);
    }

    #[test]
    fn test_find_encryption_key() {
        assert_eq!(find_encryption_key(17807724, 8), 14897079);
        assert_eq!(find_encryption_key(5764801, 11), 14897079);
    }
}
