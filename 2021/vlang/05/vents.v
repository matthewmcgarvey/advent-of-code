module main

import os

struct Point {
	x int
	y int
}

fn get_range(x int, y int) []int {
	from := if x > y { y } else { x }
	to := if x > y { x } else { y }
	mut temp := []int{}
	for i in from .. to {
		temp << i
	}
	temp << to
	return temp
}

fn part_a(lines []string) {
	mut count := map[string]int{}
	for line in lines {
		str_range := line.split('->').map(it.trim_space().split(',').map(it.int()))
		begin_point := Point{str_range[0][0], str_range[0][1]}
		end_point := Point{str_range[1][0], str_range[1][1]}
		if (begin_point.x != end_point.x) && (begin_point.y != end_point.y) {
			continue
		}

		range := if begin_point.x != end_point.x {
			get_range(begin_point.x, end_point.x).map(Point{it, begin_point.y})
		} else {
			get_range(begin_point.y, end_point.y).map(Point{begin_point.x, it})
		}
		for point in range {
			temp := "${point.x},${point.y}"
			if !(temp in count) {
				count[temp] = 0
			}
			count[temp] += 1
		}
	}
	mut total := 0
	for _, val in count {
		if val > 1 {
			total += 1
		}
	}
	println('PART A: $total')
}

fn part_b(lines []string) {
	println('PART B:')
}

fn main() {
	lines := os.read_lines('./example_input.txt') ?
	part_a(lines)
	part_b(lines)
}
