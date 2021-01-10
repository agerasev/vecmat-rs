use crate::vector::*;

#[cfg(feature = "std")]
mod mem {
    use super::*;
    use std::{mem, rc::Rc, vec::Vec};

    #[test]
    fn init_drop() {
        let a = <Vector16<_>>::init(|| Rc::new(()));
        let b = a.clone();
        for x in a.iter() {
            assert_eq!(Rc::strong_count(x), 2);
        }

        mem::drop(b);
        for x in a.iter() {
            assert_eq!(Rc::strong_count(x), 1);
        }
    }

    #[test]
    fn into_iter() {
        let a = <Vector16<_>>::init(|| Rc::new(()));
        let b = a.clone();
        for x in a.iter() {
            assert_eq!(Rc::strong_count(x), 2);
        }

        let mut c = b.into_iter().skip(8);
        c.next().unwrap();

        for (i, x) in a.iter().enumerate() {
            if i < 9 {
                assert_eq!(Rc::strong_count(x), 1);
            } else {
                assert_eq!(Rc::strong_count(x), 2);
            }
        }

        mem::drop(c);
        for x in a.iter() {
            assert_eq!(Rc::strong_count(x), 1);
        }
    }

    #[test]
    fn iter_loop() {
        let a = <Vector16<_>>::init(|| Rc::new(()));
        let b = a.clone();
        for x in a.iter() {
            assert_eq!(Rc::strong_count(x), 2);
        }

        let mut c = b.into_iter();
        for x in &mut c {
            assert_eq!(Rc::strong_count(&x), 2);
        }

        mem::drop(c);
        for x in a.iter() {
            assert_eq!(Rc::strong_count(x), 1);
        }
    }

    #[test]
    fn try_from_iter() {
        let v = (0..16).map(Rc::new).collect::<Vec<_>>();
        let a = <Vector16<_>>::try_from_iter(&mut v.iter().cloned()).unwrap();

        for (i, x) in v.iter().enumerate() {
            assert_eq!(Rc::strong_count(x), 2);
            assert_eq!(**x, i);
        }
        mem::drop(a);

        assert!(<Vector16<_>>::try_from_iter(&mut v.iter().cloned().take(8)).is_none());
        for x in v.iter() {
            assert_eq!(Rc::strong_count(x), 1);
        }
    }

    #[test]
    fn for_each() {
        let a = <Vector16<_>>::try_from_iter(&mut (0..16).map(Rc::new)).unwrap();
        let b = a.clone();

        let mut i = 0;
        b.for_each(|x| {
            assert_eq!(Rc::strong_count(&x), 2);
            assert_eq!(*x, i);
            i += 1;
        });

        for x in a.iter() {
            assert_eq!(Rc::strong_count(x), 1);
        }
    }

    #[test]
    fn into_iter_rev() {
        let a = <Vector16<_>>::init(|| Rc::new(()));
        let b = a.clone();
        for x in a.iter() {
            assert_eq!(Rc::strong_count(x), 2);
        }

        let mut c = b.into_iter().rev().skip(7);
        c.next().unwrap();

        for (i, x) in a.iter().enumerate() {
            if i >= 8 {
                assert_eq!(Rc::strong_count(x), 1);
            } else {
                assert_eq!(Rc::strong_count(x), 2);
            }
        }

        mem::drop(c);
        for x in a.iter() {
            assert_eq!(Rc::strong_count(x), 1);
        }
    }

    #[test]
    fn into_iter_both() {
        let a = <Vector16<_>>::init(|| Rc::new(()));
        let b = a.clone();
        for x in a.iter() {
            assert_eq!(Rc::strong_count(x), 2);
        }

        let mut c = b.into_iter().skip(3);
        c.next().unwrap();
        let mut c = c.rev().skip(3);
        c.next().unwrap();

        for (i, x) in a.iter().enumerate() {
            if !(4..12).contains(&i) {
                assert_eq!(Rc::strong_count(x), 1);
            } else {
                assert_eq!(Rc::strong_count(x), 2);
            }
        }

        mem::drop(c);
        for x in a.iter() {
            assert_eq!(Rc::strong_count(x), 1);
        }
    }

    #[test]
    fn into_iter_len() {
        let mut a = Vector16::indices().into_iter();
        
        for i in 0..8 {
            assert_eq!(a.len(), 16 - i);
            assert_eq!(a.next().unwrap(), i);
        }
        for i in 0..8 {
            assert_eq!(a.len(), 8 - i);
            assert_eq!(a.next_back().unwrap(), 16 - i - 1);
        }
        assert_eq!(a.len(), 0);
        assert!(a.next().is_none());
    }
}

mod iter {
    use super::*;

    #[test]
    fn map() {
        let a = <Vector16<usize>>::try_from_iter(&mut (0..16)).unwrap();

        for (i, x) in a.map(|x| 15 - x).iter().enumerate() {
            assert_eq!(15 - i, *x);
        }
    }

    #[test]
    fn zip() {
        let a = <Vector16<i32>>::try_from_iter(&mut (0..16)).unwrap();
        let b = <Vector16<i8>>::try_from_iter(&mut (-16..0)).unwrap();
        let c = a.zip(b);

        for ((x, y), (a, b)) in c.into_iter().zip(a.iter().zip(b.iter())) {
            assert_eq!(x, *a);
            assert_eq!(y, *b);
        }
    }

    #[test]
    fn unzip() {
        let c = <Vector16<_>>::try_from_iter(&mut (0i32..16).zip(-16..0i8)).unwrap();
        let (a, b) = c.clone().unzip();

        for ((x, y), (a, b)) in c.into_iter().zip(a.iter().zip(b.iter())) {
            assert_eq!(x, *a);
            assert_eq!(y, *b);
        }
    }
}
