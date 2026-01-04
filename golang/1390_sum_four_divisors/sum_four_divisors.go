// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func sumFourDivisors(nums []int) int {
	// C 是数组 nums 元素的上限，C3 是 C 的立方根
	C, C3 := 100000, 46

	isprime := make([]int, C+1)
	for i := 2; i <= C; i++ {
		isprime[i] = 1
	}
	primes := []int{}

	// 埃拉托斯特尼筛法
	for i := 2; i <= C; i++ {
		if isprime[i] == 1 {
			primes = append(primes, i)
		}
		for j := i + i; j <= C; j += i {
			isprime[j] = 0
		}
	}

	// 欧拉筛法
	/*
	   for i := 2; i <= C; i++ {
	       if isprime[i] == 1 {
	           primes = append(primes, i)
	       }
	       for _, prime := range primes {
	           if i * prime > C {
	               break
	           }
	           isprime[i * prime] = 0
	           if i % prime == 0 {
	               break
	           }
	       }
	   }
	*/

	// 通过质数表构造出所有的四因数
	factor4 := make(map[int]int)
	for _, prime := range primes {
		if prime <= C3 {
			factor4[prime*prime*prime] = 1 + prime + prime*prime + prime*prime*prime
		}
	}
	for i := 0; i < len(primes); i++ {
		for j := i + 1; j < len(primes); j++ {
			if primes[i] <= C/primes[j] {
				factor4[primes[i]*primes[j]] = 1 + primes[i] + primes[j] + primes[i]*primes[j]
			} else {
				break
			}
		}
	}

	ans := 0
	for _, num := range nums {
		if val, exists := factor4[num]; exists {
			ans += val
		}
	}
	return ans
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{21, 4, 7}, 32},
		{[]int{21, 21}, 64},
		{[]int{1, 2, 3, 4, 5}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, sumFourDivisors(test.nums), index)
	}
}
