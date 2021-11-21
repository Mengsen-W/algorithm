/*
 * @Date: 2021-11-21 02:11:21
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-21 02:21:49
 * @FilePath: /algorithm/559_max_depth/max_depth.go
 * @Description: file content
 */

package main

type Node struct {
	Val      int
	Children []*Node
}

func maxDepth(root *Node) (ans int) {
	if root == nil {
		return
	}
	queue := []*Node{root}
	for len(queue) > 0 {
		q := queue
		queue = nil
		for _, node := range q {
			queue = append(queue, node.Children...)
		}
		ans++
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		root := &Node{1, []*Node{{2, nil}, {3, []*Node{{5, nil}, {6, nil}}}, {4, nil}}}
		assert(maxDepth(root) == 3)
	}
	{
		root := &Node{1, []*Node{{2, nil}, {3, []*Node{{6, nil}, {7, []*Node{{11, []*Node{{12, nil}}}}}}}, {4, []*Node{{8, []*Node{{12, nil}}}}}, {5, []*Node{{9, nil}, {10, nil}}}}}
		assert(maxDepth(root) == 5)
	}
}
