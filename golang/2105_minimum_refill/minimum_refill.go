/*
 * @Date: 2024-05-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-09
 * @FilePath: /algorithm/golang/2105_minimum_refill/minimum_refill.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumRefill(plants []int, capacityA int, capacityB int) int {
	res := 0 // 灌满水罐次数
	n := len(plants)
	posa, posb := 0, n-1               // 两人位置
	vala, valb := capacityA, capacityB // 两人剩余水量
	// 模拟相遇前的浇水过程
	for posa < posb {
		if vala < plants[posa] {
			res++
			vala = capacityA - plants[posa]
		} else {
			vala -= plants[posa]
		}
		posa++
		if valb < plants[posb] {
			res++
			valb = capacityB - plants[posb]
		} else {
			valb -= plants[posb]
		}
		posb--
	}
	// 模拟相遇后可能的浇水过程
	if posa == posb {
		if vala >= valb && vala < plants[posa] {
			res++
		}
		if vala < valb && valb < plants[posb] {
			res++
		}
	}
	return res
}

func main() {
	tests := []struct {
		plants    []int
		capacityA int
		capacityB int
		ans       int
	}{
		{[]int{2, 2, 3, 3}, 5, 5, 1},
		{[]int{2, 2, 3, 3}, 3, 4, 2},
		{[]int{5}, 10, 8, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minimumRefill(test.plants, test.capacityA, test.capacityB), index)
	}
}
