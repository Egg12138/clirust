
#![allow(unused_variables)]
fn main() {
    #[cfg(any(unix, windows))]
    fn foo() {println!("Foo compiled");}

//    #[cfg(all(unix, target_pointer_width = "32"))]
    #[cfg(all(unix, target_pointer_width = "64"))]
    fn bar() {println!("Bar compiled");}

    #[cfg(not(foo))]
    fn not_foo() {println!("Foo is not compiled, so not_foo is compiled.");}

    fn nest_arbitrarily() {
        println!("not Unix, => Windows");
    } 

    
    foo();
    bar();
    not_foo();
    nest_arbitrarily();
    on_linux();
    if cfg!(target_os = "windows") {
        println!("Wrong!I'm running windows!");
    } else {
        println!("Right.");
    }

}

#[cfg(target_os = "linux")]
fn on_linux() {
    println!("On linux");
}
#[cfg(not(target_os = "linux"))]
fn on_linux() {
    println!("Not on linux");
}


