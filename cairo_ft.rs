/* automatically generated by rust-bindgen */

import libc::*;

#[link_name="cairo"]
native mod bindgen {

fn cairo_ft_font_face_create_for_ft_face(++face: FT_Face, ++load_flags: c_int) -> *cairo_font_face_t;

fn cairo_ft_scaled_font_lock_face(++scaled_font: *cairo_scaled_font_t) -> FT_Face;

fn cairo_ft_scaled_font_unlock_face(++scaled_font: *cairo_scaled_font_t);

fn cairo_ft_font_face_create_for_pattern(++pattern: *FcPattern) -> *cairo_font_face_t;

fn cairo_ft_font_options_substitute(++options: *cairo_font_options_t, ++pattern: *FcPattern);

}