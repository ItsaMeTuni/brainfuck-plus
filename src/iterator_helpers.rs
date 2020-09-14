use std::iter::{Peekable};

pub trait PeekableTrait: Iterator
{
    fn peek(&mut self) -> Option<&Self::Item>;
}

impl<T> PeekableTrait for Peekable<T>
    where T: Iterator,
{
    fn peek(&mut self) -> Option<&Self::Item>
    {
        Peekable::peek(self)
    }
}

pub trait TakeWhilePeekingImpl<Peekable>
    where Peekable: PeekableTrait
{
    fn take_while_peeking<P>(&mut self, predicate: P) -> TakeWhilePeeking<P, Peekable>
        where
            Self: Sized,
            P: FnMut(&Peekable::Item) -> bool;
}

impl<T> TakeWhilePeekingImpl<Peekable<T>> for Peekable<T>
    where T: Iterator
{
    fn take_while_peeking<P>(&mut self, predicate: P) -> TakeWhilePeeking<P, Peekable<T>>
        where
            Self: Sized,
            P: FnMut(&<Peekable<T> as Iterator>::Item) -> bool
    {
        TakeWhilePeeking::new(self, predicate)
    }
}

pub struct TakeWhilePeeking<'b, P, I>
    where I: PeekableTrait
{
    iter: &'b mut I,
    flag: bool,
    predicate: P,
}

impl<'b, P, I> TakeWhilePeeking<'b, P, I>
    where I: PeekableTrait
{
    pub fn new(iter: &'b mut I, predicate: P) -> TakeWhilePeeking<'b, P, I>
    {
        TakeWhilePeeking::<'b> {
            iter: iter,
            flag: false,
            predicate
        }
    }
}

impl<'b, P, I> Iterator for TakeWhilePeeking<'b, P, I>
    where
        I: PeekableTrait,
        P: FnMut(&I::Item) -> bool
{
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item>
    {
        if self.flag
        {
            None
        }
        else
        {
            let x = self.iter.peek()?;

            if (self.predicate)(x)
            {
                //Consume
                let x = self.iter.next()?;

                Some(x)
            }
            else
            {
                self.flag = true;
                None
            }
        }
    }
}