/*
 * @Date: 2021-09-13 08:20:58
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-08
 */

package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func numberOfBoomerangs(points [][]int) (ans int) {
	for _, p := range points {
		cnt := map[int]int{}
		for _, q := range points {
			dis := (p[0]-q[0])*(p[0]-q[0]) + (p[1]-q[1])*(p[1]-q[1])
			cnt[dis]++
		}
		for _, m := range cnt {
			ans += m * (m - 1)
		}
	}
	return
}

func main() {
	tests := []struct {
		points [][]int
		ans    int
	}{
		{[][]int{{0, 0}, {1, 0}, {2, 0}}, 2},
		{[][]int{{1, 1}, {2, 2}, {3, 3}}, 2},
		{[][]int{{1, 1}}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, numberOfBoomerangs(test.points), index)
	}
}
