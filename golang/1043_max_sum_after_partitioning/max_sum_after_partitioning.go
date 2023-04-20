/*
 * @Date: 2023-04-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-20
 * @FilePath: /algorithm/golang/1043_max_sum_after_partitioning/max_sum_after_partitioning.go
 */

// Package main ...
package main

func maxSumAfterPartitioning(arr []int, k int) int {
	n := len(arr)
	d := make([]int, n+1)
	max := func(x, y int) int {
		if x > y {
			return x
		}
		return y
	}

	for i := 1; i <= n; i++ {
		maxValue := arr[i-1]
		for j := i - 1; j >= max(0, i-k); j-- {
			d[i] = max(d[i], d[j]+maxValue*(i-j))
			if j > 0 && arr[j-1] > maxValue {
				maxValue = arr[j-1]
			}
		}
	}
	return d[n]
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		arr := []int{1, 15, 7, 9, 2, 5, 10}
		k := 3
		ans := 84
		assert(maxSumAfterPartitioning(arr, k) == ans)
	}

	{
		arr := []int{1, 4, 1, 5, 7, 3, 6, 1, 9, 9, 3}
		k := 4
		ans := 83
		assert(maxSumAfterPartitioning(arr, k) == ans)
	}

	{
		arr := []int{1}
		k := 1
		ans := 1
		assert(maxSumAfterPartitioning(arr, k) == ans)
	}
}
