/*
 * @Date: 2022-06-06 09:37:28
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-06 09:52:22
 * @FilePath: /algorithm/732_my_calendar_three/my_calendar_three.go
 */

package main

type pair struct{ num, lazy int }

type MyCalendarThree map[int]pair

func Constructor() MyCalendarThree {
	return MyCalendarThree{}
}

func (t MyCalendarThree) update(start, end, l, r, idx int) {
	if r < start || end < l {
		return
	}
	if start <= l && r <= end {
		p := t[idx]
		p.num++
		p.lazy++
		t[idx] = p
	} else {
		mid := (l + r) / 2
		t.update(start, end, l, mid, idx*2)
		t.update(start, end, mid+1, r, idx*2+1)
		p := t[idx]
		p.num = p.lazy + max(t[idx*2].num, t[idx*2+1].num)
		t[idx] = p
	}
}

func (t MyCalendarThree) Book(start, end int) int {
	t.update(start, end-1, 0, 1e9, 1)
	return t[1].num
}

func max(a, b int) int {
	if b > a {
		return b
	}
	return a
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	my_calendar_three := &MyCalendarThree{}
	assert(my_calendar_three.Book(10, 20) == 1)
	assert(my_calendar_three.Book(50, 60) == 1)
	assert(my_calendar_three.Book(10, 40) == 2)
	assert(my_calendar_three.Book(5, 15) == 3)
	assert(my_calendar_three.Book(5, 10) == 3)
	assert(my_calendar_three.Book(25, 55) == 3)
}
