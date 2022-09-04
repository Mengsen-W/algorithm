/*
 * @Date: 2022-09-04
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-04
 * @FilePath: /algorithm/1582_num_special/num_special.go
 */

package main

func numSpecial(mat [][]int) (ans int) {
	for i, row := range mat {
		cnt1 := 0
		for _, x := range row {
			cnt1 += x
		}
		if i == 0 {
			cnt1--
		}
		if cnt1 > 0 {
			for j, x := range row {
				if x == 1 {
					mat[0][j] += cnt1
				}
			}
		}
	}

	for _, x := range mat[0] {
		if x == 1 {
			ans++
		}
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		mat := [][]int{{1, 0, 0}, {0, 0, 1}, {1, 0, 0}}
		ans := 1
		assert(numSpecial(mat) == ans)
	}

	{
		mat := [][]int{{1, 0, 0}, {0, 1, 0}, {0, 0, 1}}
		ans := 3
		assert(numSpecial(mat) == ans)
	}

	{
		mat := [][]int{{0, 0, 0, 1}, {1, 0, 0, 0}, {0, 1, 1, 0}, {0, 0, 0, 0}}
		ans := 2
		assert(numSpecial(mat) == ans)
	}

	{
		mat := [][]int{{0, 0, 0, 0, 0}, {1, 0, 0, 0, 0}, {0, 1, 0, 0, 0}, {0, 0, 1, 0, 0}, {0, 0, 0, 1, 1}}
		ans := 3
		assert(numSpecial(mat) == ans)
	}
}
