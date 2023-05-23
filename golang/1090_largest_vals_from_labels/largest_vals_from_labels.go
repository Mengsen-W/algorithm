/*
 * @Date: 2023-05-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-23
 * @FilePath: /algorithm/golang/1090_largest_vals_from_labels/largest_vals_from_labels.go
 */

// Package main ...
package main

import "sort"

func largestValsFromLabels(values []int, labels []int, numWanted int, useLimit int) int {
	n := len(values)
	idx := make([]int, n)
	for i := 0; i < n; i++ {
		idx[i] = i
	}
	sort.Slice(idx, func(i, j int) bool {
		return values[idx[i]] > values[idx[j]]
	})

	ans, choose := 0, 0
	cnt := make(map[int]int)
	for i := 0; i < n; i++ {
		label := labels[idx[i]]
		if cnt[label] == useLimit {
			continue
		}
		choose++
		ans += values[idx[i]]
		cnt[label]++
		if choose == numWanted {
			break
		}
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
		values := []int{5, 4, 3, 2, 1}
		labels := []int{1, 1, 2, 2, 3}
		numWanted := 3
		useLimit := 1
		ans := 9
		assert(largestValsFromLabels(values, labels, numWanted, useLimit) == ans)
	}

	{
		values := []int{5, 4, 3, 2, 1}
		labels := []int{1, 3, 3, 3, 2}
		numWanted := 3
		useLimit := 2
		ans := 12
		assert(largestValsFromLabels(values, labels, numWanted, useLimit) == ans)
	}

	{
		values := []int{9, 8, 8, 7, 6}
		labels := []int{0, 0, 0, 1, 1}
		numWanted := 3
		useLimit := 1
		ans := 16
		assert(largestValsFromLabels(values, labels, numWanted, useLimit) == ans)
	}
}
