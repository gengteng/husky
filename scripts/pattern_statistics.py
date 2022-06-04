#!/usr/bin/python
import re
import sys
import os
from termcolor import colored

# Usage: path/to/script <pattern> <search-dir> <depth>

pattern = sys.argv[1]
search_path = sys.argv[2]
depth = int(sys.argv[3])


def subpaths(path: str) -> "list[str]":
    return [os.path.join(path, filename) for filename in os.listdir(path)]


def search(pattern: str, search_path: str, depth: int):
    if depth <= 0 or os.path.isfile(search_path):
        print(
            "    {:30s}".format(colored(search_path, "green")),
            colored("{}".format(count_patterns(pattern, search_path)), "yellow"),
        )
    else:
        for subpath in subpaths(search_path):
            search(pattern, subpath, depth - 1)


def count_patterns(pattern: str, search_path: str) -> int:
    total = 0
    if os.path.isdir(search_path):
        for subpath in subpaths(search_path):
            total += count_patterns(pattern, subpath)
    elif os.path.isfile(search_path):
        file = open(search_path, "r")
        for line in file:
            if re.search(pattern, line):
                total += 1
    else:
        raise Exception("{} is not a valid search path".format(search_path))
    return total


print(colored("\nCount todos:", "cyan"))
search(pattern, search_path, depth)
