// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

type RangeFreqQuery struct {
	// 数值为键，出现下标数组为值的哈希表
	occurrence map[int][]int
}

func Constructor(arr []int) RangeFreqQuery {
	occurrence := make(map[int][]int)
	// 顺序遍历数组初始化哈希表
	for i, v := range arr {
		occurrence[v] = append(occurrence[v], i)
	}
	return RangeFreqQuery{occurrence: occurrence}
}

func (this *RangeFreqQuery) Query(left int, right int, value int) int {
	// 查找对应的出现下标数组，不存在则为空
	pos, exists := this.occurrence[value]
	if !exists {
		return 0
	}
	// 两次二分查找计算子数组内出现次数
	l := lowerBound(pos, left)
	r := upperBound(pos, right)
	return r - l
}

func lowerBound(pos []int, target int) int {
	low, high := 0, len(pos)-1
	for low <= high {
		mid := low + (high-low)/2
		if pos[mid] < target {
			low = mid + 1
		} else {
			high = mid - 1
		}
	}
	return low
}

func upperBound(pos []int, target int) int {
	low, high := 0, len(pos)-1
	for low <= high {
		mid := low + (high-low)/2
		if pos[mid] <= target {
			low = mid + 1
		} else {
			high = mid - 1
		}
	}
	return low
}

func main() {
	input := []int{12, 33, 4, 56, 22, 2, 34, 33, 22, 12, 34, 56}
	rangeFreqQuery := Constructor(input)
	assert.Equal(&testing.T{}, 1, rangeFreqQuery.Query(1, 2, 4))
	assert.Equal(&testing.T{}, 2, rangeFreqQuery.Query(0, 11, 33))
}
