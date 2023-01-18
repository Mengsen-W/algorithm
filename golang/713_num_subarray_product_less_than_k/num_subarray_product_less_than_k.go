/*
 * @Date: 2022-05-05 09:40:17
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-05 09:41:33
 * @FilePath: /algorithm/713_num_subarray_product_less_than_k/num_subarray_product_less_than_k.go
 */

package main

func numSubarrayProductLessThanK(nums []int, k int) (ans int) {
	prod, i := 1, 0
	for j, num := range nums {
		prod *= num
		for ; i <= j && prod >= k; i++ {
			prod /= nums[i]
		}
		ans += j - i + 1
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(numSubarrayProductLessThanK([]int{10, 5, 2, 6}, 100) == 8)
	assert(numSubarrayProductLessThanK([]int{1, 2, 3}, 0) == 0)
}
