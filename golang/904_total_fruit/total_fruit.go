// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func totalFruit(fruits []int) (ans int) {
	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}
	cnt := map[int]int{}
	left := 0
	for right, x := range fruits {
		cnt[x]++
		for len(cnt) > 2 {
			y := fruits[left]
			cnt[y]--
			if cnt[y] == 0 {
				delete(cnt, y)
			}
			left++
		}
		ans = max(ans, right-left+1)
	}
	return
}

func main() {
	tests := []struct {
		fruits []int
		ans    int
	}{
		{[]int{1, 2, 1}, 3},
		{[]int{0, 1, 2, 2}, 3},
		{[]int{1, 2, 3, 2, 2}, 4},
		{[]int{3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4}, 5},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, totalFruit(test.fruits), index)
	}
}
