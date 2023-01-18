/*
 * @Date: 2022-03-14 00:48:28
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-14 01:16:42
 * @FilePath: /algorithm/599_find_restaurant/find_restaurant.cpp
 */

#include <cassert>
#include <climits>
#include <string>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<string> findRestaurant(vector<string>& list1, vector<string>& list2) {
    unordered_map<string, int> index;
    for (int i = 0; i < list1.size(); i++) {
      index[list1[i]] = i;
    }

    vector<string> ret;
    int indexSum = INT_MAX;
    for (int i = 0; i < list2.size(); i++) {
      if (index.count(list2[i]) > 0) {
        int j = index[list2[i]];
        if (i + j < indexSum) {
          ret.clear();
          ret.push_back(list2[i]);
          indexSum = i + j;
        } else if (i + j == indexSum) {
          ret.push_back(list2[i]);
        }
      }
    }
    return ret;
  }
};

int main() {
  {
    vector<string> list1{"Shogun", "Tapioca Express", "Burger King", "KFC"};
    vector<string> list2{"Piatti", "The Grill at Torrey Pines",
                         "Hungry Hunter Steakhouse", "Shogun"};
    vector<string> ans{"Shogun"};
    assert(Solution().findRestaurant(list1, list2) == ans);
  }
  {
    vector<string> list1{"Shogun", "Tapioca Express", "Burger King", "KFC"};
    vector<string> list2{"KFC", "Shogun", "Burger King"};
    vector<string> ans{"Shogun"};
    assert(Solution().findRestaurant(list1, list2) == ans);
  }
}