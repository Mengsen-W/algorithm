/*
 * @Date: 2024-03-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-21
 * @FilePath: /algorithm/golang/2671_FrequencyTracker/FrequencyTracker.go
 */

// Package main ...
package main

type FrequencyTracker struct {
	freq     map[int]int
	freq_cnt map[int]int
}

func Constructor() FrequencyTracker {
	return FrequencyTracker{map[int]int{}, map[int]int{}}
}

func (this *FrequencyTracker) Add(number int) {
	(*this).freq_cnt[(*this).freq[number]]--
	(*this).freq[number]++
	(*this).freq_cnt[(*this).freq[number]]++
}

func (this *FrequencyTracker) DeleteOne(number int) {
	if (*this).freq[number] == 0 {
		return
	}
	(*this).freq_cnt[(*this).freq[number]]--
	(*this).freq[number]--
	(*this).freq_cnt[(*this).freq[number]]++
}

func (this *FrequencyTracker) HasFrequency(frequency int) bool {
	return (*this).freq_cnt[frequency] > 0
}

func main() {
	{
	}
	{
	}
	{
	}
}
