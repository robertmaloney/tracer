pub struct TwoDimensionalIterator<U, V> where
    U: Iterator + Clone,
    V: Iterator,
    <V as Iterator>::Item: Clone,
{
    x_iter: U,
    y_iter: V,
    x_copy: U,
    y_copy: Option<<V as Iterator>::Item>,
}

impl<U, V> TwoDimensionalIterator<U, V> where
    U: Iterator + Clone,
    V: Iterator,
    <V as Iterator>::Item: Clone,
{
    pub fn new(a: U, b: V) -> TwoDimensionalIterator<U, V> {
        let mut b = b;
        let y0 = b.next();
        TwoDimensionalIterator {
            x_copy: a.clone(),
            x_iter: a,
            y_copy: y0,
            y_iter: b,
        }
    }
}

impl<U, V> Iterator for TwoDimensionalIterator<U, V> where
    U: Iterator + Clone,
    V: Iterator,
    <V as Iterator>::Item: Clone,
{
    type Item = (<U as Iterator>::Item, <V as Iterator>::Item);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y_copy.is_none() {
            return None;
        }

        match self.x_iter.next() {
            Some(x) => {
                let y = self.y_copy.clone().unwrap();
                Some((x, y))
            },
            None => {
                self.x_iter = self.x_copy.clone();
                self.y_copy = self.y_iter.next();
                self.next()
            }
        }
    }
}
