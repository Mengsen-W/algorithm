/*
 * @Date: 2023-09-30
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-30
 * @FilePath: /algorithm/golang/2136_earliest_full_bloom/earliest_full_bloom.go
 */

// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func earliestFullBloom(plantTime, growTime []int) (ans int) {
	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}
	type pair struct{ p, g int }
	a := make([]pair, len(plantTime))
	for i, p := range plantTime {
		a[i] = pair{p, growTime[i]}
	}
	sort.Slice(a, func(i, j int) bool { return a[i].g > a[j].g })
	days := 0
	for _, p := range a {
		days += p.p              // 累加播种天数
		ans = max(ans, days+p.g) // 再加上生长天数，就是这个种子的开花时间
	}
	return
}

func main() {
	tests := []struct {
		plantTime []int
		growTime  []int
		ans       int
	}{
		{[]int{1, 4, 3}, []int{2, 3, 1}, 9},
		{[]int{1, 2, 3, 2}, []int{2, 1, 2, 1}, 9},
		{[]int{1}, []int{1}, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, earliestFullBloom(test.plantTime, test.growTime), index)
	}
}
