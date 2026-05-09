// Package main ...
package main

import (
	"fmt"
	"reflect"
)

func rotateGrid(grid [][]int, k int) [][]int {
	m := len(grid)
	n := len(grid[0])
	nlayer := min(m/2, n/2) // 层数
	// 从左上角起逆时针枚举每一层
	for layer := 0; layer < nlayer; layer++ {
		r := make([]int, 0)
		c := make([]int, 0)
		val := make([]int, 0)                // 每个元素的行下标，列下标与数值
		for i := layer; i < m-layer-1; i++ { // 左
			r = append(r, i)
			c = append(c, layer)
			val = append(val, grid[i][layer])
		}
		for j := layer; j < n-layer-1; j++ { // 下
			r = append(r, m-layer-1)
			c = append(c, j)
			val = append(val, grid[m-layer-1][j])
		}
		for i := m - layer - 1; i > layer; i-- { // 右
			r = append(r, i)
			c = append(c, n-layer-1)
			val = append(val, grid[i][n-layer-1])
		}
		for j := n - layer - 1; j > layer; j-- { // 上
			r = append(r, layer)
			c = append(c, j)
			val = append(val, grid[layer][j])
		}
		total := len(val) // 每一层的元素总数
		kk := k % total   // 等效轮转次数
		// 找到每个下标对应的轮转后的取值
		for i := 0; i < total; i++ {
			idx := (i + total - kk) % total // 轮转后取值对应的下标
			grid[r[i]][c[i]] = val[idx]
		}
	}
	return grid
}

func main() {
	tests := []struct {
		grid [][]int
		k    int
		ans  [][]int
	}{
		{
			[][]int{{40, 10}, {30, 20}},
			1,
			[][]int{{10, 20}, {40, 30}},
		},
		{
			[][]int{{1, 2, 3, 4}, {5, 6, 7, 8}, {9, 10, 11, 12}, {13, 14, 15, 16}},
			2,
			[][]int{{3, 4, 8, 12}, {2, 11, 10, 16}, {1, 7, 6, 15}, {5, 9, 13, 14}},
		},
	}

	for _, test := range tests {
		ans := rotateGrid(test.grid, test.k)
		fmt.Println(reflect.DeepEqual(ans, test.ans))
	}
}
