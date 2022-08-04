pub struct SingleLinkNode<T> {
    pub next: Option<Box<SingleLinkNode<T>>>,
    pub content: T,
}
