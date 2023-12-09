#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let gain1: Vec<i32> = vec![-5, 1, 5, 0, -7];
        assert_eq!(largest_altitude(gain1), 1);

        let gain2: Vec<i32> = vec![-4, -3, -2, -1, 4, 3, 2];
        assert_eq!(largest_altitude(gain2), 0);
    }
}

pub fn largest_altitude(gain: Vec<i32>) -> i32 {
    let mut altitude: Vec<i32> = vec![0; gain.len() + 1];
    for i in 0..gain.len() {
        altitude[i + 1] = altitude[i] + gain[i];
    }

    *altitude.iter().max().unwrap()
}
