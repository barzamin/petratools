pub struct BspNode<T> {
    values: Vec<T>,
    front: Option<Box<BspNode<T>>>,
    back: Option<Box<BspNode<T>>>,
}
