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

use std::marker::PhantomData;

use crate::VecGrow;

pub struct Iter<'a, T> {
    counter: usize,
    inner: &'a VecGrow<T>,
}

impl<'a, T> Iter<'a, T> {
    pub fn new(data: &'a VecGrow<T>) -> Self {
        Self {
            counter: 0,
            inner: data,
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.counter >= self.inner.objects.len() {
            return None;
        }

        let obj = &self.inner[self.counter];

        if let Some(obj) = obj {
            self.counter += 1;
            return Some(obj);
        } else {
            self.counter += 1;
            return self.next();
        }
    }
}

pub struct IterMut<'a, T> {
    counter: usize,
    inner: *mut VecGrow<T>,
    _marker: PhantomData<&'a mut Option<T>>,
}

impl<'a, T> IterMut<'a, T> {
    pub fn new(data: &'a mut VecGrow<T>) -> Self {
        Self {
            counter: 0,
            inner: data,
            _marker: PhantomData::default(),
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let data = &mut *self.inner;
            if self.counter >= data.objects.len() {
                return None;
            }

            let obj = &mut data[self.counter];

            if let Some(obj) = obj {
                self.counter += 1;
                return Some(obj);
            } else {
                self.counter += 1;
                return self.next();
            }
        }
    }
}
