//our own smartpointer:
struct _MyBox<T>(T);

use std::ops::Deref;

impl<T> Deref for _MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> _MyBox<T> {
    fn _new(x: T) -> _MyBox<T> {
        _MyBox(x)
    }
}

fn main() {
    let b = Box::new(5);
    //stores data on the heap rather than the stack
    println!("b = {b}");

    let mut x = 10;
    {
        let y = &mut x; // y borrows x mutably
        *y = 15; // Change x through y
    } // y's scope ends here, mutable borrow is dropped

    x = 20; // Now it’s safe to modify x directly

    println!("x: {}", x); // Prints: 20
}

//drop trait:

fn _drop() {
    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        // the Drop is important -> Basically a destructor, not really needed to be called as it gets called automatically at the end
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }

    fn main() {
        let _c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        let _d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        println!("CustomSmartPointers created.");
    }

    // even though _c.drop() or _d.drop() is not called, it will still be called
}
