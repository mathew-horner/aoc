type Report = Vec<Level>;
type Level = i32;

fn parse(input: crate::Input) -> Vec<Report> {
    input
        .read_lines()
        .map(|line| {
            line.split(' ')
                .map(|s| s.trim().parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

pub fn part1(input: crate::Input) -> usize {
    parse(input).into_iter().filter(is_safe).count()
}

fn is_safe(report: &Report) -> bool {
    (report.is_sorted() || report.iter().rev().is_sorted()) && all_level_deltas_in_range(report)
}

fn all_level_deltas_in_range(report: &Report) -> bool {
    (1..report.len())
        .map(|idx| (report[idx] - report[idx - 1]).abs())
        .all(|delta| (1..=3).contains(&delta))
}

pub fn part2(input: crate::Input) -> usize {
    parse(input)
        .into_iter()
        .filter_map(|report| is_safe_besides_one(report).then_some(()))
        .count()
}

fn is_safe_besides_one(report: Report) -> bool {
    (0..report.len()).into_iter().any(|idx| {
        let (left, right) = report.split_at(idx);
        let reduced = [left, &right[1..]].concat();
        is_safe(&reduced)
    })
}
