/*
 * @Date: 2022-04-13 09:17:12
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-13 09:36:54
 * @FilePath: /algorithm/380_randomized_set/randomized_set.cpp
 */

#include <cassert>
#include <unordered_map>
#include <vector>
#include <iostream>

using namespace std;

class RandomizedSet {
 public:
  RandomizedSet() { srand((unsigned)time(NULL)); }

  bool insert(int val) {
    if (indices.count(val)) {
      return false;
    }
    int index = nums.size();
    nums.emplace_back(val);
    indices[val] = index;
    return true;
  }

  bool remove(int val) {
    if (!indices.count(val)) {
      return false;
    }
    int index = indices[val];
    int last = nums.back();
    nums[index] = last;
    indices[last] = index;
    nums.pop_back();
    indices.erase(val);
    return true;
  }

  int getRandom() {
    int randomIndex = rand() % nums.size();
    return nums[randomIndex];
  }

 private:
  vector<int> nums;
  unordered_map<int, int> indices;
};

int main() {
  RandomizedSet randomSet;
  assert(randomSet.insert(1));
  assert(!randomSet.remove(2));
  assert(randomSet.insert(2));
  assert(randomSet.getRandom() == 1 || randomSet.getRandom() == 2);
  assert(randomSet.remove(1));
  assert(!randomSet.insert(2));
  assert(randomSet.getRandom() == 2);
  return 0;
}