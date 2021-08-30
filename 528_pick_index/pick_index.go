/*
 * @Date: 2021-08-30 13:03:45
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-30 13:09:08
 */

package main

import (
	"math/rand"
	"sort"
)

type Solution struct {
	pre []int
}

func Constructor(w []int) Solution {
	for i := 1; i < len(w); i++ {
		w[i] += w[i-1]
	}
	return Solution{w}
}

func (s *Solution) PickIndex() int {
	x := rand.Intn(s.pre[len(s.pre)-1]) + 1
	return sort.SearchInts(s.pre, x)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		s := Constructor([]int{1})
		assert(s.PickIndex() == 0)
	}
	{
		s := Constructor([]int{1, 3})
		assert(s.PickIndex() == 1)
	}
}
