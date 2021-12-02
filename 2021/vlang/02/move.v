module main

import os

fn part_a(lines []string) {
	mut position := 0
	mut depth := 0
	for line in lines {
		split := line.split(' ')
		command := split[0]
		units := split[1].int()
		if command == 'forward' {
			position += units
		} else if command == 'up' {
			depth -= units
		} else if command == 'down' {
			depth += units
		}
	}

	multiplied := position * depth
	println('PART A: $multiplied')
}

fn part_b(lines []string) {
	mut position := 0
	mut depth := 0
	mut aim := 0
	for line in lines {
		split := line.split(' ')
		command := split[0]
		units := split[1].int()
		if command == 'forward' {
			position += units
			depth += aim * units
		} else if command == 'up' {
			aim -= units
		} else if command == 'down' {
			aim += units
		}
	}

	multiplied := position * depth
	println('PART B: $multiplied')
}

fn main() {
	lines := os.read_lines('./input.txt') ?
	part_a(lines)
	part_b(lines)
}
