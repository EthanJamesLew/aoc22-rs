
"""
Advent of Code Runner
"""
import pathlib
import abc


class AoC(abc.ABC):
    """
    Advent of Code Runner
    """
    @classmethod
    def from_file(cls: 'AoC', filename: pathlib.Path) -> None:
        self = cls()
        self.load_from_file(filename)
        return self

    @abc.abstractmethod
    def load_from_file(self, filepath: pathlib.Path):
        """all puzzle load from a file"""
        raise NotImplementedError

    @abc.abstractmethod
    def part1(self):
        """all puzzles have a part1"""
        raise NotImplementedError

    @abc.abstractmethod
    def part2(self):
        """all puzzles have a part2"""
        raise NotImplementedError

    @property
    def day(self) -> int:
        return self.__class__.__name__ 

    def run(self):
        print(f"Advent of Code ({self.day})")
        print(f"--Part 1--")
        try:
            ans = self.part1()
            print(f"Solution: {ans}")
        except Exception as exc:
            print(f"Encountered Error: <{exc}>")
        print(f"--Part 2--")
        try:
            ans = self.part2()
            print(f"Solution: {ans}")
        except Exception as exc:
            print(f"Encountered Error: <{exc}>")
        print(f"Done!")

