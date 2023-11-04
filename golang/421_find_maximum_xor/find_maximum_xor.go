/*
 * @Date: 2021-05-16 09:52:02
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-04
 */

package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

const highBit = 30

type trie struct {
	left, right *trie
}

func (t *trie) add(num int) {
	cur := t
	for i := highBit; i >= 0; i-- {
		bit := num >> i & 1
		if bit == 0 {
			if cur.left == nil {
				cur.left = &trie{}
			}
			cur = cur.left
		} else {
			if cur.right == nil {
				cur.right = &trie{}
			}
			cur = cur.right
		}
	}
}

func (t *trie) check(num int) (x int) {
	cur := t
	for i := highBit; i >= 0; i-- {
		bit := num >> i & 1
		if bit == 0 {
			// a_i 的第 k 个二进制位为 0，应当往表示 1 的子节点 right 走
			if cur.right != nil {
				cur = cur.right
				x = x*2 + 1
			} else {
				cur = cur.left
				x = x * 2
			}
		} else {
			// a_i 的第 k 个二进制位为 1，应当往表示 0 的子节点 left 走
			if cur.left != nil {
				cur = cur.left
				x = x*2 + 1
			} else {
				cur = cur.right
				x = x * 2
			}
		}
	}
	return
}

func findMaximumXOR(nums []int) (x int) {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	root := &trie{}
	for i := 1; i < len(nums); i++ {
		// 将 nums[i-1] 放入字典树，此时 nums[0 .. i-1] 都在字典树中
		root.add(nums[i-1])
		// 将 nums[i] 看作 ai，找出最大的 x 更新答案
		x = max(x, root.check(nums[i]))
	}
	return
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{3, 10, 5, 25, 2, 8}, 28},
		{[]int{0}, 0},
		{[]int{2, 4}, 6},
		{[]int{8, 10, 2}, 10},
		{[]int{14, 70, 53, 83, 49, 91, 36, 80, 92, 51, 66, 70}, 127},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, findMaximumXOR(test.nums), index)
	}
}
