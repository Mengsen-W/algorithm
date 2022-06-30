/*
 * @Date: 2022-06-30
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-06-30
 * @FilePath: /algorithm/1175_num_prime_arrangements/num_prime_arrangements.go
 */

package main

const mod int = 1e9 + 7

func numPrimeArrangements(n int) int {
	numPrimes := 0
	for i := 2; i <= n; i++ {
		if isPrime(i) {
			numPrimes++
		}
	}
	return factorial(numPrimes) * factorial(n-numPrimes) % mod
}

func isPrime(n int) bool {
	for i := 2; i*i <= n; i++ {
		if n%i == 0 {
			return false
		}
	}
	return true
}

func factorial(n int) int {
	f := 1
	for i := 1; i <= n; i++ {
		f = f * i % mod
	}
	return f
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(numPrimeArrangements(5) == 12)
	assert(numPrimeArrangements(100) == 682289015)
}
