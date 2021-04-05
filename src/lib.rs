pub trait IteratorExt: Iterator {
    fn flatten2(self) -> Flatten<Self>
    where
        Self: Sized,
        Self::Item: IntoIterator;
}

impl<T> IteratorExt for T
where
    Self: Sized,
    T: Iterator,
{
    fn flatten2(self) -> Flatten<Self>
    where
        Self::Item: IntoIterator,
    {
        flatten(self)
    }
}

pub fn flatten<I>(iter: I) -> Flatten<I::IntoIter>
where
    I: IntoIterator,
    I::Item: IntoIterator,
{
    Flatten::new(iter.into_iter())
}

pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    f_inner: Option<<O::Item as IntoIterator>::IntoIter>,
    b_inner: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    fn new(iter: O) -> Self {
        Flatten {
            outer: iter,
            f_inner: None,
            b_inner: None,
        }
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner_iter) = self.f_inner {
                if let Some(i) = inner_iter.next() {
                    return Some(i);
                }
                self.f_inner = None;
            }
            if let Some(next_inner) = self.outer.next() {
                self.f_inner = Some(next_inner.into_iter());
            } else {
                return self.b_inner.as_mut()?.next();
            }
        }
    }
}

impl<O> DoubleEndedIterator for Flatten<O>
where
    O: DoubleEndedIterator,
    O::Item: IntoIterator,
    <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner_iter) = self.b_inner {
                if let Some(i) = inner_iter.next_back() {
                    return Some(i);
                }
                self.b_inner = None;
            }
            if let Some(next_inner) = self.outer.next_back() {
                self.b_inner = Some(next_inner.into_iter());
            } else {
                return self.f_inner.as_mut()?.next_back();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0);
    }

    #[test]
    fn empty_wide() {
        assert_eq!(
            flatten(vec![Vec::<()>::new(), vec![], vec![], vec![]]).count(),
            0
        );
    }

    #[test]
    fn one() {
        assert_eq!(flatten(std::iter::once(vec!["A"])).count(), 1);
    }

    #[test]
    fn two() {
        assert_eq!(flatten(std::iter::once(vec!["A", "B"])).count(), 2);
    }

    #[test]
    fn two_wide() {
        assert_eq!(flatten(vec![vec!["a"], vec!["b"]]).count(), 2);
    }

    #[test]
    fn reverse_two() {
        assert_eq!(
            flatten(std::iter::once(vec!["a", "b"]))
                .rev()
                .collect::<Vec<_>>(),
            vec!["b", "a"]
        );
    }

    #[test]
    fn reverse_two_wide() {
        assert_eq!(
            flatten(vec![vec!["a"], vec!["b"]])
                .rev()
                .collect::<Vec<_>>(),
            vec!["b", "a"]
        );
    }

    #[test]
    fn both_ends() {
        let mut iter = flatten(vec![vec!["a1", "a2", "a3"], vec!["b1", "b2", "b3"]]);
        assert_eq!(iter.next(), Some("a1"));
        assert_eq!(iter.next_back(), Some("b3"));

        assert_eq!(iter.next(), Some("a2"));
        assert_eq!(iter.next_back(), Some("b2"));

        assert_eq!(iter.next(), Some("a3"));
        assert_eq!(iter.next(), Some("b1"));
        assert_eq!(iter.next_back(), None);

        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn inf() {
        let mut iter = flatten((0..).map(|i| 0..i));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
    }

    #[test]
    fn deep() {
        assert_eq!(flatten(flatten(vec![vec![vec![0, 1]]])).count(), 2);
    }

    #[test]
    fn ext() {
        assert_eq!(vec![vec![0, 1]].into_iter().flatten2().count(), 2);
    }
}
