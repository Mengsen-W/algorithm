/*
 * @Date: 2022-02-14 08:26:16
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-14 09:02:17
 * @FilePath: /algorithm/540_single_non_duplicate/single_non_duplicate.go
 */

package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func singleNonDuplicate(nums []int) int {
	low, high := 0, len(nums)-1
	for low < high {
		mid := low + (high-low)/2
		mid -= mid & 1
		if nums[mid] == nums[mid+1] {
			low = mid + 2
		} else {
			high = mid
		}
	}
	return nums[low]
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{1, 1, 2, 3, 3, 4, 4, 8, 8}, 2},
		{[]int{3, 3, 7, 7, 10, 11, 11}, 10},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, singleNonDuplicate(test.nums), index)
	}
}
