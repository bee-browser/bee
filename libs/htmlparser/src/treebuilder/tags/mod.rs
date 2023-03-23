// DO NOT EDIT THIS FILE BY HAND.
//
// This file was automagically generated by:
// bee-tools-codegen.js --no-escape --input-stdin mod.rs.hbs

mod a;
mod any_other;
mod aside;
mod b;
mod basefont;
mod bgsound;
mod body;
mod br;
mod caption;
mod center;
mod col;
mod colgroup;
mod dd;
mod div;
mod dl;
mod dt;
mod em;
mod font;
mod form;
mod frame;
mod frameset;
mod head;
mod hr;
mod html;
mod i;
mod img;
mod input;
mod li;
mod link;
mod main;
mod math;
mod meta;
mod nobr;
mod noframes;
mod noscript;
mod ol;
mod option;
mod p;
mod plaintext;
mod pre;
mod script;
mod select;
mod style;
mod svg;
mod table;
mod tbody;
mod td;
mod textarea;
mod tfoot;
mod th;
mod thead;
mod title;
mod tr;
mod ul;

use super::*;

impl<T> TreeBuilder<T>
where
    T: DomTreeBuilder,
{
    pub fn handle_start_tag(&mut self, tag: Tag<'_>) -> Control {
        self.ignore_lf = false;
        let local_name = LocalName::lookup(tag.name);
        match local_name {
            tag!(A) => self.handle_start_a(&tag),
            tag!(Aside) => self.handle_start_aside(&tag),
            tag!(B) => self.handle_start_b(&tag),
            tag!(Basefont) => self.handle_start_basefont(&tag),
            tag!(Bgsound) => self.handle_start_bgsound(&tag),
            tag!(Body) => self.handle_start_body(&tag),
            tag!(Br) => self.handle_start_br(&tag),
            tag!(Caption) => self.handle_start_caption(&tag),
            tag!(Center) => self.handle_start_center(&tag),
            tag!(Col) => self.handle_start_col(&tag),
            tag!(Colgroup) => self.handle_start_colgroup(&tag),
            tag!(Dd) => self.handle_start_dd(&tag),
            tag!(Div) => self.handle_start_div(&tag),
            tag!(Dl) => self.handle_start_dl(&tag),
            tag!(Dt) => self.handle_start_dt(&tag),
            tag!(Em) => self.handle_start_em(&tag),
            tag!(Font) => self.handle_start_font(&tag),
            tag!(Form) => self.handle_start_form(&tag),
            tag!(Frame) => self.handle_start_frame(&tag),
            tag!(Frameset) => self.handle_start_frameset(&tag),
            tag!(Head) => self.handle_start_head(&tag),
            tag!(Hr) => self.handle_start_hr(&tag),
            tag!(Html) => self.handle_start_html(&tag),
            tag!(I) => self.handle_start_i(&tag),
            tag!(Img) => self.handle_start_img(&tag),
            tag!(Input) => self.handle_start_input(&tag),
            tag!(Li) => self.handle_start_li(&tag),
            tag!(Link) => self.handle_start_link(&tag),
            tag!(Main) => self.handle_start_main(&tag),
            tag!(Math) => self.handle_start_math(&tag),
            tag!(Meta) => self.handle_start_meta(&tag),
            tag!(Nobr) => self.handle_start_nobr(&tag),
            tag!(Noframes) => self.handle_start_noframes(&tag),
            tag!(Noscript) => self.handle_start_noscript(&tag),
            tag!(Ol) => self.handle_start_ol(&tag),
            tag!(Option) => self.handle_start_option(&tag),
            tag!(P) => self.handle_start_p(&tag),
            tag!(Plaintext) => self.handle_start_plaintext(&tag),
            tag!(Pre) => self.handle_start_pre(&tag),
            tag!(Script) => self.handle_start_script(&tag),
            tag!(Select) => self.handle_start_select(&tag),
            tag!(Style) => self.handle_start_style(&tag),
            tag!(Svg) => self.handle_start_svg(&tag),
            tag!(Table) => self.handle_start_table(&tag),
            tag!(Tbody) => self.handle_start_tbody(&tag),
            tag!(Td) => self.handle_start_td(&tag),
            tag!(Textarea) => self.handle_start_textarea(&tag),
            tag!(Tfoot) => self.handle_start_tfoot(&tag),
            tag!(Th) => self.handle_start_th(&tag),
            tag!(Thead) => self.handle_start_thead(&tag),
            tag!(Tr) => self.handle_start_tr(&tag),
            tag!(Title) => self.handle_start_title(&tag),
            tag!(Ul) => self.handle_start_ul(&tag),
            _ => self.handle_start_any_other(&tag),
        }
    }

    pub fn handle_end_tag(&mut self, tag: Tag<'_>) -> Control {
        self.ignore_lf = false;
        let local_name = LocalName::lookup(tag.name);
        match local_name {
            tag!(A) => self.handle_end_a(&tag),
            tag!(Aside) => self.handle_end_aside(&tag),
            tag!(B) => self.handle_end_b(&tag),
            tag!(Basefont) => self.handle_end_basefont(&tag),
            tag!(Bgsound) => self.handle_end_bgsound(&tag),
            tag!(Body) => self.handle_end_body(&tag),
            tag!(Br) => self.handle_end_br(&tag),
            tag!(Caption) => self.handle_end_caption(&tag),
            tag!(Center) => self.handle_end_center(&tag),
            tag!(Col) => self.handle_end_col(&tag),
            tag!(Colgroup) => self.handle_end_colgroup(&tag),
            tag!(Dd) => self.handle_end_dd(&tag),
            tag!(Div) => self.handle_end_div(&tag),
            tag!(Dl) => self.handle_end_dl(&tag),
            tag!(Dt) => self.handle_end_dt(&tag),
            tag!(Em) => self.handle_end_em(&tag),
            tag!(Font) => self.handle_end_font(&tag),
            tag!(Form) => self.handle_end_form(&tag),
            tag!(Frame) => self.handle_end_frame(&tag),
            tag!(Frameset) => self.handle_end_frameset(&tag),
            tag!(Head) => self.handle_end_head(&tag),
            tag!(Hr) => self.handle_end_hr(&tag),
            tag!(Html) => self.handle_end_html(&tag),
            tag!(I) => self.handle_end_i(&tag),
            tag!(Img) => self.handle_end_img(&tag),
            tag!(Input) => self.handle_end_input(&tag),
            tag!(Li) => self.handle_end_li(&tag),
            tag!(Link) => self.handle_end_link(&tag),
            tag!(Main) => self.handle_end_main(&tag),
            tag!(Math) => self.handle_end_math(&tag),
            tag!(Meta) => self.handle_end_meta(&tag),
            tag!(Nobr) => self.handle_end_nobr(&tag),
            tag!(Noframes) => self.handle_end_noframes(&tag),
            tag!(Noscript) => self.handle_end_noscript(&tag),
            tag!(Ol) => self.handle_end_ol(&tag),
            tag!(Option) => self.handle_end_option(&tag),
            tag!(P) => self.handle_end_p(&tag),
            tag!(Plaintext) => self.handle_end_plaintext(&tag),
            tag!(Pre) => self.handle_end_pre(&tag),
            tag!(Script) => self.handle_end_script(&tag),
            tag!(Select) => self.handle_end_select(&tag),
            tag!(Style) => self.handle_end_style(&tag),
            tag!(Svg) => self.handle_end_svg(&tag),
            tag!(Table) => self.handle_end_table(&tag),
            tag!(Tbody) => self.handle_end_tbody(&tag),
            tag!(Td) => self.handle_end_td(&tag),
            tag!(Textarea) => self.handle_end_textarea(&tag),
            tag!(Tfoot) => self.handle_end_tfoot(&tag),
            tag!(Th) => self.handle_end_th(&tag),
            tag!(Thead) => self.handle_end_thead(&tag),
            tag!(Tr) => self.handle_end_tr(&tag),
            tag!(Title) => self.handle_end_title(&tag),
            tag!(Ul) => self.handle_end_ul(&tag),
            _ => self.handle_end_any_other(&tag),
        }
    }

    fn set_attributes_to_html_element(&mut self, tag: &Tag<'_>) {
        let node = self.html_element.expect("<html> must exist");
        self.inner.set_attribute(node, tag.attrs(), false);
    }

    fn set_attributes_to_body_element(&mut self, tag: &Tag<'_>) {
        let node = self.body_element.expect("<body> must exist");
        self.inner.set_attribute(node, tag.attrs(), false);
    }

    fn reset_insertion_mode_appropriately(&mut self) {
        self.switch_to(self.context().reset_mode);
    }

    fn clear_stack_back_to_table_context(&mut self) {
        loop {
            match self.context().open_element.local_name {
                tag!(Html, Table, Template) => break,
                _ => self.pop_element(),
            }
        }
    }

    fn clear_stack_back_to_table_body_context(&mut self) {
        loop {
            match self.context().open_element.local_name {
                tag!(Tbody, Tfoot, Thead) => break,
                _ => self.pop_element(),
            }
        }
    }

    fn clear_stack_back_to_table_row_context(&mut self) {
        loop {
            match self.context().open_element.local_name {
                tag!(Html, Template, Tr) => break,
                _ => self.pop_element(),
            }
        }
    }

    fn close_cell(&mut self) {
        // TODO: Generate implied end tags.
        loop {
            match self.context().open_element.local_name {
                tag!(Td, Th) => {
                    self.pop_element();
                    break;
                }
                _ => {
                    // TODO: Parse error.
                    self.pop_element();
                }
            }
        }
        // TODO: Clear the list of active formatting elements up to the last marker.
        self.switch_to(mode!(InRow));
    }

    fn is_visible_input(tag: &Tag<'_>) -> bool {
        debug_assert!(tag.name == "input");
        for (name, value) in tag.attrs() {
            if name == "type" {
                if value.eq_ignore_ascii_case("hidden") {
                    return false;
                }
                return true;
            }
        }
        true
    }
}
