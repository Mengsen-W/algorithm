/*
 * @Date: 2021-12-27 01:43:28
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-27 02:14:10
 */

package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func numFriendRequests(ages []int) (ans int) {
	const mx = 121
	var cnt, pre [mx]int
	for _, age := range ages {
		cnt[age]++
	}
	for i := 1; i < mx; i++ {
		pre[i] = pre[i-1] + cnt[i]
	}
	for i := 15; i < mx; i++ {
		if cnt[i] > 0 {
			bound := i/2 + 8
			ans += cnt[i] * (pre[i] - pre[bound-1] - 1)
		}
	}
	return
}

func main() {
	tests := []struct {
		ages []int
		ans  int
	}{
		{[]int{16, 16}, 2},
		{[]int{16, 17, 18}, 2},
		{[]int{20, 30, 100, 110, 120}, 3},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, numFriendRequests(test.ages), index)
	}
}
