
"""
Day 6:
"""
from aoc import AoC
import pathlib

class Day06(AoC):
    @staticmethod
    def scan_overlap(window_size, line):
        for i in range(window_size, len(line)):
            chunk = line[i-window_size:i]
            if len(chunk) == len(set(chunk)):
                return i
        return None

    def __init__(self):
        self.line = ''

    def load_from_file(self, filepath: pathlib.Path):
       with open(filepath, 'r') as fp:
        self.line = fp.read()

    def part1(self):
        return self.scan_overlap(4, self.line) 

    def part2(self):
        return self.scan_overlap(14, self.line) 

if __name__ == "__main__":
    Day06.from_file('./inputs/day06/input.txt').run()
