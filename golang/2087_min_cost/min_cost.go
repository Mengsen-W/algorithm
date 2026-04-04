// Package main ...
package main

func minCost(startPos []int, homePos []int, rowCosts []int, colCosts []int) int {
	r1, c1 := startPos[0], startPos[1]
	r2, c2 := homePos[0], homePos[1]
	res := 0 // 总代价

	// 移动至家所在行，判断行间移动方向并计算对应代价
	if r2 >= r1 {
		for i := r1 + 1; i <= r2; i++ {
			res += rowCosts[i]
		}
	} else {
		for i := r2; i < r1; i++ {
			res += rowCosts[i]
		}
	}

	// 移动至家所在位置，判断列间移动方向并计算对应代价
	if c2 >= c1 {
		for i := c1 + 1; i <= c2; i++ {
			res += colCosts[i]
		}
	} else {
		for i := c2; i < c1; i++ {
			res += colCosts[i]
		}
	}

	return res
}

func main() {
	tests := []struct {
		startPos []int
		homePos  []int
		rowCosts []int
		colCosts []int
		ans      int
	}{
		{[]int{1, 0}, []int{2, 3}, []int{5, 4, 3}, []int{8, 2, 6, 7}, 18},
		{[]int{0, 0}, []int{0, 0}, []int{5}, []int{26}, 0},
	}

	for _, test := range tests {
		ans := minCost(test.startPos, test.homePos, test.rowCosts, test.colCosts)
		if ans != test.ans {
			panic("bad")
		}
	}
}
