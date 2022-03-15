/*
 * @Date: 2022-03-15 00:28:17
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-15 00:53:10
 * @FilePath: /algorithm/2044_count_max_or_subsets/count_max_or_subsets.go
 */

package main

import "reflect"

func countMaxOrSubsets(nums []int) (ans int) {
	maxOr := 0
	var dfs func(int, int)
	dfs = func(pos, or int) {
		if pos == len(nums) {
			if or > maxOr {
				maxOr = or
				ans = 1
			} else if or == maxOr {
				ans++
			}
			return
		}
		dfs(pos+1, or|nums[pos])
		dfs(pos+1, or)
	}
	dfs(0, 0)
	return
}

func main() {
	assert := func(a, b int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		input := []int{3, 1}
		ans := 2
		assert(countMaxOrSubsets(input), ans)
	}

	{
		input := []int{2, 2, 2}
		ans := 7
		assert(countMaxOrSubsets(input), ans)
	}

	{
		input := []int{3, 2, 1, 5}
		ans := 6
		assert(countMaxOrSubsets(input), ans)
	}
}
