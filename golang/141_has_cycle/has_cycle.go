/*
 * @Date: 2023-07-29
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-29
 * @FilePath: /algorithm/golang/141_has_cycle/has_cycle.go
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

func hasCycle(head *ListNode) bool {
	if head == nil || head.Next == nil {
		return false
	}
	slow, fast := head, head.Next
	for fast != slow {
		if fast == nil || fast.Next == nil {
			return false
		}
		slow = slow.Next
		fast = fast.Next.Next
	}
	return true
}

func main() {
	tests := []struct {
		head *ListNode
		ans  bool
	}{
		{&ListNode{0, nil}, false},
	}

	for _, item := range tests {
		assert.Equal(&testing.T{}, item.ans, hasCycle(item.head))
	}
}
