use std::{
    mem,
    rc::Rc,
    vec::Vec,
};
use crate::array::*;


#[test]
fn init_drop() {
    let a = <[_; 16]>::init_ext(|| Rc::new(()));
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
    let a = <[_; 16]>::init_ext(|| Rc::new(()));
    let b = a.clone();
    for x in a.iter() {
        assert_eq!(Rc::strong_count(x), 2);
    }

    let mut c = b.into_iter_ext().skip(8);
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
    let a = <[_; 16]>::init_ext(|| Rc::new(()));
    let b = a.clone();
    for x in a.iter() {
        assert_eq!(Rc::strong_count(x), 2);
    }

    let mut c = b.into_iter_ext();
    for x in &mut c {
        assert_eq!(Rc::strong_count(&x), 2);
    }

    mem::drop(c);
    for x in a.iter() {
        assert_eq!(Rc::strong_count(x), 1);
    }
}

#[test]
fn from_iter() {
    let v = (0..16).map(|i| Rc::new(i)).collect::<Vec<_>>();
    let a = <[_; 16]>::from_iter_ext(&mut v.iter().cloned()).unwrap();

    for (i, x) in v.iter().enumerate() {
        assert_eq!(Rc::strong_count(x), 2);
        assert_eq!(**x, i);
    }
    mem::drop(a);

    assert!(<[_; 16]>::from_iter_ext(&mut v.iter().cloned().take(8)).is_none());
    for x in v.iter() {
        assert_eq!(Rc::strong_count(x), 1);
    }
}

#[test]
fn for_each() {
    let a = <[_; 16]>::from_iter_ext(&mut (0..16).map(|i| Rc::new(i))).unwrap();
    let b = a.clone();

    let mut i = 0;
    b.for_each_ext(|x| {
        assert_eq!(Rc::strong_count(&x), 2);
        assert_eq!(*x, i);
        i += 1;
    });

    for x in a.iter() {
        assert_eq!(Rc::strong_count(x), 1);
    }
}

#[test]
fn map() {
    let a = <[usize; 16]>::from_iter_ext(&mut (0..16)).unwrap();

    for (i, x) in a.map_ext(|x| 15 - x).iter().enumerate() {
        assert_eq!(15 - i, *x);
    }
}

#[test]
fn zip() {
    let a = <[i32; 16]>::from_iter_ext(&mut (0..16)).unwrap();
    let b = <[i8; 16]>::from_iter_ext(&mut (-16..0)).unwrap();
    let c = a.clone().zip_ext(b.clone());

    for ((x, y), (a, b)) in c.into_iter_ext().zip(a.iter().zip(b.iter())) {
        assert_eq!(x, *a);
        assert_eq!(y, *b);
    }
}

#[test]
fn unzip() {
    let c = <[_; 16]>::from_iter_ext(&mut (0i32..16).zip(-16..0i8)).unwrap();
    let (a, b) = c.clone().unzip_ext();

    for ((x, y), (a, b)) in c.into_iter_ext().zip(a.iter().zip(b.iter())) {
        assert_eq!(x, *a);
        assert_eq!(y, *b);
    }
}
