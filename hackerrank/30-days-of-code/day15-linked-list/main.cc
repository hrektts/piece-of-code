#include <iostream>
#include <cstddef>

using namespace std;

class Node {
 public:
  int data;
  Node *next;
  explicit Node(int d) {
    data = d;
    next = NULL;
  }
};

class Solution {
 public:
  Node* insert(Node *head, int data) {
    if (!head) {
      return new Node(data);
    }

    Node *p = head;
    while (p->next) {
      p = p->next;
    }
    p->next = new Node(data);
    return head;
  }

  void display(Node *head) {
    Node *start = head;
    while (start) {
      cout << start->data << " ";
      start = start->next;
    }
  }
};

int main() {
  Node* head = NULL;
  Solution mylist;
  int T, data;
  cin >> T;
  while (T-- > 0) {
    cin >> data;
    head = mylist.insert(head, data);
  }
  mylist.display(head);
}
