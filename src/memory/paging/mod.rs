pub use self::entry::*;
use memory::{PAGE_SIZE, Frame, FrameAllocator};

const ENTRY_COUNT: usize = 512;

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

pub struct Page {
   number: usize,
}

// function that translates a virtual address to the corresponding physical address
pub fn translate(virtual_address: VirtualAddress)
    -> Option<PhysicalAddress>
{
    let offset = virtual_address % PAGE_SIZE;
    translate_page(Page::containing_address(virtual_address))
        .map(|frame| frame.number * PAGE_SIZE + offset)
}


pub fn containing_address(address: VirtualAddress) -> Page {
    assert!(address < 0x0000_8000_0000_0000 ||
        address >= 0xffff_8000_0000_0000,
        "invalid address: 0x{:x}", address);
    Page { number: address / PAGE_SIZE }
}


fn start_address(&self) -> usize {
    self.number * PAGE_SIZE
}


fn translate_page(page: Page) -> Option<Frame> {
    use self::entry::HUGE_PAGE;

    let p3 = unsafe { &*table::P4 }.next_table(page.p4_index());

    let huge_page = || {
        // TODO
    };

    p3.and_then(|p3| p3.next_table(page.p3_index()))
      .and_then(|p2| p2.next_table(page.p2_index()))
      .and_then(|p1| p1[page.p1_index()].pointed_frame())
      .or_else(huge_page)
}


//maps a Page to a Frame
pub fn map_to<A>(page: Page, frame: Frame, flags: EntryFlags,
                 allocator: &mut A)
    where A: FrameAllocator
{
    let p4 = unsafe { &mut *P4 };
    let mut p3 = p4.next_table_create(page.p4_index(), allocator);
    let mut p2 = p3.next_table_create(page.p3_index(), allocator);
    let mut p1 = p2.next_table_create(page.p2_index(), allocator);

    assert!(p1[page.p1_index()].is_unused());
    p1[page.p1_index()].set(frame, flags | PRESENT);
}



//return the different table indexes
fn p4_index(&self) -> usize {
    (self.number >> 27) & 0o777
}
fn p3_index(&self) -> usize {
    (self.number >> 18) & 0o777
}
fn p2_index(&self) -> usize {
    (self.number >> 9) & 0o777
}
fn p1_index(&self) -> usize {
    (self.number >> 0) & 0o777
}

