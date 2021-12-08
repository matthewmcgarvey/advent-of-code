module main

import os

struct Point {
	x int
	y int
}

fn advance_point(a int, b int) int {
	if a == b {
		return a
	} else if a > b {
		return a - 1
	} else {
		return a + 1
	}
}

fn move_one(from Point, to Point) Point {
	new_x := advance_point(from.x, to.x)
	new_y := advance_point(from.y, to.y)

	return Point{new_x, new_y}
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

		mut range := []Point{}
		range << begin_point
		for {
			last := range.last()
			next := move_one(last, end_point)
			if last == next {
				break
			} else {
				range << next
			}
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
	mut count := map[string]int{}
	for line in lines {
		str_range := line.split('->').map(it.trim_space().split(',').map(it.int()))
		begin_point := Point{str_range[0][0], str_range[0][1]}
		end_point := Point{str_range[1][0], str_range[1][1]}

		mut range := []Point{}
		range << begin_point
		for {
			last := range.last()
			next := move_one(last, end_point)
			if last == next {
				break
			} else {
				range << next
			}
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
	println('PART B: $total')
}

fn main() {
	lines := os.read_lines('./input.txt') ?
	part_a(lines)
	part_b(lines)
}
