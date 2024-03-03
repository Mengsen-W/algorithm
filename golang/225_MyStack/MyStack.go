/*
 * @Date: 2024-03-03
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-03
 * @FilePath: /algorithm/golang/225_MyStack/MyStack.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

type MyStack struct {
	queue []int
}

/** Initialize your data structure here. */
func Constructor() (s MyStack) {
	return
}

/** Push element x onto stack. */
func (s *MyStack) Push(x int) {
	n := len(s.queue)
	s.queue = append(s.queue, x)
	for ; n > 0; n-- {
		s.queue = append(s.queue, s.queue[0])
		s.queue = s.queue[1:]
	}
}

/** Removes the element on top of the stack and returns that element. */
func (s *MyStack) Pop() int {
	v := s.queue[0]
	s.queue = s.queue[1:]
	return v
}

/** Get the top element. */
func (s *MyStack) Top() int {
	return s.queue[0]
}

/** Returns whether the stack is empty. */
func (s *MyStack) Empty() bool {
	return len(s.queue) == 0
}

func main() {
	myStack := MyStack{}
	myStack.Push(1)
	myStack.Push(2)
	assert.Equal(&testing.T{}, 2, myStack.Top()) // 返回 2
	assert.Equal(&testing.T{}, 2, myStack.Pop()) // 返回 2
	assert.False(&testing.T{}, myStack.Empty())  // 返回 False
}
