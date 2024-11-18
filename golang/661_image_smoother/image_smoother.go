/*
 * @Date: 2022-03-23 23:29:28
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-23 23:41:28
 * @FilePath: /algorithm/661_image_smoother/image_smoother.go
 */

package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func imageSmoother(img [][]int) [][]int {
	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}
	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}
	m, n := len(img), len(img[0])
	ans := make([][]int, m)
	for i := range ans {
		ans[i] = make([]int, n)
		for j := range ans[i] {
			sum, num := 0, 0
			for _, row := range img[max(i-1, 0):min(i+2, m)] {
				for _, v := range row[max(j-1, 0):min(j+2, n)] {
					sum += v
					num++
				}
			}
			ans[i][j] = sum / num
		}
	}
	return ans
}

func main() {
	tests := []struct {
		img [][]int
		ans [][]int
	}{
		{[][]int{{1, 1, 1}, {1, 0, 1}, {1, 1, 1}}, [][]int{{0, 0, 0}, {0, 0, 0}, {0, 0, 0}}},
		{[][]int{{100, 200, 100}, {200, 50, 200}, {100, 200, 100}}, [][]int{{137, 141, 137}, {141, 138, 141}, {137, 141, 137}}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, imageSmoother(test.img), index)
	}
}
