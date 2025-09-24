// Package main ...
package main

import (
	"strconv"
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxDiff(num int) int {
	// 替换字符串中的字符
	replace := func(s string, x, y byte) string {
		return strings.ReplaceAll(s, string(x), string(y))
	}
	minNum := strconv.Itoa(num)
	maxNum := strconv.Itoa(num)
	// 找到一个高位替换成 9
	for _, digit := range maxNum {
		if digit != '9' {
			maxNum = replace(maxNum, byte(digit), '9')
			break
		}
	}
	// 将最高位替换成 1
	// 或者找到一个与最高位不相等的高位替换成 0
	for i := 0; i < len(minNum); i++ {
		digit := minNum[i]
		if i == 0 {
			if digit != '1' {
				minNum = replace(minNum, digit, '1')
				break
			}
		} else {
			if digit != '0' && digit != minNum[0] {
				minNum = replace(minNum, digit, '0')
				break
			}
		}
	}

	max, _ := strconv.Atoi(maxNum)
	min, _ := strconv.Atoi(minNum)
	return max - min
}

func main() {
	tests := []struct {
		num int
		ans int
	}{
		{555, 888},
		{9, 8},
		{123456, 820000},
		{10000, 80000},
		{9288, 8700},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxDiff(test.num), index)
	}
}
