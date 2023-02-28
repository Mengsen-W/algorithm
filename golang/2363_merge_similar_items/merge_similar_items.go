/*
 * @Date: 2023-02-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-28
 * @FilePath: /algorithm/golang/2363_merge_similar_items/merge_similar_items.go
 */

package main

import (
	"reflect"
	"sort"
)

func mergeSimilarItems(item1, item2 [][]int) [][]int {
	mp := map[int]int{}
	for _, a := range item1 {
		mp[a[0]] += a[1]
	}
	for _, a := range item2 {
		mp[a[0]] += a[1]
	}
	var ans [][]int
	for a, b := range mp {
		ans = append(ans, []int{a, b})
	}
	sort.Slice(ans, func(i, j int) bool {
		return ans[i][0] < ans[j][0]
	})
	return ans
}

func main() {
	assert := func(a, b [][]int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		items1 := [][]int{{1, 1}, {4, 5}, {3, 8}}
		items2 := [][]int{{3, 1}, {1, 5}}
		ans := [][]int{{1, 6}, {3, 9}, {4, 5}}
		assert(mergeSimilarItems(items1, items2), ans)
	}

	{
		items1 := [][]int{{1, 1}, {3, 2}, {2, 3}}
		items2 := [][]int{{2, 1}, {3, 2}, {1, 3}}
		ans := [][]int{{1, 4}, {2, 4}, {3, 4}}
		assert(mergeSimilarItems(items1, items2), ans)
	}

	{
		items1 := [][]int{{1, 3}, {2, 2}}
		items2 := [][]int{{7, 1}, {2, 2}, {1, 4}}
		ans := [][]int{{1, 7}, {2, 4}, {7, 1}}
		assert(mergeSimilarItems(items1, items2), ans)
	}
}
