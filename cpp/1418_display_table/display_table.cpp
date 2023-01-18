/*
 * @Date: 2021-07-06 08:44:17
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-06 20:31:52
 */

#include <cassert>
#include <list>
#include <map>
#include <set>
#include <string>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<vector<string>> displayTable(vector<vector<string>>& orders) {
    int n = orders.size();
    int m = 3;
    vector<vector<string>> ans;

    //建立标题行
    set<string> title_set;      //利用set对食物进行字典序排序
    map<int, string> table_mp;  //利用map对行号进行排序
    int table_row = 1;
    for (int i = 0; i < n; ++i) {
      string food = orders[i][2];
      string table = orders[i][1];
      int table_num = stoi(table);

      if (table_mp.find(table_num) == table_mp.end())  //去重
      {
        table_mp[table_num] = table;  //桌号
      }

      title_set.insert(food);
    }
    //为了在头部插入Table字符串
    list<string> title_list(title_set.begin(), title_set.end());
    title_list.push_front("Table");
    ans.push_back(vector<string>(title_list.begin(), title_list.end()));

    //建立食物对应的列的哈希表
    unordered_map<string, int> food_idx_mp;
    food_idx_mp["Table"] = 0;
    for (int i = 1; i < ans[0].size(); ++i) {
      food_idx_mp[ans[0][i]] = i;
    }

    // 1、建立每个桌号对应的行
    // 2、建立桌号对应表里的行号的哈希表
    int col = ans[0].size();  //获取食物总数+桌号的长度，用于建表
    unordered_map<string, int> table_idx_mp;
    for (auto it = table_mp.begin(); it != table_mp.end(); ++it) {
      auto& twins = *it;
      ans.push_back(vector<string>(col, "0"));
      ans.back()[0] = twins.second;
      table_idx_mp[twins.second] = ans.size() - 1;
    }

    //通过遍历，把对应桌号行号、食物列号的数量+1
    for (int i = 0; i < n; ++i) {
      string food = orders[i][2];
      string table = orders[i][1];
      int table_idx = table_idx_mp[table];
      int food_idx = food_idx_mp[food];

      int num = stoi(ans[table_idx][food_idx]);
      ++num;
      ans[table_idx][food_idx] = to_string(num);
    }

    return ans;
  }
};

int main() {
  {
    vector<vector<string>> orders{
        {"David", "3", "Ceviche"},       {"Corina", "10", "Beef Burrito"},
        {"David", "3", "Fried Chicken"}, {"Carla", "5", "Water"},
        {"Carla", "5", "Ceviche"},       {"Rous", "3", "Ceviche"}};
    vector<vector<string>> ans{
        {"Table", "Beef Burrito", "Ceviche", "Fried Chicken", "Water"},
        {"3", "0", "2", "1", "0"},
        {"5", "0", "1", "0", "1"},
        {"10", "1", "0", "0", "0"}};
    assert(Solution{}.displayTable(orders) == ans);
  }
  {
    vector<vector<string>> orders{{"James", "12", "Fried Chicken"},
                                  {"Ratesh", "12", "Fried Chicken"},
                                  {"Amadeus", "12", "Fried Chicken"},
                                  {"Adam", "1", "Canadian Waffles"},
                                  {"Brianna", "1", "Canadian Waffles"}};

    vector<vector<string>> ans{{"Table", "Canadian Waffles", "Fried Chicken"},
                               {"1", "2", "0"},
                               {"12", "0", "3"}};
    assert(Solution{}.displayTable(orders) == ans);
  }
  {
    vector<vector<string>> orders{{"Laura", "2", "Bean Burrito"},
                                  {"Jhon", "2", "Beef Burrito"},
                                  {"Melissa", "2", "Soda"}};

    vector<vector<string>> ans{
        {"Table", "Bean Burrito", "Beef Burrito", "Soda"},
        {"2", "1", "1", "1"}};
    assert(Solution{}.displayTable(orders) == ans);
  }
  return 0;
}