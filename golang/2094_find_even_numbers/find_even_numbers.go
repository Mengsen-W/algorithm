// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func findEvenNumbers(digits []int) []int {
	res := []int{}
	freq := make(map[int]int)
	// 统计数字数组中各数字的出现次数
	for _, d := range digits {
		freq[d]++
	}
	// 枚举所有三位偶数
	for i := 100; i < 1000; i += 2 {
		freq1 := make(map[int]int)
		num := i
		// 统计当前偶数的每一位数字的出现次数
		for num > 0 {
			d := num % 10
			freq1[d]++
			num /= 10
		}
		// 检查是否满足条件
		isValid := true
		for d, count := range freq1 {
			if freq[d] < count {
				isValid = false
				break
			}
		}
		if isValid {
			res = append(res, i)
		}
	}
	return res
}

func main() {
	tests := []struct {
		digits []int
		res    []int
	}{
		{[]int{2, 1, 3, 0}, []int{102, 120, 130, 132, 210, 230, 302, 310, 312, 320}},
		{[]int{2, 2, 8, 8, 2}, []int{222, 228, 282, 288, 822, 828, 882}},
		{[]int{3, 7, 5}, []int{}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.res, findEvenNumbers(test.digits), index)
	}
}
