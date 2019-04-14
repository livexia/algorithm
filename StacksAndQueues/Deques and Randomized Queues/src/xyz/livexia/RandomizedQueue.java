//package xyz.livexia;

import java.util.Arrays;
import java.util.Iterator;
import java.lang.IllegalArgumentException;
import java.util.NoSuchElementException;
import java.lang.UnsupportedOperationException;

import edu.princeton.cs.algs4.StdOut;
import edu.princeton.cs.algs4.StdRandom;


public class RandomizedQueue<Item> implements Iterable<Item> {
    private Item[] q;
    private int size = 0;
    private int first = 0;
    private int last = 0;

    public RandomizedQueue() {
        q = (Item[]) new Object[10];
        first = -1;
        last = first;
    }

    public boolean isEmpty() {
        return size == 0;
    }

    public int size() {
        return size;
    }

    private void resize(int capacity) {
        Item[] copy = (Item[]) new Object[capacity];
        int i = first;
        int j = 0;
        while (i < last + 1) {
            if (q[i] != null) {
                copy[j] = q[i];
                j++;
            }
            i++;
        }
        last = j - 1;
        first = 0;
        q = copy;
    }

    public void enqueue(Item item) {
        if (item == null) {
            throw new IllegalArgumentException("Add null");
        }
        q[++last] = item;
        size++;
        if (last == q.length - 1) resize(2 * q.length);
        if (size == 1) first = last;
    }

    public Item dequeue() {
        if (isEmpty()) {
            throw new NoSuchElementException("Try to remove form a empty queue");
        }
        int N = StdRandom.uniform(first, last + 1);
        Item item = q[N];
        q[N] = null;
        if (item == null) {
            return dequeue();
        } else {
            if (N == last) last--;
            else if (N == first) first++;
            if (size == q.length / 4) resize(q.length / 2);
            size--;
            return item;
        }
    }

    public Item sample() {
        if (isEmpty()) {
            throw new NoSuchElementException("Try to get from a empty queue");
        }
        int N = StdRandom.uniform(first, last + 1);
        Item item = q[N];
        if (item == null) {
            return sample();
        }else {
            return q[N];
        }
    }

    @Override
    public Iterator<Item> iterator() {
        return new RandomizedDequeIterator();
    }

    private class RandomizedDequeIterator implements Iterator<Item> {
        private int i = 0;
        private Item[] items;

        public RandomizedDequeIterator() {
            if(!isEmpty()) {
                resize(q.length);
                items = Arrays.copyOfRange(q, first, last + 1);
                StdRandom.shuffle(items);
            }
        }

        public boolean hasNext() {
            return i < size;
        }

        public void remove() {
            throw new UnsupportedOperationException("Unsupported method remove");
        }

        public Item next() {
            if (!hasNext()) {
                throw new NoSuchElementException();
            }
            return items[i++];
        }
    }

    public static void main(String[] args) {
        RandomizedQueue<Integer> rQ = new RandomizedQueue<Integer>();
        if (rQ.isEmpty()) StdOut.println("Empty");

        rQ.enqueue(0);
        rQ.enqueue(1);
        rQ.enqueue(2);
        rQ.enqueue(3);
        rQ.enqueue(4);
        rQ.enqueue(5);
        rQ.enqueue(6);
        rQ.enqueue(7);
        rQ.enqueue(8);
        rQ.enqueue(9);

//        for (int i : rQ) StdOut.println(i);

        StdOut.println("*************************");

        for (int i = 0; i < 8; i++) StdOut.println(rQ.dequeue());

        StdOut.println("*************************");

        for (int i : rQ) StdOut.println(i);
    }
}