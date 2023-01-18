/*
 * @Date: 2022-01-14 01:23:07
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-14 02:17:07
 */

package main

import (
	"reflect"
	"sort"
)

func kSmallestPairs(nums1, nums2 []int, k int) (ans [][]int) {
	m, n := len(nums1), len(nums2)

	// 二分查找第 k 小的数对和
	left, right := nums1[0]+nums2[0], nums1[m-1]+nums2[n-1]+1
	pairSum := left + sort.Search(right-left, func(sum int) bool {
		sum += left
		cnt := 0
		i, j := 0, n-1
		for i < m && j >= 0 {
			if nums1[i]+nums2[j] > sum {
				j--
			} else {
				cnt += j + 1
				i++
			}
		}
		return cnt >= k
	})

	// 找数对和小于 pairSum 的数对
	i := n - 1
	for _, num1 := range nums1 {
		for i >= 0 && num1+nums2[i] >= pairSum {
			i--
		}
		for _, num2 := range nums2[:i+1] {
			ans = append(ans, []int{num1, num2})
			if len(ans) == k {
				return
			}
		}
	}

	// 找数对和等于 pairSum 的数对
	i = n - 1
	for _, num1 := range nums1 {
		for i >= 0 && num1+nums2[i] > pairSum {
			i--
		}
		for j := i; j >= 0 && num1+nums2[j] == pairSum; j-- {
			ans = append(ans, []int{num1, nums2[j]})
			if len(ans) == k {
				return
			}
		}
	}
	return
}

func main() {
	assert := func(a, b [][]int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	assert(kSmallestPairs([]int{1, 7, 11}, []int{2, 4, 6}, 3), [][]int{{1, 2}, {1, 4}, {1, 6}})
	assert(kSmallestPairs([]int{1, 1, 2}, []int{1, 2, 3}, 2), [][]int{{1, 1}, {1, 1}})
	assert(kSmallestPairs([]int{1, 2}, []int{3}, 3), [][]int{{1, 3}, {2, 3}})
}
