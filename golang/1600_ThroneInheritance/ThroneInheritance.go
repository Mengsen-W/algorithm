/*
 * @Date: 2024-04-07
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-07
 * @FilePath: /algorithm/golang/1600_ThroneInheritance/ThroneInheritance.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

type ThroneInheritance struct {
	king  string
	edges map[string][]string
	dead  map[string]bool
}

func Constructor(kingName string) (t ThroneInheritance) {
	return ThroneInheritance{kingName, map[string][]string{}, map[string]bool{}}
}

func (t *ThroneInheritance) Birth(parentName, childName string) {
	t.edges[parentName] = append(t.edges[parentName], childName)
}

func (t *ThroneInheritance) Death(name string) {
	t.dead[name] = true
}

func (t *ThroneInheritance) GetInheritanceOrder() (ans []string) {
	var preorder func(string)
	preorder = func(name string) {
		if !t.dead[name] {
			ans = append(ans, name)
		}
		for _, childName := range t.edges[name] {
			preorder(childName)
		}
	}
	preorder(t.king)
	return
}

func main() {
	t := Constructor("king")     // 继承顺序：king
	t.Birth("king", "andy")      // 继承顺序：king > andy
	t.Birth("king", "bob")       // 继承顺序：king > andy > bob
	t.Birth("king", "catherine") // 继承顺序：king > andy > bob > catherine
	t.Birth("andy", "matthew")   // 继承顺序：king > andy > matthew > bob > catherine
	t.Birth("bob", "alex")       // 继承顺序：king > andy > matthew > bob > alex > catherine
	t.Birth("bob", "asha")       // 继承顺序：king > andy > matthew > bob > alex > asha > catherine
	assert.Equal(&testing.T{}, []string{"king", "andy", "matthew", "bob", "alex", "asha", "catherine"}, t.GetInheritanceOrder())
	t.Death("bob") // 继承顺序：king > andy > matthew > bob（已经去世）> alex > asha > catherine
	assert.Equal(&testing.T{}, []string{"king", "andy", "matthew", "alex", "asha", "catherine"}, t.GetInheritanceOrder())
}
