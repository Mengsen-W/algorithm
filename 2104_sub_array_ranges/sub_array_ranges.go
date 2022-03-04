/*
 * @Date: 2022-03-04 00:51:17
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-04 00:59:32
 * @FilePath: /algorithm/2104_sub_array_ranges/sub_array_ranges.go
 */

package main

func subArrayRanges1(nums []int) (ans int64) {
	for i, num := range nums {
		minVal, maxVal := num, num
		for _, v := range nums[i+1:] {
			if v < minVal {
				minVal = v
			} else if v > maxVal {
				maxVal = v
			}
			ans += int64(maxVal - minVal)
		}
	}
	return
}

func subArrayRanges2(nums []int) int64 {
	n := len(nums)
	minLeft := make([]int, n)
	maxLeft := make([]int, n)
	var minStk, maxStk []int
	for i, num := range nums {
		for len(minStk) > 0 && nums[minStk[len(minStk)-1]] > num {
			minStk = minStk[:len(minStk)-1]
		}
		if len(minStk) > 0 {
			minLeft[i] = minStk[len(minStk)-1]
		} else {
			minLeft[i] = -1
		}
		minStk = append(minStk, i)

		// 如果 nums[maxStk[len(maxStk)-1]] == num, 那么根据定义，
		// nums[maxStk[len(maxStk)-1]] 逻辑上小于 num，因为 maxStk[len(maxStk)-1] < i
		for len(maxStk) > 0 && nums[maxStk[len(maxStk)-1]] <= num {
			maxStk = maxStk[:len(maxStk)-1]
		}
		if len(maxStk) > 0 {
			maxLeft[i] = maxStk[len(maxStk)-1]
		} else {
			maxLeft[i] = -1
		}
		maxStk = append(maxStk, i)
	}

	minRight := make([]int, n)
	maxRight := make([]int, n)
	minStk = minStk[:0]
	maxStk = maxStk[:0]
	for i := n - 1; i >= 0; i-- {
		num := nums[i]
		// 如果 nums[minStk[len(minStk)-1]] == num, 那么根据定义，
		// nums[minStk[len(minStk)-1]] 逻辑上大于 num，因为 minStk[len(minStk)-1] > i
		for len(minStk) > 0 && nums[minStk[len(minStk)-1]] >= num {
			minStk = minStk[:len(minStk)-1]
		}
		if len(minStk) > 0 {
			minRight[i] = minStk[len(minStk)-1]
		} else {
			minRight[i] = n
		}
		minStk = append(minStk, i)

		for len(maxStk) > 0 && nums[maxStk[len(maxStk)-1]] < num {
			maxStk = maxStk[:len(maxStk)-1]
		}
		if len(maxStk) > 0 {
			maxRight[i] = maxStk[len(maxStk)-1]
		} else {
			maxRight[i] = n
		}
		maxStk = append(maxStk, i)
	}

	var sumMax, sumMin int64
	for i, num := range nums {
		sumMax += int64(maxRight[i]-i) * int64(i-maxLeft[i]) * int64(num)
		sumMin += int64(minRight[i]-i) * int64(i-minLeft[i]) * int64(num)
	}
	return sumMax - sumMin
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(subArrayRanges1([]int{1, 2, 3}) == 4)
	assert(subArrayRanges2([]int{1, 2, 3}) == 4)

	assert(subArrayRanges1([]int{1, 3, 3}) == 4)
	assert(subArrayRanges2([]int{1, 3, 3}) == 4)

	assert(subArrayRanges1([]int{4, -2, -3, 4, 1}) == 59)
	assert(subArrayRanges2([]int{4, -2, -3, 4, 1}) == 59)
}
