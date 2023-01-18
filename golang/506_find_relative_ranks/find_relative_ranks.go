/*
 * @Date: 2021-12-02 06:06:11
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-02 06:09:52
 */

package main

import (
	"reflect"
	"sort"
	"strconv"
)

var desc = [3]string{"Gold Medal", "Silver Medal", "Bronze Medal"}

func findRelativeRanks(score []int) []string {
	n := len(score)
	type pair struct{ score, idx int }
	arr := make([]pair, n)
	for i, s := range score {
		arr[i] = pair{s, i}
	}
	sort.Slice(arr, func(i, j int) bool { return arr[i].score > arr[j].score })

	ans := make([]string, n)
	for i, p := range arr {
		if i < 3 {
			ans[p.idx] = desc[i]
		} else {
			ans[p.idx] = strconv.Itoa(i + 1)
		}
	}
	return ans
}

func main() {
	assert := func(a, b []string) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	assert(findRelativeRanks([]int{5, 4, 3, 2, 1}), []string{"Gold Medal", "Silver Medal", "Bronze Medal", "4", "5"})
	assert(findRelativeRanks([]int{10, 3, 8, 9, 4}), []string{"Gold Medal", "5", "Bronze Medal", "Silver Medal", "4"})
}
