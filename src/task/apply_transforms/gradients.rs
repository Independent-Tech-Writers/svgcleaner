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

use task::short::{EId, AId};
use super::utils;

use svgdom::Document;

pub fn apply_transform_to_gradients(doc: &Document) {
    let iter = doc.descendants().svg()
                  .filter(|n| ::task::is_gradient(n))
                  .filter(|n| n.has_attribute(AId::GradientTransform));

    for node in iter {
        {
            let flag = node.linked_nodes().any(|n| ::task::is_gradient(&n));

            if flag {
                // We can apply transform only to gradients that doesn't used by
                // other gradients, because this gradients may inherit new values, which will
                // break rendering.

                // TODO: check that linked gradients actually inherit something
                continue;
            }
        }

        if node.has_attribute(AId::XlinkHref) {
            // We can apply a transform to gradients which linked to other gradients
            // only when linked gradient doesn't have a transform.
            // After applying the transform - we will remove it, but if linked gradient
            // has a transform - it will be inherited. So we will get double transform.
            // Which will lead to an error.
            let link = node.attribute_value(AId::XlinkHref).unwrap().as_link().unwrap().clone();
            if link.has_attribute(AId::GradientTransform) {
                continue;
            }
        }

        let ts = *node.attribute_value(AId::GradientTransform).unwrap().as_transform().unwrap();

        if !utils::is_valid_transform(&ts) {
            continue;
        }

        if !utils::is_valid_coords(&node) {
            continue;
        }

        if node.is_tag_name(EId::LinearGradient) {
            let mut attrs = node.attributes_mut();
            utils::transform_coords(&mut attrs, AId::X1, AId::Y1, &ts);
            utils::transform_coords(&mut attrs, AId::X2, AId::Y2, &ts);
        } else {
            let mut attrs = node.attributes_mut();
            utils::transform_coords(&mut attrs, AId::Cx, AId::Cy, &ts);
            utils::transform_coords(&mut attrs, AId::Fx, AId::Fy, &ts);

            if ts.has_scale() {
                let (sx, _) = ts.get_scale();
                utils::scale_coord(&mut attrs, AId::R, &sx);
            }
        }

        node.remove_attribute(AId::GradientTransform);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use svgdom::{Document, WriteToString};
    use task::utils;

    macro_rules! test {
        ($name:ident, $in_text:expr, $out_text:expr) => (
            #[test]
            fn $name() {
                let doc = Document::from_data($in_text).unwrap();
                utils::resolve_gradient_attributes(&doc).unwrap();
                apply_transform_to_gradients(&doc);
                assert_eq_text!(doc.to_string_with_opt(&write_opt_for_tests!()), $out_text);
            }
        )
    }

    macro_rules! test_eq {
        ($name:ident, $in_text:expr) => (
            test!($name, $in_text, String::from_utf8_lossy($in_text));
        )
    }

    test!(apply_1,
b"<svg>
    <linearGradient x1='10' x2='10' y1='10' y2='10' gradientTransform='translate(10 20)'/>
</svg>",
"<svg>
    <linearGradient x1='20' x2='20' y1='30' y2='30'/>
</svg>
");

    test!(apply_2,
b"<svg>
    <radialGradient cx='10' cy='10' fx='10' fy='10' r='5' gradientTransform='matrix(2 0 0 2 10 20)'/>
</svg>",
"<svg>
    <radialGradient cx='30' cy='40' fx='30' fy='40' r='10'/>
</svg>
");

    test_eq!(keep_1,
b"<svg>
    <linearGradient gradientTransform='translate(10 20)'/>
</svg>
");

    // skewX(3)
    test_eq!(keep_2,
b"<svg>
    <linearGradient gradientTransform='matrix(1 0 0.05240778 1 0 0)' x1='10' x2='10' y1='10' y2='10'/>
</svg>
");

    test_eq!(keep_3,
b"<svg>
    <linearGradient gradientTransform='scale(1.5 2)' x1='10' x2='10' y1='10' y2='10'/>
</svg>
");

    test_eq!(keep_4,
b"<svg>
    <linearGradient id='lg1' gradientTransform='translate(10 20)' x1='10' x2='10' y1='10' y2='10'/>
    <linearGradient xlink:href='#lg1'/>
</svg>
");

}
