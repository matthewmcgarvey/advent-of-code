module main

import os
import math

fn part_a(lines []string) {
	mut gamma_rate := 0
	mut epsilon_rate := 0
	line_length := lines[0].len
	for i in 0 .. line_length {
		mut zero_count := 0
		mut one_count := 0
		for line in lines {
			if line[i] == '0'[0] {
				zero_count += 1
			} else {
				one_count += 1
			}
		}
		if one_count > zero_count {
			gamma_rate += int(math.pow(2, (line_length - 1 - i)))
		} else {
			epsilon_rate += int(math.pow(2, (line_length - 1 - i)))
		}
	}
	
	result := gamma_rate * epsilon_rate
	println('PART A: $result')
}

fn binary_str_to_num(str string) int {
	mut num := 0
	for i, s in str {
		if s == byte(`1`) {
			num += int(math.pow(2, (str.len - 1 - i)))
		}
	}
	return num
}

fn part_b(lines []string) {
	line_length := lines[0].len
	mut common := ''
	for i in 0 .. line_length {
		mut zero_count := 0
		mut one_count := 0
		for line in lines {
			if !line.starts_with(common) {
				continue
			}
			if line[i] == byte(`0`) {
				zero_count += 1
			} else {
				one_count += 1
			}
		}

		if one_count >= zero_count {
			common += '1'
		} else {
			common += '0'
		}
		check := lines.filter(it.starts_with(common))
		if check.len == 1 {
			common = check[0]
			break
		}
	}
	oxy_num := binary_str_to_num(common)

	mut rare := ''
	for i in 0 .. line_length {
		mut zero_count := 0
		mut one_count := 0
		for line in lines {
			if !line.starts_with(rare) {
				continue
			}
			if line[i] == byte(`0`) {
				zero_count += 1
			} else {
				one_count += 1
			}
		}

		if one_count < zero_count {
			rare += '1'
		} else {
			rare += '0'
		}

		check := lines.filter(it.starts_with(rare))
		if check.len == 1 {
			rare = check[0]
			break
		}
	}
	scrub_num := binary_str_to_num(rare)
	
	println('PART B: ${oxy_num * scrub_num}')
}

fn main() {
	lines := os.read_lines('./input.txt') ?
	part_a(lines)
	part_b(lines)
}
