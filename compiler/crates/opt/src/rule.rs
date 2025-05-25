mod downsize_prologue_epilogue;
mod fix_incomplete_label;
mod remove_futile_inst;
mod remove_futile_jmp;

pub use downsize_prologue_epilogue::downsize_prologue_epilogue;
pub use fix_incomplete_label::fix_incomplete_label;
pub use remove_futile_inst::remove_futile_inst;
pub use remove_futile_jmp::remove_futile_jmp;
