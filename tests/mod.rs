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


#![cfg(test)]

use vector_growing::{vec_grow, VecGrow};

#[test]
fn test_create_vec_grow() {
    let _vg = VecGrow::<i32>::new();
}

#[test]
fn test_push_and_remove_element() {
    let mut vg = VecGrow::new();
    vg.push(1);
    vg.push(2);

    assert_eq!(vg[0], Some(1));
    assert_eq!(vg[1], Some(2));

    vg.remove(0);

    assert_eq!(vg[0], None);

    vg.push(1);

    assert_eq!(vg[0], Some(1));
}

#[test]
fn test_macro_vec_grow() {
    let vg_empty: VecGrow<u8> = vec_grow![];
    let vg_num = vec_grow![1, 2, 3];
    let vg_zero = vec_grow![0; 100];

    assert!(vg_empty.is_empty());
    assert_eq!(vg_num[1], Some(2));
    assert_eq!(vg_zero[99], Some(0));
}

#[test]
fn test_iter() {
    let vg = vec_grow!(1, 2, 3);
    let mut vg_iter = vg.iter();

    assert_eq!(vg_iter.next(), Some(&1));
    assert_eq!(vg_iter.next(), Some(&2));
    assert_eq!(vg_iter.next(), Some(&3));
}

#[test]
fn test_iter_mut() {
    let mut vg = vec_grow!(1, 2, 3);

    for num in vg.iter_mut() {
        *num += 1;
    }

    let mut vg_iter = vg.iter();
    assert_eq!(vg_iter.next(), Some(&2));
    assert_eq!(vg_iter.next(), Some(&3));
    assert_eq!(vg_iter.next(), Some(&4));
}
