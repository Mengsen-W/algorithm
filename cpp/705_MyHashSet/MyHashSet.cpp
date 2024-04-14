/*
 * @Date: 2024-04-14
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-14
 * @FilePath: /algorithm/cpp/705_MyHashSet/MyHashSet.cpp
 */

#include <cassert>
#include <list>
#include <vector>

using namespace std;

class MyHashSet {
 private:
  vector<list<int>> data;
  static const int base = 769;
  static int hash(int key) { return key % base; }

 public:
  /** Initialize your data structure here. */
  MyHashSet() : data(base) {}

  void add(int key) {
    int h = hash(key);
    for (auto it = data[h].begin(); it != data[h].end(); it++) {
      if ((*it) == key) {
        return;
      }
    }
    data[h].push_back(key);
  }

  void remove(int key) {
    int h = hash(key);
    for (auto it = data[h].begin(); it != data[h].end(); it++) {
      if ((*it) == key) {
        data[h].erase(it);
        return;
      }
    }
  }

  /** Returns true if this set contains the specified element */
  bool contains(int key) {
    int h = hash(key);
    for (auto it = data[h].begin(); it != data[h].end(); it++) {
      if ((*it) == key) {
        return true;
      }
    }
    return false;
  }
};

int main() {
  MyHashSet myHashSet = MyHashSet();
  myHashSet.add(1);                        // set = [1]
  myHashSet.add(2);                        // set = [1, 2]
  assert(myHashSet.contains(1) == true);   // 返回 True
  assert(myHashSet.contains(3) == false);  // 返回 False ，（未找到）
  myHashSet.add(2);                        // set = [1, 2]
  assert(myHashSet.contains(2) == true);   // 返回 True
  myHashSet.remove(2);                     // set = [1]
  assert(myHashSet.contains(2) == false);  // 返回 False ，（已移除）
}