/*
 * @Date: 2021-03-14 09:07:07
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-03-14 09:09:48
 * @FilePath: \algorithm\706_my_hash_map\my_hash_map.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class MyHashMap {
 public:
  /** Initialize your data structure here. */
  MyHashMap() : hash_(vector<int>(1000001, -1)) {}

  /** value will always be non-negative. */
  void put(int key, int value) { hash_[key] = value; }

  /** Returns the value to which the specified key is mapped, or -1 if this map
   * contains no mapping for the key */
  int get(int key) { return hash_[key]; }

  /** Removes the mapping of the specified value key if this map contains a
   * mapping for the key */
  void remove(int key) { hash_[key] = -1; }

 private:
  vector<int> hash_;
};

int main() {
  MyHashMap* obj = new MyHashMap();
  obj->put(1, 1);
  assert(obj->get(1) == 1);
  obj->remove(1);
  assert(obj->get(1) == -1);
  return 0;
}