/*
 * @Date: 2023-10-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-09
 * @FilePath: /algorithm/golang/2578_split_num/split_num.go
 */

// Package main ...
package main

import (
	"sort"
	"strconv"
	"testing"

	"github.com/stretchr/testify/assert"
)

func splitNum(num int) int {
	stnum := []byte(strconv.Itoa(num))
	sort.Slice(stnum, func(i, j int) bool {
		return stnum[i] < stnum[j]
	})
	num1, num2 := 0, 0
	for i := 0; i < len(stnum); i++ {
		if i%2 == 0 {
			num1 = num1*10 + int(stnum[i]-'0')
		} else {
			num2 = num2*10 + int(stnum[i]-'0')
		}
	}
	return num1 + num2
}

func main() {
	tests := []struct {
		num int
		ans int
	}{
		{4325, 59},
		{687, 75},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, splitNum(test.num), "第"+strconv.Itoa(index)+"个测试用例失败")
	}
}
