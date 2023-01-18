/*
 * @Date: 2022-06-19 16:53:41
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-19 17:24:15
 * @FilePath: /algorithm/508_find_frequent_tree_sum/find_frequent_tree_sum.go
 */

package main

import "reflect"

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func findFrequentTreeSum(root *TreeNode) (ans []int) {
	cnt := map[int]int{}
	maxCnt := 0
	var dfs func(*TreeNode) int
	dfs = func(node *TreeNode) int {
		if node == nil {
			return 0
		}
		sum := node.Val + dfs(node.Left) + dfs(node.Right)
		cnt[sum]++
		if cnt[sum] > maxCnt {
			maxCnt = cnt[sum]
		}
		return sum
	}
	dfs(root)

	for s, c := range cnt {
		if c == maxCnt {
			ans = append(ans, s)
		}
	}
	return
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		root := &TreeNode{5, &TreeNode{2, nil, nil}, &TreeNode{-3, nil, nil}}
		ans := []int{2, -3, 4}
		assert(findFrequentTreeSum(root), ans)
	}
	{
		root := &TreeNode{5, &TreeNode{2, nil, nil}, &TreeNode{-5, nil, nil}}
		ans := []int{2}
		assert(findFrequentTreeSum(root), ans)
	}
}
