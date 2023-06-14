/*
 * @Date: 2023-06-14
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-14
 * @FilePath: /algorithm/golang/1375_num_times_all_blue/num_times_all_blue.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/go-playground/assert/v2"
)

func numTimesAllBlue(flips []int) int {
	n, ans, right := len(flips), 0, 0
	for i := 0; i < n; i++ {
		if flips[i] > right {
			right = flips[i]
		}
		if right == i+1 {
			ans++
		}
	}
	return ans
}

func main() {
	t := &testing.B{}
	{
		flips := []int{3, 2, 4, 1, 5}
		ans := 2
		assert.Equal(t, numTimesAllBlue(flips), ans)
	}

	{
		flips := []int{4, 1, 2, 3}
		ans := 1
		assert.Equal(t, numTimesAllBlue(flips), ans)
	}
}
