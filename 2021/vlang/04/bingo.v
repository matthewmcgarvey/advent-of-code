module main

import os
import arrays

struct BingoCell {
	value int
mut:
	filled bool
}

struct BingoBoard {
mut:
	rows [][]BingoCell = []
}

fn (mut board BingoBoard) fill_num(num int) {
	for mut row in board.rows {
		for mut cell in row {
			if cell.value == num {
				cell.filled = true
			}
		}
	}
}

fn (board BingoBoard) columns() [][]BingoCell {
	mut columns := [][]BingoCell{}
	for i, row in board.rows {
		for c, cell in row {
			if columns.len <= c {
				columns << []BingoCell{}
			}
			if columns[c].len <= i {
				columns[c] << []BingoCell{}
			}
			columns[c] << cell
		}
	}
	return columns
}

fn (board BingoBoard) has_bingo() bool {
	return board.rows.any(it.all(it.filled)) || board.columns().any(it.all(it.filled))
}

fn (board BingoBoard) unmarked_nums() []int {
	mut unmarked_nums := []int{}
	for row in board.rows {
		for cell in row {
			if !cell.filled {
				unmarked_nums << cell.value
			}
		}
	}
	return unmarked_nums
}

fn create_board(lines []string) BingoBoard {
	mut board := BingoBoard{}
	for line in lines {
		row := line.trim_space().split(' ').filter(it != '').map(BingoCell{ value: it.int() })
		board.rows << row
	}
	return board
}

fn parse_numbers(lines []string) []int {
	return lines[0].trim_space().split(',').map(it.int())
}

fn parse_boards(lines []string) []BingoBoard {
	mut boards := []BingoBoard{}
	mut board_lines := []string{}
	for line in lines[2..] {
		if line.trim_space() == '' {
			boards << create_board(board_lines)
			board_lines.clear()
		} else {
			board_lines << line
		}
	}
	boards << create_board(board_lines)
	return boards
}

fn part_a(lines []string) {
	numbers := parse_numbers(lines)
	mut boards := parse_boards(lines)

	for num in numbers {
		for mut board in boards {
			board.fill_num(num)
			if board.has_bingo() {
				unmarked_sum := arrays.sum(board.unmarked_nums()) or { 0 }
				result := unmarked_sum * num
				println('PART A: $result')
				return
			}
		}
	}
}

fn part_b(lines []string) {
	numbers := parse_numbers(lines)
	mut boards := parse_boards(lines)
	for num in numbers {
		mut unfinished := boards.filter(!it.has_bingo())
		for mut board in unfinished {
			board.fill_num(num)
			if unfinished.len == 1 && board.has_bingo() {
				unmarked_sum := arrays.sum(board.unmarked_nums()) or { 0 }
				result := unmarked_sum * num
				println('PART B: $result')
				return
			}
		}
	}
}

fn main() {
	lines := os.read_lines('./input.txt') ?
	part_a(lines)
	part_b(lines)
}
