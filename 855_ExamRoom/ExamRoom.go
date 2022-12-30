/*
 * @Date: 2022-12-30
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-30
 * @FilePath: /algorithm/855_ExamRoom/ExamRoom.go
 */

package main

type ExamRoom struct {
	n int
	s []int
}

func Constructor(n int) ExamRoom {
	return ExamRoom{n: n, s: []int{}}
}

func (this *ExamRoom) Seat() int {
	student := 0
	idx := 0
	if len(this.s) > 0 {
		dist := this.s[0]
		pre := -1
		for i, v := range this.s {
			if pre != -1 {
				d := (v - pre) / 2
				if d > dist {
					dist = d
					student = pre + d
					idx = i
				}
			}
			pre = v
		}
		if this.n-1-this.s[len(this.s)-1] > dist {
			student = this.n - 1
			idx = len(this.s)
		}
	}
	this.s = append(this.s, 0)
	copy(this.s[idx+1:], this.s[idx:])
	this.s[idx] = student
	return student
}

func (this *ExamRoom) Leave(p int) {
	idx := 0
	for i := 0; i < len(this.s); i++ {
		if this.s[i] == p {
			idx = i
			break
		}
	}
	this.s = append(this.s[:idx], this.s[idx+1:]...)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	e := Constructor(10)
	assert(e.Seat() == 0)
	assert(e.Seat() == 9)
	assert(e.Seat() == 4)
	assert(e.Seat() == 2)
	e.Leave(4)
	assert(e.Seat() == 5)
}
