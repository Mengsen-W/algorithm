/*
 * @Date: 2024-05-03
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-03
 * @FilePath: /algorithm/golang/1491_average/average.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func average(salary []int) float64 {
	maxValue := salary[0]
	minValue := salary[0]
	sum := 0
	for _, s := range salary {
		if s > maxValue {
			maxValue = s
		}
		if s < minValue {
			minValue = s
		}
		sum += s
	}
	sum -= maxValue + minValue
	return float64(sum) / float64(len(salary)-2)
}

func main() {
	tests := []struct {
		salary []int
		ans    float64
	}{
		{[]int{4000, 3000, 1000, 2000}, 2500.00000},
		{[]int{1000, 2000, 3000}, 2000.00000},
		{[]int{6000, 5000, 4000, 3000, 2000, 1000}, 3500.00000},
		{[]int{8000, 9000, 2000, 3000, 6000, 1000}, 4750.00000},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, average(test.salary), index)
	}
}
