use adventofcode2021::utils::{file, get_answerer, BoxResult};

fn main() -> BoxResult<()> {
	let nums = file::lines::<i64>("inputs/day1.txt")?;

	let mut answer = get_answerer();
	answer(part1(&nums));
	answer(part2(&nums));

	Ok(())
}

fn count_increases(nums: &Vec<i64>) -> usize {
	nums.windows(2).filter(|w| w[1] > w[0]).count()
}

fn part1(nums: &Vec<i64>) -> BoxResult<usize> {
	Ok(count_increases(nums))
}

fn part2(nums: &Vec<i64>) -> BoxResult<usize> {
	let sums = nums
		.windows(3)
		.map(|w| w.iter().sum())
		.collect::<Vec<i64>>();

	Ok(count_increases(&sums))
}
