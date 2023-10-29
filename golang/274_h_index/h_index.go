/*
 * @Date: 2021-07-12 08:24:51
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-29
 */

package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func hIndex(citations []int) (h int) {
	n := len(citations)
	counter := make([]int, n+1)
	for _, citation := range citations {
		if citation >= n {
			counter[n]++
		} else {
			counter[citation]++
		}
	}
	for i, tot := n, 0; i >= 0; i-- {
		tot += counter[i]
		if tot >= i {
			return i
		}
	}
	return 0
}

func main() {
	tests := []struct {
		citations []int
		ans       int
	}{
		{[]int{3, 0, 6, 1, 5}, 3},
		{[]int{1, 3, 1}, 1},
		{[]int{0, 1, 3, 5, 6}, 3},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, hIndex(test.citations), index)
	}
}
