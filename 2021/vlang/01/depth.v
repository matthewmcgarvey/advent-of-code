module main

import os
import arrays

fn part_a(entries []int) int {
	mut count := 0
	for i, entry in entries {
		if i == 0 {
			continue
		} else if entry > entries[i - 1] {
			count++
		}
	}
	return count
}

fn part_b(entries []int) ?int {
	variations := arrays.window(entries, size: 3).map(arrays.sum(it) ?)
	return part_a(variations)
}

fn main() {
	lines := os.read_lines('./input.txt') ?
	entries := lines.map(it.int())
	println(part_b(entries) ?)
}
