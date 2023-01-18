/*
 * @Date: 2023-01-17
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-17
 * @FilePath: /algorithm/1814_count_nice_pairs/count_nice_pairs.go
 */

package main

func countNicePairs(nums []int) (ans int) {
	cnt := map[int]int{}
	for _, num := range nums {
		rev := 0
		for x := num; x > 0; x /= 10 {
			rev = rev*10 + x%10
		}
		ans += cnt[num-rev]
		cnt[num-rev]++
	}
	return ans % (1e9 + 7)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		nums := []int{42, 11, 1, 97}
		ans := 2
		assert(countNicePairs(nums) == ans)
	}

	{
		nums := []int{13, 10, 35, 24, 76}
		ans := 4
		assert(countNicePairs(nums) == ans)
	}
}
