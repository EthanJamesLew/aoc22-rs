"""
Check that rust is faster

TODO: move bash stuff here
"""
import sys
import json

if __name__ == "__main__":
    fname= sys.argv[1]
    with open(fname, "r") as fp:
        perf = json.load(fp)

    if perf['python_time'] < perf['rust_time']:
        raise Exception(f"Python time is faster ({perf})")

    print(f"{__file__}: Check Succeeded!")