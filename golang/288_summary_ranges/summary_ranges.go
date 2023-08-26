/*
 * @Date: 2023-08-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-26
 * @FilePath: /algorithm/golang/288_summary_ranges/summary_ranges.go
 */

// Package main ....
package main

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func summaryRanges(nums []int) []string {
	ans := []string{}
	for left := 0; left < len(nums); left++ {
		right := left
		gap := 0
		for right < len(nums) && nums[right] == nums[left]+gap {
			right++
			gap++
		}
		right--
		if left == right {
			ans = append(ans, fmt.Sprintf("%d", nums[left]))
		} else {
			ans = append(ans, fmt.Sprintf("%d->%d", nums[left], nums[right]))
		}
		left = right
	}
	return ans
}

func main() {
	tests := []struct {
		nums []int
		ans  []string
	}{
		{[]int{1, 2, 3, 4, 5}, []string{"1->5"}},
		{[]int{0, 1, 2, 4, 5, 7}, []string{"0->2", "4->5", "7"}},
		{[]int{0, 2, 3, 4, 6, 8, 9}, []string{"0", "2->4", "6", "8->9"}},
	}

	for index, item := range tests {
		assert.Equal(&testing.T{}, item.ans, summaryRanges(item.nums), index)
	}
}
