module main

import os
import arrays

// more natural solution growing the array of fish
fn part_a(fish []int) {
	mut result := fish.clone()
	for _ in 0 .. 80 {
		result = arrays.flatten(result.map(fn (f int) []int {
			if f == 0 {
				return [6, 8]
			} else {
				return [f - 1]
			}
		}))
	}
	println('PART A: ${result.len}')
}

// Solution in Part A would not work for 256 days without crashing
// so I had to come up with something else
// Only found out about this way by "cheating" but at least I understand it
fn part_b(fish []int) {
	mut result := [u64(0), u64(0), u64(0), u64(0), u64(0), u64(0), u64(0), u64(0), u64(0)] // count fish per day rotating
	for f in fish {
		result[f]++
	}

	for _ in 0 .. 256 {
		// rotating result for next breeding day
		first := result[0]
		result.delete(0)
		result << first

		// hackiest part
		// everything at the end of the array are the new fish
		// and every fish can only create 1 fish
		// so we add that amount to the 6th day to represent the fish that did the breeding
		new_fish := result.last()
		result[6] += new_fish
	}

	total := arrays.sum(result) or { 0 }
	println('PART B: $total')
}

fn main() {
	lines := os.read_lines('./input.txt') ?
	temp := lines[0].split(',').map(it.int())
	part_a(temp)
	part_b(temp)
}
