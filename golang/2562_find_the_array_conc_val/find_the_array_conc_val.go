/*
 * @Date: 2023-10-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-12
 * @FilePath: /algorithm/golang/2562_find_the_array_conc_val/find_the_array_conc_val.go
 */

// Package main ...
package main

import (
	"strconv"
	"testing"

	"github.com/stretchr/testify/assert"
)

func findTheArrayConcVal(nums []int) int64 {
	ans := 0
	i, j := 0, len(nums)-1
	for i <= j {
		if i != j {
			val, _ := strconv.Atoi(strconv.Itoa(nums[i]) + strconv.Itoa(nums[j]))
			ans += val
		} else {
			ans += nums[i]
		}
		i++
		j--
	}
	return int64(ans)
}

func main() {
	tests := []struct {
		nums []int
		ans  int64
	}{
		{[]int{7, 52, 2, 4}, 596},
		{[]int{5, 14, 13, 8, 12}, 673},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, findTheArrayConcVal(test.nums), "case:%d", index)
	}
}
