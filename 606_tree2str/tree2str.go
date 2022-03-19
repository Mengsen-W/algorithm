/*
 * @Date: 2022-03-18 23:49:25
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-19 00:08:01
 * @FilePath: /algorithm/606_tree2str/tree2str.go
 */

package main

import (
	"fmt"
	"reflect"
	"strconv"
	"strings"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func tree2str(root *TreeNode) string {
	switch {
	case root == nil:
		return ""
	case root.Left == nil && root.Right == nil:
		return strconv.Itoa(root.Val)
	case root.Right == nil:
		return fmt.Sprintf("%d(%s)", root.Val, tree2str(root.Left))
	default:
		return fmt.Sprintf("%d(%s)(%s)", root.Val, tree2str(root.Left), tree2str(root.Right))
	}
}

func tree2str1(root *TreeNode) string {
	ans := &strings.Builder{}
	st := []*TreeNode{root}
	vis := map[*TreeNode]bool{}
	for len(st) > 0 {
		node := st[len(st)-1]
		if vis[node] {
			if node != root {
				ans.WriteByte(')')
			}
			st = st[:len(st)-1]
		} else {
			vis[node] = true
			if node != root {
				ans.WriteByte('(')
			}
			ans.WriteString(strconv.Itoa(node.Val))
			if node.Left == nil && node.Right != nil {
				ans.WriteString("()")
			}
			if node.Right != nil {
				st = append(st, node.Right)
			}
			if node.Left != nil {
				st = append(st, node.Left)
			}
		}
	}
	return ans.String()
}

func main() {
	assert := func(a, b string) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	{
		root := &TreeNode{Val: 1, Left: &TreeNode{Val: 2, Left: &TreeNode{Val: 4}, Right: nil}, Right: &TreeNode{Val: 3}}
		assert(tree2str(root), "1(2(4))(3)")
		assert(tree2str1(root), "1(2(4))(3)")
	}

	{
		root := &TreeNode{Val: 1, Left: &TreeNode{Val: 2, Left: nil, Right: &TreeNode{Val: 4}}, Right: &TreeNode{Val: 3}}
		assert(tree2str(root), "1(2()(4))(3)")
		assert(tree2str1(root), "1(2()(4))(3)")
	}
}
