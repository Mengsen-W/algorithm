/*
 * @Date: 2022-04-08 01:51:08
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-08 03:17:11
 * @FilePath: /algorithm/429_N_tree_level_order/N_tree_level_order.go
 */

package main

import "reflect"

// Definition for a Node.
type Node struct {
	Val      int
	Children []*Node
}

func levelOrder(root *Node) (ans [][]int) {
	if root == nil {
		return
	}
	q := []*Node{root}
	for q != nil {
		level := []int{}
		tmp := q
		q = nil
		for _, node := range tmp {
			level = append(level, node.Val)
			q = append(q, node.Children...)
		}
		ans = append(ans, level)
	}
	return
}

func main() {
	assert := func(a, b [][]int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	assert(levelOrder(&Node{Val: 1, Children: []*Node{{Val: 3, Children: []*Node{{Val: 5, Children: nil}, {Val: 6, Children: nil}}}, {Val: 2, Children: nil}, {Val: 4, Children: nil}}}), [][]int{{1}, {3, 2, 4}, {5, 6}})
	assert(levelOrder(&Node{Val: 1, Children: []*Node{{Val: 2, Children: nil}, {Val: 3, Children: []*Node{{Val: 6, Children: nil}, {Val: 7, Children: []*Node{{Val: 11, Children: []*Node{{Val: 14, Children: nil}}}}}}}, {Val: 4, Children: []*Node{{Val: 8, Children: []*Node{{Val: 12, Children: nil}}}}}, {Val: 5, Children: []*Node{{Val: 9, Children: []*Node{{Val: 13, Children: nil}}}, {Val: 10, Children: nil}}}}}), [][]int{{1}, {2, 3, 4, 5}, {6, 7, 8, 9, 10}, {11, 12, 13}, {14}})
}
