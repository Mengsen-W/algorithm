/*
 * @Date: 2023-04-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-28
 * @FilePath: /algorithm/golang/1172_DinnerPlates/DinnerPlates.go
 */

// Package main ...
package main

import "sort"

type DinnerPlates struct {
	capacity  int
	stack     []int
	top       []int
	poppedPos []int
}

func Constructor(capacity int) DinnerPlates {
	return DinnerPlates{capacity, []int{}, []int{}, []int{}}
}

func (this *DinnerPlates) Push(val int) {
	if len(this.poppedPos) == 0 {
		pos := len(this.stack)
		this.stack = append(this.stack, val)
		if pos%this.capacity == 0 {
			this.top = append(this.top, 0)
		} else {
			stackPos := len(this.top) - 1
			stackTop := this.top[stackPos]
			this.top[stackPos] = stackTop + 1
		}
	} else {
		pos := this.poppedPos[0]
		this.poppedPos = this.poppedPos[1:]
		this.stack[pos] = val
		index := pos / this.capacity
		stackTop := this.top[index]
		this.top[index] = stackTop + 1
	}
}

func (this *DinnerPlates) Pop() int {
	for len(this.stack) > 0 && len(this.poppedPos) > 0 && this.poppedPos[len(this.poppedPos)-1] == len(this.stack)-1 {
		this.stack = this.stack[:len(this.stack)-1]
		pos := this.poppedPos[len(this.poppedPos)-1]
		this.poppedPos = this.poppedPos[:len(this.poppedPos)-1]
		if pos%this.capacity == 0 {
			this.top = this.top[:len(this.top)-1]
		}
	}
	if len(this.stack) == 0 {
		return -1
	} else {
		pos := len(this.stack) - 1
		val := this.stack[pos]
		this.stack = this.stack[:pos]
		if pos%this.capacity == 0 && len(this.top) > 0 {
			this.top = this.top[:len(this.top)-1]
		} else if len(this.top) > 0 {
			this.top[len(this.top)-1] -= 1
		}
		return val
	}
}

func (this *DinnerPlates) PopAtStack(index int) int {
	if index >= len(this.top) {
		return -1
	}
	stackTop := this.top[index]
	if stackTop < 0 {
		return -1
	}
	this.top[index] = stackTop - 1
	pos := index*this.capacity + stackTop
	i := sort.SearchInts(this.poppedPos, pos)
	this.poppedPos = append(this.poppedPos, 0)
	copy(this.poppedPos[i+1:], this.poppedPos[i:])
	this.poppedPos[i] = pos
	return this.stack[pos]
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	d := Constructor(2)
	d.Push(1)
	d.Push(2)
	d.Push(3)
	d.Push(4)
	d.Push(5)
	assert(d.PopAtStack(0) == 2)
	d.Push(20)
	d.Push(21)
	assert(d.PopAtStack(0) == 20)
	assert(d.PopAtStack(2) == 21)
	assert(d.Pop() == 5)
	assert(d.Pop() == 4)
	assert(d.Pop() == 3)
	assert(d.Pop() == 1)
	assert(d.Pop() == -1)
}
