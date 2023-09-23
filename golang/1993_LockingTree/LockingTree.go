/*
 * @Date: 2023-09-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-23
 * @FilePath: /algorithm/golang/1993_LockingTree/LockingTree.go
 */

package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

type LockingTree struct {
	parent       []int
	lockNodeUser []int
	children     [][]int
}

func Constructor(parent []int) LockingTree {
	n := len(parent)
	lockNodeUser := make([]int, n)
	children := make([][]int, n)
	for i := 0; i < n; i++ {
		lockNodeUser[i] = -1
		p := parent[i]
		if p != -1 {
			children[p] = append(children[p], i)
		}
	}
	return LockingTree{parent, lockNodeUser, children}
}

func (this *LockingTree) Lock(num int, user int) bool {
	if this.lockNodeUser[num] == -1 {
		this.lockNodeUser[num] = user
		return true
	}
	return false
}

func (this *LockingTree) Unlock(num int, user int) bool {
	if this.lockNodeUser[num] == user {
		this.lockNodeUser[num] = -1
		return true
	}
	return false
}

func (this *LockingTree) Upgrade(num int, user int) bool {
	res := this.lockNodeUser[num] == -1 && !this.hasLockedAncestor(num) && this.checkAndUnlockDescendant(num)
	if res {
		this.lockNodeUser[num] = user
	}
	return res
}

func (this *LockingTree) hasLockedAncestor(num int) bool {
	num = this.parent[num]
	for num != -1 {
		if this.lockNodeUser[num] != -1 {
			return true
		}
		num = this.parent[num]
	}
	return false
}

func (this *LockingTree) checkAndUnlockDescendant(num int) bool {
	res := this.lockNodeUser[num] != -1
	this.lockNodeUser[num] = -1
	for _, child := range this.children[num] {
		if this.checkAndUnlockDescendant(child) {
			res = true
		}
	}
	return res
}

func main() {
	t := &testing.T{}
	lockingTree := Constructor([]int{-1, 0, 0, 1, 1, 2})
	assert.True(t, lockingTree.Lock(2, 2))
	assert.True(t, !lockingTree.Unlock(2, 3))
	assert.True(t, lockingTree.Unlock(2, 2))
	assert.True(t, lockingTree.Lock(4, 5))
	assert.True(t, lockingTree.Upgrade(0, 1))
	assert.True(t, !lockingTree.Lock(0, 1))
}
