
"""
Day 4: Camp Cleanup 
"""
from aoc import AoC
import pathlib

class Day04(AoC):
    @staticmethod
    def subseteq(pts0, pts1):
        return pts0[0] >= pts1[0] and pts0[1] <= pts1[1]

    @staticmethod
    def overlaps(pts0, pts1):
        return (pts0[0] <= pts1[1] and pts0[1] >= pts1[0]) or \
            (pts1[0] <= pts0[1] and pts1[1] >= pts0[0])

    def __init__(self):
        self.end_pts = []

    def load_from_file(self, filepath: pathlib.Path):
        with open(filepath, 'r') as fp:
            in_str = fp.read()
        for line in in_str.splitlines():
            segs = []
            for seg in line.split(','):
                end_pts = [int(pt) for pt in seg.split('-')]
                segs.append(end_pts)
            self.end_pts.append(segs)

    def part1(self):
        return len([None for pts0, pts1 in self.end_pts if \
            self.subseteq(pts0, pts1) or self.subseteq(pts1, pts0)]) 

    def part2(self):
        return len([None for pts0, pts1 in self.end_pts if \
            self.overlaps(pts0, pts1) or self.overlaps(pts1, pts0)]) 


if __name__ == "__main__":
    Day04.from_file('./inputs/day04/input.txt').run()
