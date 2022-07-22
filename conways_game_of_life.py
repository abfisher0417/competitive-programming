"""
Conway's Game of Life
"""

from copy import deepcopy
from time import sleep
from typing import List, Tuple
import os
import random

World = List[List[int]]
Cell = Tuple[int, int]

LIVE = "@"
EMPTY = "."
PAD = " "


def is_cell_alive(world: World, i: int, j: int) -> bool:
    h = len(world)
    w = len(world[0])
    i += h
    i %= h
    j += w
    j %= w
    return world[i][j] > 0


def next_cell_state(world: World, i: int, j: int) -> int:
    alive_cnt = 0
    for y in range(-1, 2):
        for x in range(-1, 2):
            if (y != 0 or x != 0) and is_cell_alive(world, i + y, j + x):
                alive_cnt += 1
    return alive_cnt == 3 or alive_cnt == 2 and is_cell_alive(world, i, j)


def next_generation(world: World) -> World:
    new_world = deepcopy(world)
    for i in range(len(new_world)):
        for j in range(len(new_world[0])):
            new_world[i][j] = next_cell_state(world, i, j)
    return new_world


def init_world(width: int, height: int, shape: str = None) -> World:
    world: World = [[0] * width for _ in range(height)]
    for i in range(height):
        for j in range(width):
            world[i][j] = random.randint(0, 1)
    return world


def life(world: World, n: int) -> World:
    for _ in range(n):
        yield world
        world = next_generation(world)


def picture(world) -> str:
    def row(r):
        return PAD.join(LIVE if val > 0 else EMPTY for val in r)

    return "\n".join(row(r) for r in world)


def population(world: World) -> int:
    cnt = 0
    for i in range(len(world)):
        for j in range(len(world[0])):
            if world[i][j]:
                cnt += 1
    return cnt


def animate_life(world: World, n: int = 100, pause: float = 1 / 5):
    w = len(world[0])
    for g, world in enumerate(life(world, n)):
        p = population(world)
        os.system("cls" if os.name == "nt" else "clear")
        print(f"Generation: {g + 1}      Population: {p}")
        print("=" * w * 2)
        print(picture(world))
        sleep(pause)


def main():
    world = init_world(30, 30)
    animate_life(world)


if __name__ == "__main__":
    main()
