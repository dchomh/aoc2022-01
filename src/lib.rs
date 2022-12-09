use std::error::Error;
use std::fs;

pub struct Config {
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let file_path = args[1].clone();

        Ok(Config { file_path })
    }
}

pub fn run(config: Config) -> Result<i64, Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
        
    let max = find_max(&contents);
    let three_sum = top_three_sum(&contents);
    
    println!("Max: {}", max);
    println!("Top three sum: {}", three_sum);

    Ok(max)
}

pub fn find_max<'a>(contents: &'a str) -> i64 {
    let totals = list_totals(contents);

    let max = totals.iter().rev().take(1).sum();
    return max;
}

pub fn top_three_sum(contents: &str) -> i64 {
    let totals = list_totals(contents);

    let sum: i64 = totals.iter().rev().take(3).sum();

    return sum;
}

pub fn list_totals(contents: &str) -> Vec<i64> {
    let mut totals: Vec<i64> = Vec::new();
    let mut current_sum = 0;

    let mut content = contents.lines().peekable();
    while let Some(line) = content.next() {
        if line == "" {
            totals.push(current_sum);
            current_sum = 0;
        } else if content.peek().is_none() {
            current_sum = current_sum + line.parse::<i64>().unwrap();
            totals.push(current_sum);
        } else {
            current_sum = current_sum + line.parse::<i64>().unwrap();
        }
    }

    totals.sort();

    return totals;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_max_test() {
        let contents = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        assert_eq!(24000, find_max(contents));
    }

    #[test]
    fn find_top_three_total_test() {
        let contents = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        assert_eq!(45000, top_three_sum(contents));
    }
}
