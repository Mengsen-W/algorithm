/*
 * @Date: 2022-09-23
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-23
 * @FilePath: /algorithm/707_MyLinkedList/MyLinkedList.cpp
 */

#include <assert.h>

#define max(a, b) (a > b ? a : b)

struct DLinkListNode {
  int val;
  DLinkListNode *prev, *next;
  DLinkListNode(int _val) : val(_val), prev(nullptr), next(nullptr) {}
};

class MyLinkedList {
 public:
  MyLinkedList() {
    this->size = 0;
    this->head = new DLinkListNode(0);
    this->tail = new DLinkListNode(0);
    head->next = tail;
    tail->prev = head;
  }

  int get(int index) {
    if (index < 0 || index >= size) {
      return -1;
    }
    DLinkListNode *curr;
    if (index + 1 < size - index) {
      curr = head;
      for (int i = 0; i <= index; i++) {
        curr = curr->next;
      }
    } else {
      curr = tail;
      for (int i = 0; i < size - index; i++) {
        curr = curr->prev;
      }
    }
    return curr->val;
  }

  void addAtHead(int val) { addAtIndex(0, val); }

  void addAtTail(int val) { addAtIndex(size, val); }

  void addAtIndex(int index, int val) {
    if (index > size) {
      return;
    }
    index = max(0, index);
    DLinkListNode *pred, *succ;
    if (index < size - index) {
      pred = head;
      for (int i = 0; i < index; i++) {
        pred = pred->next;
      }
      succ = pred->next;
    } else {
      succ = tail;
      for (int i = 0; i < size - index; i++) {
        succ = succ->prev;
      }
      pred = succ->prev;
    }
    size++;
    DLinkListNode *toAdd = new DLinkListNode(val);
    toAdd->prev = pred;
    toAdd->next = succ;
    pred->next = toAdd;
    succ->prev = toAdd;
  }

  void deleteAtIndex(int index) {
    if (index < 0 || index >= size) {
      return;
    }
    DLinkListNode *pred, *succ;
    if (index < size - index) {
      pred = head;
      for (int i = 0; i < index; i++) {
        pred = pred->next;
      }
      succ = pred->next->next;
    } else {
      succ = tail;
      for (int i = 0; i < size - index - 1; i++) {
        succ = succ->prev;
      }
      pred = succ->prev->prev;
    }
    size--;
    DLinkListNode *p = pred->next;
    pred->next = succ;
    succ->prev = pred;
    delete p;
  }

 private:
  int size;
  DLinkListNode *head;
  DLinkListNode *tail;
};

int main() {
  MyLinkedList linkedList{};
  linkedList.addAtHead(1);
  linkedList.addAtTail(3);
  linkedList.addAtIndex(1, 2);     //链表变为1-> 2-> 3
  assert(linkedList.get(1) == 2);  //返回2
  linkedList.deleteAtIndex(1);     //现在链表是1-> 3
  assert(linkedList.get(1) == 3);  //返回3
}