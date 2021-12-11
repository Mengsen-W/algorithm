/*
 * @Date: 2021-12-11 07:08:52
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-11 07:24:49
 */

package main

import "sort"

type TopVotedCandidate struct {
	tops, times []int
}

func Constructor(persons, times []int) TopVotedCandidate {
	tops := []int{}
	top := -1
	voteCounts := map[int]int{-1: -1}
	for _, p := range persons {
		voteCounts[p]++
		if voteCounts[p] >= voteCounts[top] {
			top = p
		}
		tops = append(tops, top)
	}
	return TopVotedCandidate{tops, times}
}

func (c *TopVotedCandidate) Q(t int) int {
	return c.tops[sort.SearchInts(c.times, t+1)-1]
}

func main() {
	assert := func(a, b int) {
		if a != b {
			panic("Not Passed")
		}
	}
	obj := Constructor([]int{0, 1, 1, 0, 0, 1, 0}, []int{0, 5, 10, 15, 20, 25, 30})

	assert(obj.Q(3), 0)
	assert(obj.Q(12), 1)
	assert(obj.Q(25), 1)
	assert(obj.Q(15), 0)
	assert(obj.Q(24), 0)
	assert(obj.Q(8), 1)
}
