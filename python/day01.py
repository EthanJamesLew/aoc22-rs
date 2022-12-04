"""
Day 1: Calorie Counting
"""
from aoc import AoC
import pathlib

class Day01(AoC):
    def __init__(self):
        self.elfs = []

    def load_from_file(self, filepath: pathlib.Path):
        with open(filepath, 'r') as fp:
            in_str = fp.read()
        elf_strs = in_str.split('\n\n')
        elfs = []
        for elf_s in elf_strs:
            cals = []
            for cal_s in elf_s.split('\n'):
                cals.append(int(cal_s))
            elfs.append(cals)
        self.elfs = elfs
    
    def part1(self):
        return max(sum(elf) for elf in self.elfs) 

    def part2(self):
        total_cals = [sum(elf) for elf in self.elfs]
        total_cals.sort()
        return sum(total_cals[-3:])


if __name__ == "__main__":
    Day01.from_file('./inputs/day01/input.txt').run()
