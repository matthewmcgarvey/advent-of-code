module main

import os
import arrays
import math
import math.mathutil

struct Thing {
	pos int
	fuel int
}

fn part_a(positions []int) {
	mut uniq_pos := []int{}
	min := arrays.min(positions) or { 0 }
	max := arrays.max(positions) or { 100 }
	for pos in min .. max {
		uniq_pos << pos
	}

	total_gas_to_align := fn [positions] (target int) int {
		return arrays.sum(positions.map(mathutil.abs(target - it))) or { 0 }
	}

	mut things := uniq_pos.map(Thing{it, total_gas_to_align(it)})
	things.sort(a.fuel < b.fuel)
	result := things.first()
	
	println("PART A: ${result.fuel}")
}

fn part_b(positions []int) {
	mut uniq_pos := []int{}
	min := arrays.min(positions) or { 0 }
	max := arrays.max(positions) or { 100 }
	for pos in min .. max {
		uniq_pos << pos
	}

	total_gas_to_align := fn [positions] (target int) int {
		gas_calc := fn [target] (pos int) int {
			distance := mathutil.abs(target - pos)
			if distance <= 1 {
				return distance
			}
			return (distance * distance + distance) / 2
		}
		gas := positions.map(gas_calc)
		return arrays.sum(gas) or { 0 }
	}

	mut things := uniq_pos.map(Thing{it, total_gas_to_align(it)})
	things.sort(a.fuel < b.fuel)
	result := things.first()
	
	println("PART B: ${result.fuel}")
}

fn main() {
	lines := os.read_lines('./input.txt') ?
	positions := lines[0].split(',').map(it.int())
	part_a(positions)
	part_b(positions)
}
