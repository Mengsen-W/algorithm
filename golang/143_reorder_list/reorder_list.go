/*
 * @Date: 2023-07-31
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-31
 * @FilePath: /algorithm/golang/143_reorder_list/reorder_list.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

// Definition for singly-linked list.
type ListNode struct {
	Val  int
	Next *ListNode
}

func reorderList(head *ListNode) {
	if head == nil {
		return
	}
	nodes := []*ListNode{}
	for node := head; node != nil; node = node.Next {
		nodes = append(nodes, node)
	}
	i, j := 0, len(nodes)-1
	for i < j {
		nodes[i].Next = nodes[j]
		i++
		if i == j {
			break
		}
		nodes[j].Next = nodes[i]
		j--
	}
	nodes[i].Next = nil
}

func listToVec(head *ListNode) []int {
	step := head
	ans := []int{}
	for step != nil {
		ans = append(ans, step.Val)
		step = step.Next
	}
	return ans
}

func vecToList(vec []int) *ListNode {
	size := len(vec)
	if size == 0 {
		return nil
	}
	res := &ListNode{}
	cur := res
	for i := 0; i < size; i++ {
		cur.Val = vec[i]
		if i == size-1 {
			cur.Next = nil
		} else {
			cur.Next = &ListNode{}
		}
		cur = cur.Next
	}
	return res
}

func main() {
	tests := []struct {
		list *ListNode
		ans  []int
	}{
		{vecToList([]int{1, 2, 3, 4}), []int{1, 4, 2, 3}},
		{vecToList([]int{1, 2, 3, 4, 5}), []int{1, 5, 2, 4, 3}},
	}

	for _, item := range tests {
		reorderList(item.list)
		assert.Equal(&testing.T{}, item.ans, listToVec(item.list))
	}
}
