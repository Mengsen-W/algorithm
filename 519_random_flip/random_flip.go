/*
 * @Date: 2021-12-04 05:49:20
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-04 05:49:21
 */

package main

import (
	"math"
	"math/rand"
)

type Solution struct {
	m, n, total, bucketSize int
	buckets                 []map[int]bool
}

func Constructor(m, n int) Solution {
	total := m * n
	bucketSize := int(math.Sqrt(float64(total)))
	buckets := make([]map[int]bool, (total+bucketSize-1)/bucketSize)
	for i := range buckets {
		buckets[i] = map[int]bool{}
	}
	return Solution{m, n, total, bucketSize, buckets}
}

func (s *Solution) Flip() []int {
	x := rand.Intn(s.total)
	s.total--
	sumZero, curr := 0, 0
	for _, bucket := range s.buckets {
		if sumZero+s.bucketSize-len(bucket) > x {
			for i := 0; i < s.bucketSize; i++ {
				if !bucket[curr+i] {
					if sumZero == x {
						bucket[curr+i] = true
						return []int{(curr + i) / s.n, (curr + i) % s.n}
					}
					sumZero++
				}
			}
		}
		curr += s.bucketSize
		sumZero += s.bucketSize - len(bucket)
	}
	return nil
}

func (s *Solution) Reset() {
	s.total = s.m * s.n
	for i := range s.buckets {
		s.buckets[i] = map[int]bool{}
	}
}
