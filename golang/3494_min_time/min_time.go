// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minTime(skill []int, mana []int) int64 {
	n, m := len(skill), len(mana)
	times := make([]int64, n)

	for j := 0; j < m; j++ {
		curTime := int64(0)
		for i := 0; i < n; i++ {
			if curTime < times[i] {
				curTime = times[i]
			}
			curTime += int64(skill[i]) * int64(mana[j])
		}
		times[n-1] = curTime
		for i := n - 2; i >= 0; i-- {
			times[i] = times[i+1] - int64(skill[i+1])*int64(mana[j])
		}
	}
	return times[n-1]
}

func main() {
	tests := []struct {
		skill []int
		mana  []int
		ans   int64
	}{
		{[]int{1, 5, 2, 4}, []int{5, 1, 4, 2}, 110},
		{[]int{1, 1, 1}, []int{1, 1, 1}, 5},
		{[]int{1, 2, 3, 4}, []int{1, 2}, 21},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minTime(test.skill, test.mana), "test case %d failed", index)
	}
}
