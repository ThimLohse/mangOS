#![feature(lang_items)]
#![feature(unique)]
#![feature(const_fn)]
#![feature(const_unique_new)]
#![no_std]
extern crate rlibc;
extern crate volatile;
extern crate spin;
extern crate multiboot2;

#[macro_use]
extern crate bitflags;
extern crate x86_64;

#[macro_use]
mod vga_buffer;
mod memory;


#[no_mangle]
pub extern "C" fn rust_main(multiboot_information_address: usize) {
    //ATTENTION: we have a very small stack and no guard page
    
    //Extra lines from github
    use memory::FrameAllocator;
    
    //clear screen
    vga_buffer::clear_screen();
    println!("Hello World{}", "!");

    let boot_info = unsafe{ multiboot2::load(multiboot_information_address) };
    let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required");
    let elf_sections_tag = boot_info.elf_sections_tag().expect("Elf-sections tag required");

    //Calculate start and end addresses of our loaded kernel.
    let kernel_start = elf_sections_tag.sections().map(|s| s.addr).min().unwrap();
    let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size).max().unwrap();

    //Retrieve the multiboot information structure
    let multiboot_start = multiboot_information_address;
    let multiboot_end = multiboot_start + (boot_info.total_size as usize);

    println!("kernel start: 0x{:x}, kernel end: 0x{:x}", kernel_start, kernel_end);
    println!("multiboot start: 0x{:x}, multiboot end: 0x{:x}",  multiboot_start, multiboot_end);
    println!("");
    

    
     let mut frame_allocator = memory::AreaFrameAllocator::new(
        kernel_start as usize, kernel_end as usize, multiboot_start,
         multiboot_end, memory_map_tag.memory_areas());

    
    //call function to test paging
    //memory::test_paging(&mut frame_allocator);

    //test remapping the kernel (Enable NoExecute bit before to avoid page fault)
    enable_nxe_bit();
    enable_write_protect_bit();
    memory::remap_the_kernel(&mut frame_allocator, boot_info);

    //new : try to allocate a frame -> should cause page fault as the AreaFrameAllocator uses the memory map of the Multiboot information structure, but we did not map the Multibbot structure.
    frame_allocator.allocate_frame();
    
    println!("It did not crash!");


    loop{}
   


}
fn enable_write_protect_bit() {
    use x86_64::registers::control_regs::{cr0, cr0_write, Cr0};

    unsafe { cr0_write(cr0() | Cr0::WRITE_PROTECT) };
}
fn enable_nxe_bit(){
    use x86_64::registers::msr::{IA32_EFER, rdmsr, wrmsr};

    let nxe_bit = 1 << 11;
    unsafe {
        let efer = rdmsr(IA32_EFER);
        wrmsr(IA32_EFER, efer | nxe_bit);
    }
}

fn print_evil_computer_init(){
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
 

loop{}
}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle]
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("   {}", fmt);
    loop{}
}
