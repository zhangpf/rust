#![feature(fn_traits)]

fn id<T>(t: T) -> T { t }

fn f<'r, T>(v: &'r T) -> Box<FnMut() -> T + 'r> {
    id(Box::new(|| *v))
        //~^ ERROR E0373
        //~| ERROR E0507
}

fn main() {
    let v = &5;
    println!("{}", f(v).call_mut(()));
}
