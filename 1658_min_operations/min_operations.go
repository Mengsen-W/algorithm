/*
 * @Date: 2023-01-07
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-07
 * @FilePath: /algorithm/1658_min_operations/min_operations.go
 */

package main

func minOperations(nums []int, x int) int {
	min := func(a, b int) int {
		if b < a {
			return b
		}
		return a
	}
	n := len(nums)
	sum := 0
	for _, num := range nums {
		sum += num
	}
	if sum < x {
		return -1
	}

	right := 0
	lsum := 0
	rsum := sum
	ans := n + 1

	for left := -1; left < n; left++ {
		if left != -1 {
			lsum += nums[left]
		}
		for right < n && lsum+rsum > x {
			rsum -= nums[right]
			right++
		}
		if lsum+rsum == x {
			ans = min(ans, (left+1)+(n-right))
		}
	}
	if ans > n {
		return -1
	}
	return ans
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		nums := []int{1, 1, 4, 2, 3}
		x := 5
		ans := 2
		assert(minOperations(nums, x) == ans)
	}

	{
		nums := []int{5, 6, 7, 8, 9}
		x := 4
		ans := -1
		assert(minOperations(nums, x) == ans)
	}

	{
		nums := []int{3, 2, 20, 1, 1, 3}
		x := 10
		ans := 5
		assert(minOperations(nums, x) == ans)
	}
}
