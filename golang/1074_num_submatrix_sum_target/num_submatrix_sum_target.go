/*
 * @Date: 2021-05-29 09:38:37
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-29 09:54:30
 */
package main

func numSubmatrixSumTarget(matrix [][]int, target int) (ans int) {
	subarraySum := func(nums []int, k int) (ans int) {
		mp := map[int]int{0: 1}
		for i, pre := 0, 0; i < len(nums); i++ {
			pre += nums[i]
			if _, ok := mp[pre-k]; ok {
				ans += mp[pre-k]
			}
			mp[pre]++
		}
		return
	}
	for i := range matrix { // 枚举上边界
		sum := make([]int, len(matrix[0]))
		for _, row := range matrix[i:] { // 枚举下边界
			for c, v := range row {
				sum[c] += v // 更新每列的元素和
			}
			ans += subarraySum(sum, target)
		}
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed!")
		}
	}

	assert(numSubmatrixSumTarget([][]int{{0, 1, 0}, {1, 1, 1}, {0, 1, 0}}, 0) == 4)
	assert(numSubmatrixSumTarget([][]int{{1, -1}, {-1, 1}}, 0) == 5)
	assert(numSubmatrixSumTarget([][]int{{904}}, 0) == 0)
}
