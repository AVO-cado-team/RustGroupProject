#include <stdlib.h>
#include <stdio.h>

typedef struct double_linked_list_node
{
    int data;
    struct double_linked_list_node *next;
    struct double_linked_list_node *prev;

} DoubleLinkedListNode;

typedef struct double_linked_list
{
    int size;
    DoubleLinkedListNode *front;
    DoubleLinkedListNode *list;
    DoubleLinkedListNode *back;

} DoubleLinkedList;

typedef struct double_linked_list_result
{
    int data;
    int result;
} DoubleLinkedListResult;

int len(DoubleLinkedList *list);
void clear(DoubleLinkedList *list);
int is_empty(DoubleLinkedList *list);
void printDoubleLinkedList(DoubleLinkedList *head);

DoubleLinkedList *createDoubleLinkedList();

DoubleLinkedListNode *createDoubleLinkedListNode(int data);

DoubleLinkedListResult pop_back(DoubleLinkedList *list);
DoubleLinkedListResult pop_front(DoubleLinkedList *list);

DoubleLinkedListNode *find(DoubleLinkedList *list, int data);
DoubleLinkedListResult get(DoubleLinkedList *list, int index);
DoubleLinkedListResult _remove(DoubleLinkedList *list, int index);

void push_back(DoubleLinkedList *list, int data);
void push_front(DoubleLinkedList *list, int data);

int main()
{

    DoubleLinkedList *list = createDoubleLinkedList();

    push_back(list, 1);
    push_front(list, 2);

    printDoubleLinkedList(list);

    printf("%d\n", get(list, 1));

    printDoubleLinkedList(list);

    return 0;
}

DoubleLinkedList *createDoubleLinkedList()
{
    DoubleLinkedList *list = (DoubleLinkedList *)malloc(sizeof(DoubleLinkedList));
    list->front = list->list = list->back = NULL;
    list->size = 0;

    return list;
}

DoubleLinkedListNode *createDoubleLinkedListNode(int data)
{
    DoubleLinkedListNode *node = (DoubleLinkedListNode *)malloc(sizeof(DoubleLinkedListNode));

    node->data = data;
    node->next = node->prev = NULL;

    return node;
}

int len(DoubleLinkedList *list) { return list->size; }

void clear(DoubleLinkedList *list)
{
    DoubleLinkedListNode *node = list->front;

    while (node != NULL)
    {
        DoubleLinkedListNode *temp = node;
        node = node->next;
        free(temp);
    }

    list->front = list->list = list->back = NULL;
    list->size = 0;
}

int is_empty(DoubleLinkedList *list) { return list->size == 0; }

void push_back(DoubleLinkedList *list, int data)
{
    DoubleLinkedListNode *node = createDoubleLinkedListNode(data);

    (list->size)++;

    if (list->front == NULL)
        list->front = list->list = list->back = node;

    else
    {
        list->back->next = node;
        node->prev = list->back;
        list->back = node;
    }
}

void push_front(DoubleLinkedList *list, int data)
{
    DoubleLinkedListNode *node = createDoubleLinkedListNode(data);

    (list->size)++;

    if (list->front == NULL)
        list->front = list->list = list->back = node;

    else
    {
        list->front->prev = node;
        node->next = list->front;
        list->front = node;
    }
}

DoubleLinkedListNode *find(DoubleLinkedList *list, int data)
{
    DoubleLinkedListNode *node = list->front;

    while (node != NULL)
    {
        if (node->data == data)
            return node;

        node = node->next;
    }

    return NULL;
}

DoubleLinkedListResult get(DoubleLinkedList *list, int index)
{
    DoubleLinkedListResult result;
    DoubleLinkedListNode *node = list->front;

    if (index < 0 || index >= list->size)
    {
        result.data = result.result = 0;
        return result;
    }

    for (int i = 0; i < index; i++)
        node = node->next;

    if (node == NULL)
        result.data = result.result = 0;

    else
    {
        result.result = 1;
        result.data = node->data;
    }

    return result;
}

DoubleLinkedListResult _remove(DoubleLinkedList *list, int index)
{
    DoubleLinkedListResult result;
    DoubleLinkedListNode *node = list->front;

    (list->size)--;

    if (index < 0 || index >= list->size)
    {
        result.data = result.result = 0;
        return result;
    }

    else if (index == 0)
        return pop_front(list);

    else if (index == list->size - 1)
        return pop_back(list);

    else
    {
        for (int i = 0; i < index; i++)
            node = node->next;

        node->prev->next = node->next;
        node->next->prev = node->prev;
    }

    if (node == NULL)
        result.data = result.result = 0;

    else
    {
        result.result = 1;
        result.data = node->data;
    }

    return result;
}
DoubleLinkedListResult pop_back(DoubleLinkedList *list)
{
    DoubleLinkedListResult result;
    DoubleLinkedListNode *node = list->back;

    (list->size)--;

    if (list->front == NULL)
    {
        result.data = result.result = 0;
        return result;
    }

    else if (list->front == list->back)
        list->front = list->list = list->back = NULL;

    else
    {
        list->back = list->back->prev;
        list->back->next = NULL;
    }

    if (node == NULL)
        result.data = result.result = 0;

    else
    {
        result.result = 1;
        result.data = node->data;
    }

    return result;
}

DoubleLinkedListResult pop_front(DoubleLinkedList *list)
{
    DoubleLinkedListResult result;
    DoubleLinkedListNode *node = list->front;

    (list->size)--;

    if (list->front == NULL)
    {
        result.data = result.result = 0;
        return result;
    }

    else if (list->front == list->back)
        list->front = list->list = list->back = NULL;

    else
    {
        list->front = list->front->next;
        list->front->prev = NULL;
    }

    if (node == NULL)
        result.data = result.result = 0;

    else
    {
        result.result = 1;
        result.data = node->data;
    }

    return result;
}

void printDoubleLinkedList(DoubleLinkedList *list)
{
    DoubleLinkedListNode *node = list->front;

    while (node != NULL)
    {
        printf("%d ", node->data);
        node = node->next;
    }
    printf("\n");
}