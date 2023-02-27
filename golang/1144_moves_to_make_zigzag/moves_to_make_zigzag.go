/*
 * @Date: 2023-02-27
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-27
 * @FilePath: /algorithm/golang/1144_moves_to_make_zigzag/moves_to_make_zigzag.go
 */

package main

func movesToMakeZigzag(nums []int) int {
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
	help := func(pos int) int {
		res := 0
		for i := pos; i < len(nums); i += 2 {
			a := 0
			if i-1 >= 0 {
				a = max(a, nums[i]-nums[i-1]+1)
			}
			if i+1 < len(nums) {
				a = max(a, nums[i]-nums[i+1]+1)
			}
			res += a
		}
		return res
	}

	return min(help(0), help(1))
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		nums := []int{1, 2, 3}
		ans := 2
		assert(movesToMakeZigzag(nums) == ans)
	}

	{
		nums := []int{9, 6, 1, 6, 2}
		ans := 4
		assert(movesToMakeZigzag(nums) == ans)
	}
}
