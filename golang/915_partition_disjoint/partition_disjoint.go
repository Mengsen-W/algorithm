/*
 * @Date: 2022-10-24
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-24
 * @FilePath: /algorithm/915_partition_disjoint/partition_disjoint.go
 */

package main

func partitionDisjoint(nums []int) int {
	n := len(nums)
	leftMax, leftPos, curMax := nums[0], 0, nums[0]
	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}
	for i := 1; i < n-1; i++ {
		curMax = max(curMax, nums[i])
		if nums[i] < leftMax {
			leftMax = curMax
			leftPos = i
		}
	}
	return leftPos + 1
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		nums := []int{5, 0, 3, 8, 6}
		ans := 3
		assert(partitionDisjoint(nums) == ans)
	}

	{
		nums := []int{1, 1, 1, 0, 6, 12}
		ans := 4
		assert(partitionDisjoint(nums) == ans)
	}
}
