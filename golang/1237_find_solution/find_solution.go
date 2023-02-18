/*
 * @Date: 2023-02-18
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-18
 * @FilePath: /algorithm/golang/1237_find_solution/find_solution.go
 */

package main

func findSolution(customFunction func(int, int) int, z int) (ans [][]int) {
	for x, y := 1, 1000; x <= 1000 && y > 0; x++ {
		for y > 0 && customFunction(x, y) > z {
			y--
		}
		if y > 0 && customFunction(x, y) == z {
			ans = append(ans, []int{x, y})
		}
	}
	return
}
