/*
 * @Date: 2022-11-30
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-30
 * @FilePath: /algorithm/895_FreqStack/FreqStack.go
 */

package main

type FreqStack struct {
	freq    map[int]int
	group   map[int][]int
	maxFreq int
}

func Constructor() FreqStack {
	return FreqStack{map[int]int{}, map[int][]int{}, 0}
}

func (f *FreqStack) Push(val int) {
	f.freq[val]++
	f.group[f.freq[val]] = append(f.group[f.freq[val]], val)
	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}
	f.maxFreq = max(f.maxFreq, f.freq[val])
}

func (f *FreqStack) Pop() int {
	val := f.group[f.maxFreq][len(f.group[f.maxFreq])-1]
	f.group[f.maxFreq] = f.group[f.maxFreq][:len(f.group[f.maxFreq])-1]
	f.freq[val]--
	if len(f.group[f.maxFreq]) == 0 {
		f.maxFreq--
	}
	return val
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	f := Constructor()
	f.Push(5)
	f.Push(7)
	f.Push(5)
	f.Push(7)
	f.Push(4)
	f.Push(5)
	assert(f.Pop() == 5)
	assert(f.Pop() == 7)
	assert(f.Pop() == 5)
	assert(f.Pop() == 4)
}
