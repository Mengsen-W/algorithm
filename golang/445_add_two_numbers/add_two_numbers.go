/*
 * @Date: 2023-07-03
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-03
 * @FilePath: /algorithm/golang/445_add_two_numbers/add_two_numbers.go
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

func reverseList(head *ListNode) *ListNode {
	if head == nil || head.Next == nil {
		return head
	}
	newHead := reverseList(head.Next)
	head.Next.Next = head // 把下一个节点指向自己
	head.Next = nil       // 断开指向下一个节点的连接，保证最终链表的末尾节点的 next 是空节点
	return newHead
}

// l1 和 l2 为当前遍历的节点，carry 为进位
func addTwo(l1, l2 *ListNode, carry int) *ListNode {
	if l1 == nil && l2 == nil { // 递归边界：l1 和 l2 都是空节点
		if carry != 0 {
			return &ListNode{Val: carry} // 如果进位了，就额外创建一个节点
		}
		return nil
	}
	if l1 == nil { // 如果 l1 是空的，那么此时 l2 一定不是空节点
		l1, l2 = l2, l1 // 交换 l1 与 l2，保证 l1 非空，从而简化代码
	}
	carry += l1.Val // 节点值和进位加在一起
	if l2 != nil {
		carry += l2.Val // 节点值和进位加在一起
		l2 = l2.Next    // 下一个节点
	}
	l1.Val = carry % 10                     // 每个节点保存一个数位
	l1.Next = addTwo(l1.Next, l2, carry/10) // 进位
	return l1
}

func addTwoNumbers(l1, l2 *ListNode) *ListNode {
	l1 = reverseList(l1)
	l2 = reverseList(l2) // l1 和 l2 反转后，就变成【2. 两数相加】了
	l3 := addTwo(l1, l2, 0)
	return reverseList(l3)
}

func arrayToListNode(arr []int) *ListNode {
	if len(arr) == 0 {
		return nil
	}
	res := &ListNode{}
	cur := res
	for i := 0; i < len(arr); i++ {
		cur.Val = arr[i]
		if i == len(arr)-1 {
			cur.Next = nil
		} else {
			cur.Next = &ListNode{}
		}
		cur = cur.Next
	}
	return res
}

func same(l1, l2 *ListNode) bool {
	if l1 == nil && l2 == nil {
		return true
	}
	if l1 != nil && l2 != nil {
		return l1.Val == l2.Val && same(l1.Next, l2.Next)
	}
	return false
}

func main() {
	testMap := []struct {
		l1  []int
		l2  []int
		ans []int
	}{
		{[]int{7, 2, 3, 4}, []int{5, 6, 4}, []int{7, 8, 0, 7}},
		{[]int{2, 3, 4}, []int{5, 6, 4}, []int{8, 0, 7}},
		{[]int{0}, []int{0}, []int{0}},
	}

	for _, item := range testMap {
		assert.True(&testing.T{}, same(addTwoNumbers(arrayToListNode(item.l1), arrayToListNode(item.l2)), arrayToListNode(item.ans)))
	}
}
