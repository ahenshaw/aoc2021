#! /usr/bin/python3
from datetime import datetime
from time import sleep
import webbrowser
from subprocess import run

start = now = datetime.now()

while start.day == now.day:
    sleep(0.1)
    now = datetime.now()

# time to go!
webbrowser.open(f'https://adventofcode.com/{now.year}/day/{now.day}')
run(['cargo', 'aoc', 'input', '-d', f'{now.day}'])