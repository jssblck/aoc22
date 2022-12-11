# aoc22
Advent Of Code 2022

# the goal

aoc isn't fun without some goal. basically, my goal is to be idiomatic and performant, but not think too hard about it.
kinda simulate "i'm working on this rust project for work, and my employer doesn't care about performance too much".

explicitly though, i want to avoid:
- too much "this will only work so long as the problem can fit in memory".
- panics at all, only proper error messages here.

finally, i'm also pretending i need to make this maintainable for other people on my hypothetical team.
given that, i'll be focusing on _simple_ and _clear_ operations over clever tricks.
this will likely result in more verbose code (simple does not mean easy) but the idea is that it'd be easier to follow.
