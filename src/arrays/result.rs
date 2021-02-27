use core::fmt::Debug;
use std::time::SystemTime;
use crate::arrays::{PureArray, SingleArray, VectorArray, FactorArray, MyArray};

/*

SingleArray.
Массив, где мы при добавлении каждого нового элемента: создаем новый массив размером на 1 больше текущего,
копируем содеримое в новый массив, добавляем новое значение в новый слот.
Как и ожидается, работает крайне долго, добавление ВСЕГОЛИШЬ 20к элементов занимает 15 сек.

VectorArray.
Массив, где мы создаем массив с запасом в N ячеек, если они все забиваются, то создаем новый массив
на N ячеек больше текущего. Копируем массив только раз в N добавлений.
Работает быстрее чем SingleArray, но все равно долго, сильно зависит от N.
Чем больше N тем быстрее рабортает, но чем больше N тем больше холостой памяти для маленких массивов.
Добавление 50к элементов при N = 10 равно добавлению 500к элементов при N = 1000 (10сек).

FactorArray.
Продолжение идей VectorArray, только размер масива умеличиваем в 2 раза (или другой множитель),
если текущий массив заполнился.
Работает крайне быстро, 10кк элементов добавилось менее чем за 2 секунды.

PureArray.
Обертка над динамическим массивом (vec) из стандартной библиотеки.
Вообще все предыдущие массивы тоже использовали vec, но я нуб в Rust. Так [i32; length] не работает,
вместо length должна быть статическая константа.
Работает примерно в три раза быстрее чем FactorArray.

Test: "SingleArray"  N:1_000 time:  0.0386 sec
Test: "SingleArray" N:10_000 time:  3.8576 sec
Test: "SingleArray" N:20_000 time: 15.5518 sec

Test: "VectorArray::new(10)"     N:1_000 time: 0.0039 sec
Test: "VectorArray::new(10)"    N:10_000 time: 0.3795 sec
Test: "VectorArray::new(10)"    N:20_000 time: 1.4855 sec
Test: "VectorArray::new(10)"    N:50_000 time: 9.5125 sec
Test: "VectorArray::new(100)"    N:1_000 time: 0.0004 sec
Test: "VectorArray::new(100)"   N:10_000 time: 0.0367 sec
Test: "VectorArray::new(100)"   N:20_000 time: 0.1579 sec
Test: "VectorArray::new(100)"   N:50_000 time: 0.9461 sec
Test: "VectorArray::new(100)"  N:100_000 time: 3.7942 sec
Test: "VectorArray::new(1000)" N:100_000 time: 0.3959 sec
Test: "VectorArray::new(1000)" N:500_000 time: 9.5184 sec

Test: "FactorArray::new()"      N:1_000 time: 0.0001 sec
Test: "FactorArray::new()"     N:10_000 time: 0.0018 sec
Test: "FactorArray::new()"    N:100_000 time: 0.0152 sec
Test: "FactorArray::new()"  N:1_000_000 time: 0.1353 sec
Test: "FactorArray::new()" N:10_000_000 time: 1.8687 sec

Test: "PureArray::new()"      N:1_000 time: 0.0001 sec
Test: "PureArray::new()"     N:10_000 time: 0.0005 sec
Test: "PureArray::new()"    N:100_000 time: 0.0049 sec
Test: "PureArray::new()"  N:1_000_000 time: 0.0514 sec
Test: "PureArray::new()" N:10_000_000 time: 0.5432 sec
*/
pub fn run_array_tests() {
    // test_array(&mut SingleArray::new());
    // test_array(&mut VectorArray::new(3));
    // test_array(&mut FactorArray::new());

    // test_arr("SingleArray", &mut SingleArray::new(), 1_000);
    // test_arr("SingleArray", &mut SingleArray::new(), 10_000);
    // test_arr("SingleArray", &mut SingleArray::new(), 20_000);

    // test_arr("VectorArray::new(10)", &mut VectorArray::new(10), 1_000);
    // test_arr("VectorArray::new(10)", &mut VectorArray::new(10), 10_000);
    // test_arr("VectorArray::new(10)", &mut VectorArray::new(10), 20_000);
    // test_arr("VectorArray::new(10)", &mut VectorArray::new(10), 50_000);
    // test_arr("VectorArray::new(100)", &mut VectorArray::new(100), 1_000);
    // test_arr("VectorArray::new(100)", &mut VectorArray::new(100), 10_000);
    // test_arr("VectorArray::new(100)", &mut VectorArray::new(100), 20_000);
    // test_arr("VectorArray::new(100)", &mut VectorArray::new(100), 50_000);
    // test_arr("VectorArray::new(100)", &mut VectorArray::new(100), 100_000);
    // test_arr("VectorArray::new(1000)", &mut VectorArray::new(1000), 100_000);
    // test_arr("VectorArray::new(1000)", &mut VectorArray::new(1000), 500_000);

    // test_arr("FactorArray::new()", &mut FactorArray::new(), 1_000);
    // test_arr("FactorArray::new()", &mut FactorArray::new(), 10_000);
    // test_arr("FactorArray::new()", &mut FactorArray::new(), 100_000);
    // test_arr("FactorArray::new()", &mut FactorArray::new(), 1_000_000);
    // test_arr("FactorArray::new()", &mut FactorArray::new(), 10_000_000);

    // test_arr("PureArray::new()", &mut PureArray::new(), 1_000);
    // test_arr("PureArray::new()", &mut PureArray::new(), 10_000);
    // test_arr("PureArray::new()", &mut PureArray::new(), 100_000);
    // test_arr("PureArray::new()", &mut PureArray::new(), 1_000_000);
    // test_arr("PureArray::new()", &mut PureArray::new(), 10_000_000);
}

fn test_arr<T: MyArray>(title: &str, arr: &mut T, n: i32) {
    let sys_time = SystemTime::now();
    for i in 0..n {
        arr.add(i);
    }
    let difference = SystemTime::now().duration_since(sys_time).unwrap();
    println!(
        "Test: \"{}\" N:{} time: {:.4} sec",
        title,
        n,
        difference.as_secs_f32()
    );
}

fn test_array<T: MyArray + std::fmt::Debug>(my_arr: &mut T) {
    my_arr.add(1);
    my_arr.add(2);
    my_arr.add(3);
    my_arr.add(4);
    my_arr.add(5);

    println!("arr: {:?}", my_arr);

    my_arr.add_to(10, 2);
    println!("arr: {:?}", my_arr);

    let removed = my_arr.remove(3);
    println!("arr: {:?} -------- {:?}", my_arr, removed);
}
