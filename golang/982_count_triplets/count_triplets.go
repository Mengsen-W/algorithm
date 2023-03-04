/*
 * @Date: 2023-03-04
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-04
 * @FilePath: /algorithm/golang/982_count_triplets/count_triplets.go
 */

package main

func countTriplets(nums []int) int {
	var cnt [1 << 16]int
	for i := range nums {
		for j := range nums {
			cnt[nums[i]&nums[j]]++
		}
	}
	res := 0
	for i := range nums {
		x := nums[i] ^ 0xffff
		for sub := x; sub > 0; sub = (sub - 1) & x {
			res += cnt[sub]
		}
		res += cnt[0]
	}
	return res
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		nums := []int{2, 1, 3}
		ans := 12
		assert(countTriplets(nums) == ans)
	}

	{
		nums := []int{0, 0, 0}
		ans := 27
		assert(countTriplets(nums) == ans)
	}
}
