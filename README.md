# Two Sum - Modified

[![Build Status](https://travis-ci.org/iamdejan/rust-two-sum-modified.svg?branch=master)](https://travis-ci.org/iamdejan/rust-two-sum-modified)

This repository contains my solution from modified "Two Sum" problem.

## Original Problem Statement

Source: [Leetcode](https://leetcode.com/problems/two-sum/)

```
Given an array of integers,
return indices of the two numbers
such that they add up to a specific target.

You may assume that each input would have exactly one solution,
and you may not use the same element twice.
```

## Modified Problem Statement

```
Given an array of distinct integers,
return indices of all pairs
such that they add up to a specific target.

You may not use same element in each pair.
However, one element can be used multiple times
in different pairs.

Note: (0,4) and (4,0) are considered same,
so the result contains only 1 of them.

Examples:

Test Case #1
numbers = [1,2,3,4,5,6,7,8]
target = 6
valid pairs = [(0,4),(1,3)] or [(3,1),(4,0)]
invalid pairs 1 = [(2,2)]
invalid pairs 2 = [(0,4), (4,0)]

Test Case #2
numbers = [2,7,11,5,3,6,9]
target = 9
valid pairs = [(0,1),(4,5)] or [(1,0),(5,4)]
invalid pairs 1 = [(0,1),(1,0)]

Test Case #3
numbers = [1,2]
target = 10
valid pairs = none, print "no pairs found" instead
```

## My Solution

My solution is located at `solution/src/main.rs` file.

## How to Run The Tests

### Prerequisites
1) Docker
2) Docker Compose

### Run Tests

You can run from terminal:
```
$ docker-compose build
$ docker-compose up
```

## Authors
- Giovanni Dejan - [iamdejan](https://github.com/iamdejan)
