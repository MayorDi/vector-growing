//    Copyright 2024 Dmitriy Mayorov

//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at

//        http://www.apache.org/licenses/LICENSE-2.0

//    Unless required by applicable law or agreed to in writing, software
//    distributed under the License is distributed on an "AS IS" BASIS,
//    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//    See the License for the specific language governing permissions and
//    limitations under the License.


use std::ops::{Index, IndexMut};
mod iter;
use iter::*;

/// It is similar to a regular `Vec`, with one exception - 
/// the size is not reduced. `VecGrow` can constantly grow, 
/// but not decrease, and when objects are deleted, 
/// their place remains for the new object as a free index, 
/// which eases the memory power in the case of permanent 
/// deletion and creation of objects.
/// # Examples
/// ```
/// use vector_growing::*;
/// 
/// let mut vg = VecGrow::new();
/// vg.push(1);
/// vg.push(2);
/// 
/// assert_eq!(vg[0], Some(1));
/// assert_eq!(vg[1], Some(2));
/// 
/// vg.remove(0);
/// 
/// assert_eq!(vg[0], None);
/// 
/// vg.push(1);
/// 
/// assert_eq!(vg[0], Some(1));
/// ```
/// Initializing VecGrow using a macro:
/// ```
/// use vector_growing::*;
/// 
/// let vg_empty: VecGrow<u8> = vec_grow![];
/// let vg_num = vec_grow![1, 2, 3];
/// let vg_zero = vec_grow![0; 100];
/// 
/// assert!(vg_empty.is_empty());
/// assert_eq!(vg_num[1], Some(2));
/// assert_eq!(vg_zero[99], Some(0));
/// ```
#[derive(Debug, Clone)]
pub struct VecGrow<T> {
    free_idxs: Vec<usize>,
    objects: Vec<Option<T>>,
}

impl<T> VecGrow<T> {
    /// Create of `VecGrow`
    /// 
    /// # Examples
    /// ```
    /// use vector_growing::*;
    /// let _vg: VecGrow<u8> = VecGrow::new();
    /// // or
    /// let _vg: VecGrow<u8> = vec_grow!();
    /// let _vg = vec_grow!(0);
    /// let _vg = vec_grow!(0; 100);
    /// let _vg = vec_grow!(1, 2, 3);
    /// ```
    pub fn new() -> Self {
        Self {
            free_idxs: vec![],
            objects: vec![],
        }
    }

    /// Adding a new element to the place of the old one, 
    /// if there is no free space, selected a new place and add the element.
    pub fn push(&mut self, value: T) {
        if let Some(last_index) = self.free_idxs.last() {
            self.objects[*last_index] = Some(value);
            self.free_idxs.remove(*last_index);
        } else {
            self.objects.push(Some(value));
        }
    }

    /// Deleting an object and memorize the vacant index.
    pub fn remove(&mut self, index: usize) {
        fn assert_failed(index: usize, len: usize) -> ! {
            panic!("removal index (is {index}) should be < len (is {len})");
        }

        if index >= self.objects.len() {
            assert_failed(index, self.objects.len());
        }

        self.free_idxs.push(index);
        self[index] = None;
    }

    /// The count of real objects excluding free cells.
    pub fn count(&self) -> usize {
        self.objects.len() - self.free_idxs.len()
    }

    /// The length of the object vector
    pub fn len(&self) -> usize {
        self.objects.len()
    }

    

    /// Creating an immutable iterator.
    /// # Examples
    /// ```
    /// use vector_growing::*;
    /// let vg = vec_grow!(1, 2, 3);
    /// let mut vg_iter = vg.iter();
    ///
    /// assert_eq!(vg_iter.next(), Some(&1));
    /// assert_eq!(vg_iter.next(), Some(&2));
    /// assert_eq!(vg_iter.next(), Some(&3));
    /// ```
    pub fn iter(&self) -> Iter<'_, T> {
        Iter::new(self)
    }

    /// Creating an mutable iterator.
    /// # Examples
    /// ```
    /// use vector_growing::*;
    /// 
    /// let mut vg = vec_grow!(1, 2, 3);
    /// 
    /// for num in vg.iter_mut() {
    ///     *num += 1;
    /// }
    /// 
    /// let mut vg_iter = vg.iter();
    /// assert_eq!(vg_iter.next(), Some(&2));
    /// assert_eq!(vg_iter.next(), Some(&3));
    /// assert_eq!(vg_iter.next(), Some(&4));
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut::new(self)
    }

    pub fn is_empty(&self) -> bool {
        self.free_idxs.len() == self.len()
    }
}

impl<T> Index<usize> for VecGrow<T> {
    type Output = Option<T>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.objects[index]
    }
}

impl<T> IndexMut<usize> for VecGrow<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.objects[index]
    }
}

#[macro_export]
macro_rules! vec_grow {
    [] => {
        VecGrow::new()
    };

    [$($v:expr),*] => {
        {
            let mut vg = VecGrow::new();
            $(
                vg.push($v);
            )*

            vg
        }
    };

    [$value:expr; $count:expr] => {
        {
            let mut vg = VecGrow::new();
            for _ in 0..$count {
                vg.push($value);
            }

            vg
        }
    }
}
