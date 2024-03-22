/*
 * @Date: 2024-03-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-22
 * @FilePath: /algorithm/golang/2671_FrequencyTracker/FrequencyTracker.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

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
		frequencyTracker := Constructor()
		frequencyTracker.Add(3)
		frequencyTracker.Add(3)
		assert.Equal(&testing.T{}, true, frequencyTracker.HasFrequency(2))
	}

	{
		frequencyTracker := Constructor()
		frequencyTracker.Add(1)
		frequencyTracker.DeleteOne(1)
		assert.Equal(&testing.T{}, false, frequencyTracker.HasFrequency(1))
	}

	{
		frequencyTracker := Constructor()
		assert.Equal(&testing.T{}, false, frequencyTracker.HasFrequency(1))
		frequencyTracker.Add(3)
		assert.Equal(&testing.T{}, true, frequencyTracker.HasFrequency(1))
	}
}
