/*!
Helper math functions
*/
use rand::Rng;

/**
Generate a random number in a range, inclusive of the ceiling.

# Example

```
use albion_terminal_rpg::prelude::random_num;

let num: usize = random_num(1, 10);
```
*/
pub fn random_num(min: usize, max: usize) -> usize {
    let result: usize = rand::thread_rng().gen_range(min..=max);

    result
}

mod tests {

    #[test]
    fn rand_nums_out_of_bounds() {
        fn looper(numbers: &mut Vec<usize>, loops: usize) {
            let min: usize = 0;
            let max: usize = loops;

            let rand = super::random_num(min, max);

            if rand < min || rand > max {
                crate::panic_menu!("The random number generator went out of bounds.");
            }

            numbers.push(rand);

            if loops == 0 {
                return;
            }

            looper(numbers, loops - 1);
        }

        let loops: usize = 500;
        let mut numbers: Vec<usize> = vec![];

        looper(&mut numbers, loops);
        println!("{numbers:?}");
    }
}
