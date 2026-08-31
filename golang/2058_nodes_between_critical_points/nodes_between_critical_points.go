// Package main ...
package main

import (
	"fmt"
	"reflect"
)

// Definition for singly-linked list.
type ListNode struct {
	Val  int
	Next *ListNode
}

func nodesBetweenCriticalPoints(head *ListNode) []int {
	minDist, maxDist := -1, -1
	first, last, pos := -1, -1, 0
	cur := head
	for cur.Next.Next != nil {
		// 获取连续的三个节点的值
		x := cur.Val
		y := cur.Next.Val
		z := cur.Next.Next.Val
		// 如果 y 是临界点
		if y > max(x, z) || y < min(x, z) {
			if last != -1 {
				// 用相邻临界点的距离更新最小值
				if minDist == -1 {
					minDist = pos - last
				} else {
					minDist = min(minDist, pos-last)
				}
				// 用到第一个临界点的距离更新最大值
				maxDist = max(maxDist, pos-first)
			}
			if first == -1 {
				first = pos
			}
			// 更新上一个临界点
			last = pos
		}
		cur = cur.Next
		pos++
	}
	return []int{minDist, maxDist}
}

func main() {
	tests := []struct {
		head *ListNode
		ans  []int
	}{
		{&ListNode{3, &ListNode{1, nil}}, []int{-1, -1}},
		{
			&ListNode{
				5, &ListNode{3, &ListNode{1, &ListNode{2, &ListNode{5, &ListNode{1, &ListNode{2, nil}}}}}},
			},
			[]int{1, 3},
		},
		{
			&ListNode{
				1,
				&ListNode{
					3,
					&ListNode{
						2,
						&ListNode{
							2, &ListNode{3, &ListNode{2, &ListNode{2, &ListNode{2, &ListNode{7, nil}}}}},
						},
					},
				},
			},
			[]int{3, 3},
		},
		{&ListNode{2, &ListNode{3, &ListNode{3, &ListNode{2, nil}}}}, []int{-1, -1}},
	}

	for index, test := range tests{
		if !reflect.DeepEqual(nodesBetweenCriticalPoints(test.head), test.ans){
			fmt.Println(index)
		}
	}
}
