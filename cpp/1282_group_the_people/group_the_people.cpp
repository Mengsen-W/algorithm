/*
 * @Date: 2022-08-12
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-12
 * @FilePath: /algorithm/1282_group_the_people/group_the_people.cpp
 */

#include <cassert>
#include <iostream>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<vector<int>> groupThePeople(vector<int> groupSizes) {
    unordered_map<int, vector<int>> groups;
    int n = groupSizes.size();
    for (int i = 0; i < n; i++) {
      int size = groupSizes[i];
      groups[size].emplace_back(i);
    }
    vector<vector<int>> groupList;
    for (auto& [size, people] : groups) {
      int groupCount = people.size() / size;
      for (int i = 0; i < groupCount; i++) {
        vector<int> group;
        int start = i * size;
        for (int j = 0; j < size; j++) {
          group.emplace_back(people[start + j]);
        }
        groupList.emplace_back(group);
      }
    }
    return groupList;
  }
};

int main() {
  assert(
      (Solution().groupThePeople(vector<int>{3, 3, 3, 3, 3, 1, 3}) == vector<vector<int>>{{5}, {0, 1, 2}, {3, 4, 6}}));
  assert((Solution().groupThePeople(vector<int>{2, 1, 3, 3, 3, 2}) == vector<vector<int>>{{2, 3, 4}, {1}, {0, 5}}));
}