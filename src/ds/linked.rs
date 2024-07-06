pub struct Link<T> {
    val: T,
    next: usize,
}

pub struct DoubleLink<T> {
    val: T,
    prev: usize,
    next: usize,
}

pub struct LinkedList<T> {
    head: usize,
    tail: usize,
    list: Vec<Link<T>>,
}

pub struct DoublyLinkedList<T> {
    head: usize,
    tail: usize,
    list: Vec<DoubleLink<T>>,
}
