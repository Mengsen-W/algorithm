/*
 * @Date: 2022-12-17
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-17
 * @FilePath: /algorithm/1764_can_choose/can_choose.go
 */

package main

func canChoose(groups [][]int, nums []int) bool {
	find := func(nums, g []int, k int) int {
		m, n := len(g), len(nums)
		if k+m > n {
			return -1
		}

		pi := make([]int, m)
		for i, j := 1, 0; i < m; i++ {
			for j > 0 && g[i] != g[j] {
				j = pi[j-1]
			}
			if g[i] == g[j] {
				j++
			}
			pi[i] = j
		}

		for i, j := k, 0; i < n; i++ {
			for j > 0 && nums[i] != g[j] {
				j = pi[j-1]
			}
			if nums[i] == g[j] {
				j++
			}
			if j == m {
				return i - m + 1
			}
		}
		return -1
	}
	k := 0
	for _, value := range groups {
		k = find(nums, value, k)
		if k == -1 {
			return false
		}
		k += len(value)
	}
	return true
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		groups := [][]int{{1, -1, -1}, {3, -2, 0}}
		nums := []int{1, -1, 0, 1, -1, -1, 3, -2, 0}
		assert(canChoose(groups, nums))
	}

	{
		groups := [][]int{{10, -2}, {1, 2, 3, 4}}
		nums := []int{1, 2, 3, 4, 10, -2}
		assert(!canChoose(groups, nums))
	}

	{
		groups := [][]int{{1, 2, 3}, {3, 4}}
		nums := []int{7, 7, 1, 2, 3, 4, 7, 7}
		assert(!canChoose(groups, nums))
	}
}
