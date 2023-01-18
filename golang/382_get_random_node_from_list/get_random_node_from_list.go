/*
 * @Date: 2022-01-16 02:48:35
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-16 02:57:10
 */

package main

import "math/rand"

// Definition for singly-linked list.
type ListNode struct {
	Val  int
	Next *ListNode
}

// type Solution []int

// func Constructor(head *ListNode) (s Solution) {
// 	for node := head; node != nil; node = node.Next {
// 		s = append(s, node.Val)
// 	}
// 	return s
// }

// func (s Solution) GetRandom() int {
// 	return s[rand.Intn(len(s))]
// }

type Solution struct {
	head *ListNode
}

func Constructor(head *ListNode) Solution {
	return Solution{head}
}

func (s *Solution) GetRandom() (ans int) {
	for node, i := s.head, 1; node != nil; node = node.Next {
		if rand.Intn(i) == 0 { // 1/i 的概率选中（替换为答案）
			ans = node.Val
		}
		i++
	}
	return
}

func main() {}
