/*
 * @Date: 2021-09-22 09:15:14
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-22 09:39:37
 */

package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func splitListToParts(head *ListNode, k int) []*ListNode {
	n := 0
	for node := head; node != nil; node = node.Next {
		n++
	}
	quotient, remainder := n/k, n%k

	parts := make([]*ListNode, k)
	for i, curr := 0, head; i < k && curr != nil; i++ {
		parts[i] = curr
		partSize := quotient
		if i < remainder {
			partSize++
		}
		for j := 1; j < partSize; j++ {
			curr = curr.Next
		}
		curr, curr.Next = curr.Next, nil
	}
	return parts
}

func main() {
	{
		head := &ListNode{1, &ListNode{2, &ListNode{3, nil}}}
		k := 5
		ans := splitListToParts(head, k)
		for _, head := range ans {
			i := head
			for i != nil {
				fmt.Println(i)
				i = i.Next
			}
		}
	}
	{
		head := &ListNode{1, &ListNode{2, &ListNode{3, &ListNode{4, &ListNode{5, &ListNode{6, &ListNode{7, &ListNode{8, &ListNode{9, &ListNode{10, nil}}}}}}}}}}
		k := 3
		ans := splitListToParts(head, k)
		for _, head := range ans {
			i := head
			for i != nil {
				fmt.Println(i)
				i = i.Next
			}
		}
	}
}
