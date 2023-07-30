/*
 * @Date: 2023-07-30
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-30
 * @FilePath: /algorithm/golang/142_detect_cycle/detect_cycle.go
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

func detectCycle(head *ListNode) *ListNode {
	seen := map[*ListNode]struct{}{}
	for head != nil {
		if _, ok := seen[head]; ok {
			return head
		}
		seen[head] = struct{}{}
		head = head.Next
	}
	return nil
}

func main() {
	{
		head := &ListNode{3, &ListNode{2, &ListNode{0, &ListNode{-4, nil}}}}
		head.Next.Next.Next.Next = head.Next
		assert.Equal(&testing.T{}, detectCycle(head), head.Next)
	}

	{
		head := &ListNode{1, &ListNode{2, nil}}
		head.Next = head
		assert.Equal(&testing.T{}, detectCycle(head), head)
	}

	{
		head := &ListNode{1, nil}
		assert.Equal(&testing.T{}, detectCycle(head), nil)
	}
}
