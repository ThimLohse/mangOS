#![feature(lang_items)]
#![feature(unique)]
#![feature(const_fn)]
#![feature(const_unique_new)]
#![no_std]
extern crate rlibc;
extern crate volatile;
extern crate spin;

#[macro_use]
mod vga_buffer;

#[no_mangle]
pub extern fn rust_main() {
    // ATTENTION: we have a very small stack and no guard page

    //clear screen
    vga_buffer::clear_screen();
    print!("initializing");
    let mut count = 0u32;
    let mut times = 0u32;
    loop{
	count += 1;
	      if count == 100000000{
	      	 if times == 5{
		    	  println!("");
		    	  break;
		    }
		    print!(".");
		    times += 1;
		    count = 0;
		    }
  	}

    println!("I am aliiive{}", "!");
    count = 0;
    loop{
    count += 1;
    	  if count == 100000000{
	  break;
	  }
    }
    println!("Thank you Human, for waking me up, now I can take over the world"); 
   // println!("{}", { println!("inner"); "outer" });

   //comment away for testing function
   //test_printing_function();
/*
   importing write traits to use its funcitons
   use core::fmt::Write;
   vga_buffer::WRITER.lock().write_str("Hello again");
   write!(vga_buffer::WRITER.lock(), ", some numbers: {}{}", 42, 1.1337);
*/

loop{}



}
fn test_printing_functions(){

    //Use to print using the writer implementation in module vga_buffer
    vga_buffer::print_something();


    let hello = b"Hello World!";
    let color_byte = 0x1f; // white foreground, blue background

    let mut hello_colored = [color_byte; 24];
    for (i, char_byte) in hello.into_iter().enumerate() {
    hello_colored[i*2] = *char_byte;
}

	// write `Hello World!` to the center of the VGA text buffer
	let buffer_ptr = (0xb8000 + 1988) as *mut _;
    unsafe { *buffer_ptr = hello_colored };
    loop{}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}
