#include <cassert>
#include <queue>
#include <unordered_map>
#include <vector>

using namespace std;

class TaskManager {
 private:
  unordered_map<int, pair<int, int>> taskInfo;
  priority_queue<pair<int, int>> heap;

 public:
  TaskManager(vector<vector<int>> tasks) {
    for (auto& task : tasks) {
      int userId = task[0], taskId = task[1], priority = task[2];
      taskInfo[taskId] = {priority, userId};
      heap.emplace(priority, taskId);
    }
  }

  void add(int userId, int taskId, int priority) {
    taskInfo[taskId] = {priority, userId};
    heap.emplace(priority, taskId);
  }

  void edit(int taskId, int newPriority) {
    if (taskInfo.find(taskId) != taskInfo.end()) {
      taskInfo[taskId].first = newPriority;
      heap.emplace(newPriority, taskId);
    }
  }

  void rmv(int taskId) { taskInfo.erase(taskId); }

  int execTop() {
    while (!heap.empty()) {
      auto [priority, taskId] = heap.top();
      heap.pop();

      if (taskInfo.find(taskId) != taskInfo.end() && taskInfo[taskId].first == priority) {
        int userId = taskInfo[taskId].second;
        taskInfo.erase(taskId);
        return userId;
      }
    }
    return -1;
  }
};

int main() {
  TaskManager taskManager = TaskManager({{1, 101, 10}, {2, 102, 20}, {3, 103, 15}});
  taskManager.add(4, 104, 5);
  taskManager.edit(102, 8);
  assert(taskManager.execTop() == 3);
  taskManager.rmv(101);
  taskManager.add(5, 105, 15);
  assert(taskManager.execTop() == 5);
}
