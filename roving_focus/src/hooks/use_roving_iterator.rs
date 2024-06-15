use crate::utils::Dir;
use std::{cell::RefCell, rc::Rc};
use yew::prelude::*;

pub trait RovingIterator {
    fn next(&mut self, dir: &Dir) -> Option<u32>;
    fn prev(&mut self, dir: &Dir) -> Option<u32>;
    fn first(&mut self, dir: &Dir) -> Option<u32>;
    fn last(&mut self, dir: &Dir) -> Option<u32>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct IndexRoving {
    pub current: u32,
    pub length: u32,
    pub r#loop: bool,
}

impl RovingIterator for IndexRoving {
    fn next(&mut self, dir: &Dir) -> Option<u32> {
        match dir {
            Dir::Ltr => {
                if self.current < self.length - 1 {
                    self.current += 1;
                    Some(self.current)
                } else if self.r#loop {
                    self.current = 0;
                    Some(self.current)
                } else {
                    return None;
                }
            }
            Dir::Rtl => self.prev(&Dir::Ltr),
        }
    }

    fn prev(&mut self, dir: &Dir) -> Option<u32> {
        match dir {
            Dir::Ltr => {
                if self.current > 0 {
                    self.current -= 1;
                    Some(self.current)
                } else if self.r#loop {
                    self.current = self.length - 1;
                    Some(self.current)
                } else {
                    return None;
                }
            }
            Dir::Rtl => self.next(&Dir::Ltr),
        }
    }

    fn first(&mut self, dir: &Dir) -> Option<u32> {
        match dir {
            Dir::Ltr => {
                self.current = 0;
                Some(self.current)
            }
            Dir::Rtl => self.last(&Dir::Ltr),
        }
    }

    fn last(&mut self, dir: &Dir) -> Option<u32> {
        match dir {
            Dir::Ltr => {
                self.current = self.length - 1;
                Some(self.current)
            }
            Dir::Rtl => self.first(&Dir::Ltr),
        }
    }
}

#[hook]
pub fn use_roving_iterator(length: u32, r#loop: bool, dir: &Dir) -> Rc<RefCell<IndexRoving>> {
    use_mut_ref(|| IndexRoving {
        current: if *dir == Dir::Ltr { 0 } else { length - 1 },
        length,
        r#loop,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_roving_next_looped_ltr() {
        let mut roving = IndexRoving {
            current: 0,
            length: 3,
            r#loop: true,
        };

        assert_eq!(roving.next(&Dir::Ltr), Some(1));
        assert_eq!(roving.next(&Dir::Ltr), Some(2));
        assert_eq!(roving.next(&Dir::Ltr), Some(0));
        assert_eq!(roving.next(&Dir::Ltr), Some(1));
    }

    #[test]
    fn test_index_roving_prev_looped_ltr() {
        let mut roving = IndexRoving {
            current: 2,
            length: 3,
            r#loop: true,
        };

        assert_eq!(roving.prev(&Dir::Ltr), Some(1));
        assert_eq!(roving.prev(&Dir::Ltr), Some(0));
        assert_eq!(roving.prev(&Dir::Ltr), Some(2));
        assert_eq!(roving.prev(&Dir::Ltr), Some(1));
    }

    #[test]
    fn test_index_roving_next_non_looped_ltr() {
        let mut roving = IndexRoving {
            current: 2,
            length: 3,
            r#loop: false,
        };

        assert_eq!(roving.next(&Dir::Ltr), None);
    }

    #[test]
    fn test_index_roving_prev_non_looped_ltr() {
        let mut roving = IndexRoving {
            current: 0,
            length: 3,
            r#loop: false,
        };

        assert_eq!(roving.prev(&Dir::Ltr), None);
    }

    #[test]
    fn test_index_roving_next_looped_rtl() {
        let mut roving = IndexRoving {
            current: 0,
            length: 3,
            r#loop: true,
        };

        assert_eq!(roving.next(&Dir::Rtl), Some(2));
        assert_eq!(roving.next(&Dir::Rtl), Some(1));
        assert_eq!(roving.next(&Dir::Rtl), Some(0));
        assert_eq!(roving.next(&Dir::Rtl), Some(2));
    }

    #[test]
    fn test_index_roving_prev_looped_rtl() {
        let mut roving = IndexRoving {
            current: 2,
            length: 3,
            r#loop: true,
        };

        assert_eq!(roving.prev(&Dir::Rtl), Some(0));
        assert_eq!(roving.prev(&Dir::Rtl), Some(1));
        assert_eq!(roving.prev(&Dir::Rtl), Some(2));
        assert_eq!(roving.prev(&Dir::Rtl), Some(0));
    }

    #[test]
    fn test_index_roving_next_non_looped_rtl() {
        let mut roving = IndexRoving {
            current: 0,
            length: 3,
            r#loop: false,
        };

        assert_eq!(roving.next(&Dir::Rtl), None);
    }

    #[test]
    fn test_index_roving_prev_non_looped_rtl() {
        let mut roving = IndexRoving {
            current: 2,
            length: 3,
            r#loop: false,
        };

        assert_eq!(roving.prev(&Dir::Rtl), None);
    }

    #[test]
    fn test_index_roving_first_ltr() {
        let mut roving = IndexRoving {
            current: 2,
            length: 3,
            r#loop: true,
        };

        assert_eq!(roving.first(&Dir::Ltr), Some(0));
    }

    #[test]
    fn test_index_roving_last_ltr() {
        let mut roving = IndexRoving {
            current: 2,
            length: 3,
            r#loop: true,
        };

        assert_eq!(roving.last(&Dir::Ltr), Some(2));
    }

    #[test]
    fn test_index_roving_first_rtl() {
        let mut roving = IndexRoving {
            current: 2,
            length: 3,
            r#loop: true,
        };

        assert_eq!(roving.first(&Dir::Rtl), Some(2));
    }

    #[test]
    fn test_index_roving_last_rtl() {
        let mut roving = IndexRoving {
            current: 2,
            length: 3,
            r#loop: true,
        };

        assert_eq!(roving.last(&Dir::Rtl), Some(0));
    }
}
