use bytemuck::{cast_slice, AnyBitPattern, NoUninit};
use std::boxed::Box;
use std::cmp::min;
use std::os::raw::{c_char, c_uint, c_void};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSLexer {
    pub lookahead: i32,
    pub result_symbol: u16,
    pub advance: Option<unsafe extern "C" fn(*mut TSLexer, bool) -> ()>,
    pub mark_end: Option<unsafe extern "C" fn(*mut TSLexer) -> ()>,
    pub get_column: Option<unsafe extern "C" fn(*mut TSLexer) -> i32>,
    pub is_at_included_range_start: Option<unsafe extern "C" fn(*const TSLexer) -> bool>,
    pub eof: Option<unsafe extern "C" fn(*const TSLexer) -> bool>,
}

#[repr(C)]
struct ValidSymbols {
    newline: bool,
    indent: bool,
    dedent: bool,
    string_start: bool,
    string_content: bool,
    string_end: bool,
    comment: bool,
    close_paren: bool,
    close_bracket: bool,
    close_brace: bool,
}

#[allow(dead_code)]
mod symbol_values {
    pub const NEWLINE: u16 = 0;
    pub const INDENT: u16 = 1;
    pub const DEDENT: u16 = 2;
    pub const STRING_START: u16 = 3;
    pub const STRING_CONTENT: u16 = 4;
    pub const STRING_END: u16 = 5;
    pub const COMMENT: u16 = 6;
    pub const CLOSE_PAREN: u16 = 7;
    pub const CLOSE_BRACKET: u16 = 8;
    pub const CLOSE_BRACE: u16 = 9;
}

#[derive(Clone, Copy, NoUninit, AnyBitPattern)]
#[repr(transparent)]
struct Delimiter {
    flags: u8,
}

impl Delimiter {
    const SINGLE_QUOTE: u8 = 1 << 0;
    const DOUBLE_QUOTE: u8 = 1 << 1;
    const BACK_QUOTE: u8 = 1 << 2;
    const RAW: u8 = 1 << 3;
    const FORMAT: u8 = 1 << 4;
    const TRIPLE: u8 = 1 << 5;
    const BYTES: u8 = 1 << 6;

    fn new() -> Self {
        Delimiter { flags: 0 }
    }

    fn is_format(&self) -> bool {
        self.flags & Delimiter::FORMAT != 0
    }

    fn is_raw(&self) -> bool {
        self.flags & Delimiter::RAW != 0
    }

    fn is_triple(&self) -> bool {
        self.flags & Delimiter::TRIPLE != 0
    }

    fn is_bytes(&self) -> bool {
        self.flags & Delimiter::BYTES != 0
    }

    fn end_character(&self) -> i32 {
        if self.flags & Delimiter::SINGLE_QUOTE != 0 {
            '\'' as i32
        } else if self.flags & Delimiter::DOUBLE_QUOTE != 0 {
            '"' as i32
        } else if self.flags & Delimiter::BACK_QUOTE != 0 {
            '`' as i32
        } else {
            0
        }
    }

    fn set_format(&mut self) {
        self.flags |= Delimiter::FORMAT;
    }

    fn set_raw(&mut self) {
        self.flags |= Delimiter::RAW;
    }

    fn set_triple(&mut self) {
        self.flags |= Delimiter::TRIPLE;
    }

    fn set_bytes(&mut self) {
        self.flags |= Delimiter::BYTES;
    }

    fn set_end_character(&mut self, character: char) {
        match character {
            '\'' => self.flags |= Delimiter::SINGLE_QUOTE,
            '"' => self.flags |= Delimiter::DOUBLE_QUOTE,
            '`' => self.flags |= Delimiter::BACK_QUOTE,
            _ => panic!("Invalid end character"),
        }
    }
}

const BUFFER_SIZE: usize = 1024;
struct Scanner {
    // this is stored as u16 in orig but serialized as u8 we so might as well use u8 all the time
    indent_length_stack: Vec<u8>,

    // stored as another vector of u8
    delimiter_stack: Vec<Delimiter>,
}

impl Scanner {
    fn new() -> Self {
        Scanner {
            indent_length_stack: vec![0],
            delimiter_stack: Vec::new(),
        }
    }

    fn serialize(&mut self, buffer: &mut [u8]) -> usize {
        let mut i = 0;

        // prevent overflow (because we only have 1 byte to store the delimiter count)
        let delimiter_count = min(self.delimiter_stack.len(), u8::MAX as usize);

        // first bit is the number of delimiters in the stack
        buffer[i] = delimiter_count as u8;
        i += 1;

        // copy delimiters into buffer
        if delimiter_count > 0 {
            buffer[i..(i + delimiter_count)]
                .copy_from_slice(cast_slice(self.delimiter_stack.as_slice()));
            i += delimiter_count;
        }

        // copy indent lengths into buffer (do not overflow buffer)
        let remaining_space = BUFFER_SIZE - i;
        let indents_to_copy = min(self.indent_length_stack.len() - 1, remaining_space);

        if indents_to_copy > 0 {
            buffer[i..(i + indents_to_copy)].copy_from_slice(
                // skip the first index, which is always 0
                &self.indent_length_stack[1..(1 + indents_to_copy)],
            );
            i += indents_to_copy;
        }

        // return the number of bytes written
        i
    }

    fn deserialize(&mut self, buffer: &[u8], length: usize) {
        // clear existing stacks
        self.delimiter_stack.clear();
        self.indent_length_stack.clear();

        // always start with index 0
        self.indent_length_stack.push(0);

        // return if nothing to deserialize
        if length == 0 {
            return;
        }

        let mut i = 0;

        // the first byte of the buffer is the number of delimiters
        let delimiter_count = buffer[i];
        i += 1;

        // the next delimiter_count bytes are the delimiters
        self.delimiter_stack
            .extend_from_slice(cast_slice(&buffer[i..(i + delimiter_count as usize)]));
        i += delimiter_count as usize;

        // the remaining bytes are the indent lengths
        self.indent_length_stack
            .extend_from_slice(&buffer[i..length]);
    }

    fn scan(&mut self, lexer: &mut TSLexer, valid_symbols: &ValidSymbols) -> bool {
        use symbol_values::*;

        let error_recovery_mode = valid_symbols.string_content && valid_symbols.indent;
        let within_brackets =
            valid_symbols.close_brace || valid_symbols.close_paren || valid_symbols.close_bracket;

        if valid_symbols.string_content && !self.delimiter_stack.is_empty() && !error_recovery_mode
        {
            let delimiter = self.delimiter_stack.last().unwrap();
            let end_character = delimiter.end_character();
            let mut has_content = false;

            while lexer.lookahead != 0 {
                if (lexer.lookahead == '{' as i32 || lexer.lookahead == '}' as i32)
                    && delimiter.is_format()
                {
                    mark_end(lexer);
                    lexer.result_symbol = STRING_CONTENT;
                    return has_content;
                } else if lexer.lookahead == '\\' as i32 {
                    if delimiter.is_raw() {
                        // Step over the backslash.
                        advance(lexer);
                        // Step over any escaped quotes.
                        if lexer.lookahead == delimiter.end_character()
                            || lexer.lookahead == '\\' as i32
                        {
                            advance(lexer);
                        }
                        continue;
                    } else if delimiter.is_bytes() {
                        mark_end(lexer);
                        advance(lexer);
                        if lexer.lookahead == 'N' as i32
                            || lexer.lookahead == 'u' as i32
                            || lexer.lookahead == 'U' as i32
                        {
                            // In bytes string, \N{...}, \uXXXX and \UXXXXXXXX are not escape sequences
                            // https://docs.python.org/3/reference/lexical_analysis.html#string-and-bytes-literals
                            advance(lexer);
                        } else {
                            lexer.result_symbol = STRING_CONTENT;
                            return has_content;
                        }
                    } else {
                        mark_end(lexer);
                        lexer.result_symbol = STRING_CONTENT;
                        return has_content;
                    }
                } else if lexer.lookahead == end_character {
                    if delimiter.is_triple() {
                        mark_end(lexer);
                        advance(lexer);
                        if lexer.lookahead == end_character {
                            advance(lexer);
                            if lexer.lookahead == end_character {
                                if has_content {
                                    lexer.result_symbol = STRING_CONTENT;
                                } else {
                                    advance(lexer);
                                    mark_end(lexer);
                                    self.delimiter_stack.pop();
                                    lexer.result_symbol = STRING_END;
                                }
                                return true;
                            } else {
                                mark_end(lexer);
                                lexer.result_symbol = STRING_CONTENT;
                                return true;
                            }
                        } else {
                            mark_end(lexer);
                            lexer.result_symbol = STRING_CONTENT;
                            return true;
                        }
                    } else {
                        if has_content {
                            lexer.result_symbol = STRING_CONTENT;
                        } else {
                            advance(lexer);
                            self.delimiter_stack.pop();
                            lexer.result_symbol = STRING_END;
                        }
                        mark_end(lexer);
                        return true;
                    }
                } else if lexer.lookahead == '\n' as i32 && has_content && !delimiter.is_triple() {
                    return false;
                }
                advance(lexer);
                has_content = true;
            }
        }

        mark_end(lexer);

        // find the end of the line.
        // identify indentation level and first comment along the way
        let mut found_end_of_line = false;
        let mut indent_length: u8 = 0;
        let mut first_comment_indent_length: Option<u8> = None;
        loop {
            match std::char::from_u32(lexer.lookahead as u32).unwrap() {
                '\n' => {
                    found_end_of_line = true;
                    indent_length = 0;
                    skip(lexer);
                }
                ' ' => {
                    indent_length += 1;
                    skip(lexer);
                }
                '\r' => {
                    indent_length = 0;
                    skip(lexer);
                }
                '\t' => {
                    indent_length += 8;
                    skip(lexer);
                }
                '#' => {
                    if first_comment_indent_length.is_none() {
                        first_comment_indent_length = Some(indent_length);
                    }
                    while lexer.lookahead != 0 && lexer.lookahead != '\n' as i32 {
                        skip(lexer);
                    }
                    skip(lexer);
                    indent_length = 0;
                }
                '\\' => {
                    skip(lexer);
                    if lexer.lookahead == '\r' as i32 {
                        skip(lexer);
                    }
                    if lexer.lookahead == '\n' as i32 {
                        skip(lexer);
                    } else {
                        return false;
                    }
                }
                '\x0c' => {
                    indent_length = 0;
                    skip(lexer);
                }
                '\0' => {
                    indent_length = 0;
                    found_end_of_line = true;
                    break;
                }
                _ => break,
            }
        }

        if found_end_of_line {
            if !self.indent_length_stack.is_empty() {
                let current_indent_length = *self.indent_length_stack.last().unwrap(); // will always have a 0 element at the start

                if valid_symbols.indent && indent_length > current_indent_length {
                    self.indent_length_stack.push(indent_length);
                    lexer.result_symbol = INDENT;
                    return true;
                }

                if (valid_symbols.dedent || !valid_symbols.newline && !within_brackets)
                    && indent_length < current_indent_length
                    // Wait to create a dedent token until we've consumed any comments
                    // whose indentation matches the current block.
                    && (first_comment_indent_length.is_none()
                        || first_comment_indent_length.unwrap() < current_indent_length)
                {
                    self.indent_length_stack.pop();
                    lexer.result_symbol = DEDENT;
                    return true;
                }
            }

            if valid_symbols.newline && !error_recovery_mode {
                lexer.result_symbol = NEWLINE;
                return true;
            }
        }

        if first_comment_indent_length.is_none() && valid_symbols.string_start {
            let mut delimiter = Delimiter::new();

            let mut has_flags = false;
            loop {
                match std::char::from_u32(lexer.lookahead as u32).unwrap() {
                    'f' | 'F' => delimiter.set_format(),
                    'r' | 'R' => delimiter.set_raw(),
                    'b' | 'B' => delimiter.set_bytes(),
                    'u' | 'U' => {}
                    _ => break,
                }

                has_flags = true;
                advance(lexer);
            }

            if lexer.lookahead == '`' as i32 {
                delimiter.set_end_character('`');
                advance(lexer);
                mark_end(lexer);
            } else if lexer.lookahead == '\'' as i32 {
                delimiter.set_end_character('\'');
                advance(lexer);
                mark_end(lexer);
                if lexer.lookahead == '\'' as i32 {
                    advance(lexer);
                    if lexer.lookahead == '\'' as i32 {
                        advance(lexer);
                        mark_end(lexer);
                        delimiter.set_triple();
                    }
                }
            } else if lexer.lookahead == '"' as i32 {
                delimiter.set_end_character('"');
                advance(lexer);
                mark_end(lexer);
                if lexer.lookahead == '"' as i32 {
                    advance(lexer);
                    if lexer.lookahead == '"' as i32 {
                        advance(lexer);
                        mark_end(lexer);
                        delimiter.set_triple();
                    }
                }
            }

            if delimiter.end_character() != 0 {
                self.delimiter_stack.push(delimiter);
                lexer.result_symbol = STRING_START;
                return true;
            } else if has_flags {
                return false;
            }
        }

        return false;
    }
}

fn advance(lexer: &mut TSLexer) {
    unsafe {
        lexer.advance.unwrap()(lexer, false);
    }
}

fn skip(lexer: &mut TSLexer) {
    unsafe {
        lexer.advance.unwrap()(lexer, true);
    }
}

fn mark_end(lexer: &mut TSLexer) {
    unsafe {
        lexer.mark_end.unwrap()(lexer);
    }
}

/* ======== Interface ======== */
#[no_mangle]
pub unsafe extern "C" fn tree_sitter_python_external_scanner_create() -> *mut c_void {
    let scanner = Box::new(Scanner::new());
    Box::into_raw(scanner) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn tree_sitter_python_external_scanner_destroy(payload: *mut c_void) {
    unsafe { Box::from_raw(payload as *mut Scanner) };
}

#[no_mangle]
pub unsafe extern "C" fn tree_sitter_python_external_scanner_scan(
    payload: *mut c_void,
    lexer: *mut TSLexer,
    valid_symbols: *const bool,
) -> bool {
    let scanner = payload as *mut Scanner;
    let valid_symbols = valid_symbols as *const ValidSymbols;
    (*scanner).scan(&mut *lexer, &*valid_symbols)
}

#[no_mangle]
pub unsafe extern "C" fn tree_sitter_python_external_scanner_serialize(
    payload: *mut c_void,
    buffer: *mut c_char,
) -> c_uint {
    let scanner = payload as *mut Scanner;
    let buffer = std::slice::from_raw_parts_mut(buffer as *mut u8, BUFFER_SIZE);
    (*scanner).serialize(buffer) as c_uint
}

#[no_mangle]
pub unsafe extern "C" fn tree_sitter_python_external_scanner_deserialize(
    payload: *mut c_void,
    buffer: *const c_char,
    length: c_uint,
) {
    let scanner = payload as *mut Scanner;
    let buffer = std::slice::from_raw_parts_mut(buffer as *mut u8, BUFFER_SIZE);
    (*scanner).deserialize(buffer, length as usize);
}
