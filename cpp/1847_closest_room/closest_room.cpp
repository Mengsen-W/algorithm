#include <cassert>
#include <set>
#include <tuple>
#include <vector>

using namespace std;

struct Event {
  // 事件的类型，0 表示房间，1 表示询问
  int type;
  // 房间的 size 或者询问的 minSize
  int size;
  // 房间的 roomId 或者询问的 preferred
  int id;
  // 房间在数组 room 中的原始编号或者询问在数组 queries 中的原始编号
  int origin;

  Event(int _type, int _size, int _id, int _origin) : type(_type), size(_size), id(_id), origin(_origin) {}

  // 自定义比较函数，按照事件的 size 降序排序
  // 如果 size 相同，优先考虑房间
  bool operator<(const Event& that) const { return size > that.size || (size == that.size && type < that.type); }
};

class Solution {
 public:
  vector<int> closestRoom(vector<vector<int>>& rooms, vector<vector<int>>& queries) {
    int m = rooms.size();
    int n = queries.size();

    vector<Event> events;
    for (int i = 0; i < m; ++i) {
      // 房间事件
      events.emplace_back(0, rooms[i][1], rooms[i][0], i);
    }
    for (int i = 0; i < n; ++i) {
      // 询问事件
      events.emplace_back(1, queries[i][1], queries[i][0], i);
    }

    sort(events.begin(), events.end());
    vector<int> ans(n, -1);
    // 存储房间 roomId 的有序集合
    set<int> valid;
    for (const auto& event : events) {
      if (event.type == 0) {
        // 房间事件，将 roomId 加入有序集合
        valid.insert(event.id);
      } else {
        // 询问事件
        int dist = INT_MAX;
        // 查找最小的大于等于 preferred 的元素
        auto it = valid.lower_bound(event.id);
        if (it != valid.end() && *it - event.id < dist) {
          dist = *it - event.id;
          ans[event.origin] = *it;
        }
        if (it != valid.begin()) {
          // 查找最大的严格小于 preferred 的元素
          it = prev(it);
          if (event.id - *it <= dist) {
            dist = event.id - *it;
            ans[event.origin] = *it;
          }
        }
      }
    }

    return ans;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, vector<vector<int>>, vector<int>>> tests{
      {{{2, 2}, {1, 2}, {3, 2}}, {{3, 1}, {3, 3}, {5, 2}}, {3, -1, 3}},
      {{{1, 4}, {2, 3}, {3, 5}, {4, 1}, {5, 2}}, {{2, 3}, {2, 4}, {2, 5}}, {2, 1, 3}},
  };

  for (auto &[rooms, queries,ans] : tests) {
    assert(Solution().closestRoom(rooms, queries) == ans);
  }
}