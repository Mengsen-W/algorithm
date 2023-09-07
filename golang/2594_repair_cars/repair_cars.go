/*
 * @Date: 2023-09-07
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-07
 * @FilePath: /algorithm/golang/2594_repair_cars/repair_cars.go
 */

// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func repairCars(ranks []int, cars int) int64 {
	l, r := 1, ranks[0]*cars*cars
	check := func(m int) bool {
		cnt := 0
		for _, x := range ranks {
			cnt += int(math.Sqrt(float64(m / x)))
		}
		return cnt >= cars
	}

	for l < r {
		m := (l + r) >> 1
		if check(m) {
			r = m
		} else {
			l = m + 1
		}
	}
	return int64(l)
}

func main() {
	tests := []struct {
		ranks []int
		cars  int
		ans   int64
	}{
		{[]int{4, 2, 3, 1}, 10, 16},
		{[]int{5, 1, 8}, 6, 16},
	}

	for index, item := range tests {
		assert.Equal(&testing.T{}, item.ans, repairCars(item.ranks, item.cars), index)
	}
}
