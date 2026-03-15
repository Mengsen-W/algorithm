// Package main ...
package main

const MOD = 1000000007

type Fancy struct {
	v []int
	a int
	b int
}

func Constructor() Fancy {
	return Fancy{
		v: []int{},
		a: 1,
		b: 0,
	}
}

// 快速幂
func (this *Fancy) quickMul(x, y int) int {
	ret := 1
	cur := x
	for y > 0 {
		if y&1 != 0 {
			ret = (ret * cur) % MOD
		}
		cur = (cur * cur) % MOD
		y >>= 1
	}
	return ret
}

// 乘法逆元
func (this *Fancy) inv(x int) int {
	return this.quickMul(x, MOD-2)
}

func (this *Fancy) Append(val int) {
	adjustedVal := ((val - this.b + MOD) % MOD) * this.inv(this.a) % MOD
	this.v = append(this.v, adjustedVal)
}

func (this *Fancy) AddAll(inc int) {
	this.b = (this.b + inc) % MOD
}

func (this *Fancy) MultAll(m int) {
	this.a = (this.a * m) % MOD
	this.b = (this.b * m) % MOD
}

func (this *Fancy) GetIndex(idx int) int {
	if idx >= len(this.v) {
		return -1
	}
	ans := (this.a*this.v[idx]%MOD + this.b) % MOD
	return ans
}

func main() {
	assert := func(x bool) {
		if !x {
			panic("Not Pass")
		}
	}
	fancy := Constructor()
	fancy.Append(2)                 // 奇妙序列：[2]
	fancy.AddAll(3)                 // 奇妙序列：[2+3] -> [5]
	fancy.Append(7)                 // 奇妙序列：[5, 7]
	fancy.MultAll(2)                // 奇妙序列：[5*2, 7*2] -> [10, 14]
	assert(fancy.GetIndex(0) == 10) // 返回 10
	fancy.AddAll(3)                 // 奇妙序列：[10+3, 14+3] -> [13, 17]
	fancy.Append(10)                // 奇妙序列：[13, 17, 10]
	fancy.MultAll(2)                // 奇妙序列：[13*2, 17*2, 10*2] -> [26, 34, 20]
	assert(fancy.GetIndex(0) == 26) // 返回 26
	assert(fancy.GetIndex(1) == 34) // 返回 34
	assert(fancy.GetIndex(2) == 20) // 返回 20
}
