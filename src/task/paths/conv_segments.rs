/****************************************************************************
**
** svgcleaner could help you to clean up your SVG files
** from unnecessary data.
** Copyright (C) 2012-2017 Evgeniy Reizner
**
** This program is free software; you can redistribute it and/or modify
** it under the terms of the GNU General Public License as published by
** the Free Software Foundation; either version 2 of the License, or
** (at your option) any later version.
**
** This program is distributed in the hope that it will be useful,
** but WITHOUT ANY WARRANTY; without even the implied warranty of
** MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
** GNU General Public License for more details.
**
** You should have received a copy of the GNU General Public License along
** with this program; if not, write to the Free Software Foundation, Inc.,
** 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
**
****************************************************************************/

use std::cmp::Ordering;

use svgdom::types::path::{Path, Segment, SegmentData};
use svgdom::types::{FuzzyEq, FuzzyOrd};

use super::utils;

pub fn convert_segments(path: &mut Path) {
    // repeat until we have any changes
    let mut is_changed = true;
    while is_changed {
        is_changed = false;

        if path.d.len() == 1 {
            path.d.clear();
            break;
        }

        _convert_segments(path, &mut is_changed);
    }
}

fn _convert_segments(path: &mut Path, is_changed: &mut bool) {
    let mut i = 1;
    while i < path.d.len() {
        let (prev_x, prev_y) = utils::resolve_xy(path, i - 1);
        let prev_seg = path.d[i - 1];
        let curr_seg = &mut path.d[i];
        match *curr_seg.data() {
            SegmentData::LineTo { x, y } => {
                if prev_x.fuzzy_eq(&x) && prev_y.fuzzy_ne(&y) {
                    *curr_seg = Segment::new_vline_to(y);
                    *is_changed = true;
                } else if prev_x.fuzzy_ne(&x) && prev_y.fuzzy_eq(&y) {
                    *curr_seg = Segment::new_hline_to(x);
                    *is_changed = true;
                }
            }
            SegmentData::CurveTo { x1, y1, x2, y2, x, y } => {
                let is_vlineto = || {
                    // if prev_x, x1, x2 and x are equal - this CurveTo is VerticalLineTo
                    // y1 must be equal or greater than prev_y
                    // y2 must be equal or less than y

                       prev_x.fuzzy_eq(&x)
                    && x1.fuzzy_eq(&x2)
                    && x1.fuzzy_eq(&x)
                    && y1.fuzzy_cmp(&prev_y) != Ordering::Less
                    && y2.fuzzy_cmp(&y) != Ordering::Greater
                };

                let is_hlineto = || {
                    // if prev_y, y1, y2 and y are equal - this CurveTo is HorizontalLineTo
                    // x1 must be equal or greater than prev_x
                    // x2 must be equal or less than x

                       prev_y.fuzzy_eq(&y)
                    && y1.fuzzy_eq(&y2)
                    && y1.fuzzy_eq(&y)
                    && x1.fuzzy_cmp(&prev_x) != Ordering::Less
                    && x2.fuzzy_cmp(&x) != Ordering::Greater
                };

                let is_lineto = || {
                       is_point_on_line(prev_x, prev_y, x, y, x1, y1)
                    && is_point_on_line(prev_x, prev_y, x, y, x2, y2)
                };

                if is_vlineto() {
                    *curr_seg = Segment::new_vline_to(y);
                    *is_changed = true;
                } else if is_hlineto() {
                    *curr_seg = Segment::new_hline_to(x);
                    *is_changed = true;
                } else if is_lineto() {
                    *curr_seg = Segment::new_line_to(x, y);
                    *is_changed = true;
                } else {
                    let (nx1, ny1) = match *prev_seg.data() {
                          SegmentData::CurveTo { x2: px2, y2: py2, x: px, y: py, .. }
                        | SegmentData::SmoothCurveTo { x2: px2, y2: py2, x: px, y: py } => {
                            (px * 2.0 - px2, py * 2.0 - py2)
                        }
                        _ => {
                            (prev_x, prev_y)
                        }
                    };

                    if x1.fuzzy_eq(&nx1) && y1.fuzzy_eq(&ny1) {
                        *curr_seg = Segment::new_smooth_curve_to(x2, y2, x, y);
                        *is_changed = true;
                    }
                }
            }
            // TODO: CurveTo -> Quadratic
            // TODO: Quadratic -> SmoothQuadTo
            _ => {}
        }

        i += 1;
    }
}

fn is_point_on_line(x1: f64, y1: f64, x2: f64, y2: f64, x: f64, y: f64) -> bool
{
    // check that point is actually on line
    let is_on_line = || {
        let a = (y2 - y1) / (x2 - x1);
        let b = y1 - a * x1;
        let c = (y - (a * x + b)).abs();
        c.fuzzy_eq(&0.0)
    };

    if !is_on_line() {
        return false;
    }

    // check that point is between end points
    if x1.fuzzy_eq(&x2) {
        // process vertical line
        let o1 = y.fuzzy_cmp(&y1);
        let o2 = y.fuzzy_cmp(&y2);
        if    (o1 != Ordering::Less    && o2 != Ordering::Greater)
           || (o1 != Ordering::Greater && o2 != Ordering::Less) {
            return true;
        }
    } else {
        let a = (y1 - y2) / (x1 - x2);
        let b = ((y1 + y2) - a * (x1 + x2)) / 2.0;
        let c = a * x + b;

        if y.fuzzy_eq(&c) {
            if    ( x.fuzzy_cmp(&x1) == Ordering::Greater && x.fuzzy_cmp(&x2) == Ordering::Less)
               || (x2.fuzzy_cmp(&x1) == Ordering::Greater && x.fuzzy_cmp(&x1) == Ordering::Less) {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use svgdom::{FromStream};
    use svgdom::types::path::{Path};

    macro_rules! test {
        ($name:ident, $in_text:expr, $out_text:expr) => (
            #[test]
            fn $name() {
                let mut path = Path::from_data($in_text).unwrap();
                path.conv_to_absolute();
                convert_segments(&mut path);
                assert_eq_text!(path.to_string(), $out_text);
            }
        )
    }

    test!(conv_l, b"M 10 10 L 15 10 L 15 15",
                   "M 10 10 H 15 V 15");

    test!(conv_cs_1, b"M 10 20 C 10 10 25 10 25 20 C 25 30 40 30 40 20",
                      "M 10 20 C 10 10 25 10 25 20 S 40 30 40 20");

    test!(conv_cs_2, b"M 10 10 C 10 10 10 20 30 40 C 20 35 40 50 60 70 C 80 90 10 20 30 40",
                      "M 10 10 S 10 20 30 40 C 20 35 40 50 60 70 S 10 20 30 40");

    // convert CurveTo to VerticalLineTo when control points are on the same vertical line
    test!(conv_cv_1, b"M 10 10 C 10 15 10 20 10 40",
                      "M 10 10 V 40");

    // ignore converting, because Y1 is outsize the curve
    test!(conv_cv_2, b"M 10 10 C 10 5 10 20 10 40",
                      "M 10 10 C 10 5 10 20 10 40");

    // convert CurveTo to VerticalLineTo when control points
    // are at the start and at the end of the curve
    test!(conv_cv_3, b"M 10 10 C 10 10 10 40 10 40",
                      "M 10 10 V 40");

    // same for H
    test!(conv_ch_1, b"M 10 10 C 15 10 25 10 40 10",
                      "M 10 10 H 40");

    test!(conv_ch_2, b"M 10 10 C 5 10 50 10 40 10",
                      "M 10 10 C 5 10 50 10 40 10");

    test!(conv_ch_3, b"M 10 10 C 10 10 40 10 40 10",
                      "M 10 10 H 40");

    test!(conv_cl_1, b"M 10 118 C 45 83 85 43 120 8",
                      "M 10 118 L 120 8");

    test!(conv_cl_2, b"M 10,15 C 10,15 72.5,10 72.5,55 C 72.5,100 135,100 135,55 L 10,55",
                      "M 10 15 S 72.5 10 72.5 55 S 135 100 135 55 H 10");
}
