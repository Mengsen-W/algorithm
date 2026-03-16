// Package main ...
package main

import (
	"fmt"
	"reflect"
)

type Answer struct {
	ans [3]int
}

func (this *Answer) put(x int) {
	if x > this.ans[0] {
		this.ans[2] = this.ans[1]
		this.ans[1] = this.ans[0]
		this.ans[0] = x
	} else if x != this.ans[0] && x > this.ans[1] {
		this.ans[2] = this.ans[1]
		this.ans[1] = x
	} else if x != this.ans[0] && x != this.ans[1] && x > this.ans[2] {
		this.ans[2] = x
	}
}

func (this *Answer) get() []int {
	var ret []int
	for _, num := range this.ans {
		if num != 0 {
			ret = append(ret, num)
		}
	}
	return ret
}

func getBiggestThree(grid [][]int) []int {
	m, n := len(grid), len(grid[0])
	sum1 := make([][]int, m+1)
	sum2 := make([][]int, m+1)
	for i := 0; i <= m; i++ {
		sum1[i] = make([]int, n+2)
		sum2[i] = make([]int, n+2)
	}

	for i := 1; i <= m; i++ {
		for j := 1; j <= n; j++ {
			sum1[i][j] = sum1[i-1][j-1] + grid[i-1][j-1]
			sum2[i][j] = sum2[i-1][j+1] + grid[i-1][j-1]
		}
	}

	ans := Answer{}
	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			// 单独的一个格子也是菱形
			ans.put(grid[i][j])
			for k := i + 2; k < m; k += 2 {
				ux, uy := i, j
				dx, dy := k, j
				lx, ly := (i+k)/2, j-(k-i)/2
				rx, ry := (i+k)/2, j+(k-i)/2
				if ly < 0 || ry >= n {
					break
				}
				sum := (sum2[lx+1][ly+1] - sum2[ux][uy+2]) +
					(sum1[rx+1][ry+1] - sum1[ux][uy]) +
					(sum1[dx+1][dy+1] - sum1[lx][ly]) +
					(sum2[dx+1][dy+1] - sum2[rx][ry+2]) -
					(grid[ux][uy] + grid[dx][dy] + grid[lx][ly] + grid[rx][ry])

				ans.put(sum)
			}
		}
	}

	return ans.get()
}

func main() {
	tests := []struct {
		grid [][]int
		ans  []int
	}{
		{[][]int{{3, 4, 5, 1, 3}, {3, 3, 4, 2, 3}, {20, 30, 200, 40, 10}, {1, 5, 5, 4, 1}, {4, 3, 2, 2, 5}}, []int{228, 216, 211}},
		{[][]int{{1, 2, 3}, {4, 5, 6}, {7, 8, 9}}, []int{20, 9, 8}},
		{[][]int{{7, 7, 7}}, []int{7}},
	}

	for _, test := range tests {
		if got := getBiggestThree(test.grid); !reflect.DeepEqual(got, test.ans) {
			fmt.Println(test.grid, got, test.ans)
		}
	}
}
