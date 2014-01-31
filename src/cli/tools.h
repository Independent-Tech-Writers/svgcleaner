/****************************************************************************
**
** SVG Cleaner is batch, tunable, crossplatform SVG cleaning program.
** Copyright (C) 2012-2014 Evgeniy Reizner
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

#ifndef TOOLS_H
#define TOOLS_H

#include <QStringList>
#include <QRectF>
#include <QtDebug>

#include "keys.h"

#define QL1S(x) QLatin1String(x)

class Tools
{
public:
    explicit Tools() {}
    enum RoundType { COORDINATE, TRANSFORM, ATTRIBUTE };
    static bool isZero(qreal value);
    static bool isZeroTs(qreal value);
    static QString convertUnitsToPx(const QString &text, qreal baseValue = 0);
    static QString roundNumber(qreal value, RoundType type = COORDINATE);
    static QString roundNumber(qreal value, int precision);
    static QString trimColor(const QString &color);
    static QVariantHash initDefaultStyleHash();
    static StringHash splitStyle(const QString &style);
    static QString removeEdgeSpaces(const QString &str);
    static qreal getNum(const QChar *&str);
    static qreal strToDouble(const QString &str);
    static QString doubleToStr(const qreal value, int precision = 6);

private:
    static bool isDigit(ushort ch);
    static qreal toDouble(const QChar *&str);
    static QString replaceColorName(const QString &color);
    static int numbersBeforePoint(qreal value);
    static int zerosAfterPoint(qreal value);
};

// TODO: maybe add namespace with static string with usual elem/attr names,
// to prevent QString::fromLatin1_helper executing

namespace Props {
static const StringSet fillList = StringSet() << "fill" << "fill-rule" << "fill-opacity";
static const StringSet strokeList = StringSet()
    << "stroke" << "stroke-width" << "stroke-linecap" << "stroke-linejoin" << "stroke-miterlimit"
    << "stroke-dasharray" << "stroke-dashoffset" << "stroke-opacity";

static const StringSet presentationAttributes = StringSet()
    << "alignment-baseline" << "baseline-shift" << "clip-path" << "clip-rule" << "clip"
    << "color-interpolation-filters" << "color-interpolation" << "color-profile"
    << "color-rendering" << "color" << "cursor" << "direction" << "display" << "dominant-baseline"
    << "enable-background" << "fill-opacity" << "fill-rule" << "fill" << "filter" << "flood-color"
    << "flood-opacity" << "font-family" << "font-size-adjust" << "font-size" << "font-stretch"
    << "font-style" << "font-variant" << "font-weight" << "glyph-orientation-horizontal"
    << "glyph-orientation-vertical" << "image-rendering" << "kerning" << "letter-spacing"
    << "lighting-color" << "marker-end" << "marker-mid" << "marker-start" << "mask" << "opacity"
    << "overflow" << "pointer-events" << "shape-rendering" << "stop-color" << "stop-opacity"
    << "stroke-dasharray" << "stroke-dashoffset" << "stroke-linecap" << "stroke-linejoin"
    << "stroke-miterlimit" << "stroke-opacity" << "stroke-width" << "stroke" << "text-anchor"
    << "text-decoration" << "text-rendering" << "unicode-bidi" << "visibility" << "word-spacing"
    << "writing-mode";

static const CharList linkableStyleAttributes = CharList()
    << "clip-path" << "fill" << "mask" << "filter" << "stroke" << "marker-start"
    << "marker-mid" << "marker-end";

static const QStringList linearGradient = QStringList()
    << "gradientTransform" << "xlink:href" << "x1" << "y1" << "x2" << "y2"
    << "gradientUnits" << "spreadMethod" << "externalResourcesRequired";

static const QStringList radialGradient = QStringList()
    << "gradientTransform" << "xlink:href" << "cx" << "cy" << "r" << "fx" << "fy"
    << "gradientUnits" << "spreadMethod" << "externalResourcesRequired";

static const QStringList filter = QStringList()
    << "gradientTransform" << "xlink:href" << "x" << "y" << "width" << "height" << "filterRes"
    << "filterUnits" << "primitiveUnits" << "externalResourcesRequired";

static const StringSet maskAttributes = StringSet()
    << "x" << "y" << "width" << "height"
    << "maskUnits" << "maskContentUnits" << "externalResourcesRequired";

static const StringSet digitList = StringSet()
    << "x" << "y" << "x1" << "y1" << "x2" << "y2" << "width" << "height" << "r" << "rx" << "ry"
    << "fx" << "fy" << "cx" << "cy" << "dx" << "dy" << "offset";

static const StringSet filterDigitList = StringSet()
    << "stdDeviation" << "baseFrequency" << "k" << "k1" << "k2" << "k3" << "specularConstant"
    << "dx" << "dy" << "stroke-dasharray";

static const StringSet defsList = StringSet()
    << "altGlyphDef" << "clipPath" << "cursor" << "filter" << "linearGradient"
    << "marker" << "mask" << "pattern" << "radialGradient"/* << "symbol"*/;

static const StringSet referencedElements = StringSet()
    << "a" << "altGlyphDef" << "clipPath" << "color-profile" << "cursor" << "filter" << "font"
    << "font-face" << "foreignObject" << "image" << "marker" << "mask" << "pattern" << "script"
    << "style" << "switch" << "text" << "view";

static const StringSet textElements = StringSet()
    << "text" << "tspan" << "flowRoot" << "flowPara" << "flowSpan" << "textPath";

static const StringSet textAttributes = StringSet()
    << "font-style" << "font-variant" << "font-weight" << "font-weight" << "font-stretch"
    << "font-size" << "font-size-adjust" << "kerning" << "letter-spacing" << "word-spacing"
    << "text-decoration" << "writing-mode" << "glyph-orientation-vertical"
    << "glyph-orientation-horizontal" << "direction" << "text-anchor" << "dominant-baseline"
    << "alignment-baseline" << "baseline-shift";

static const QVariantHash defaultStyleValues = Tools::initDefaultStyleHash();

static const StringSet svgElementList = StringSet()
    << "a" << "altGlyph" << "altGlyphDef" << "altGlyphItem" << "animate" << "animateColor"
    << "animateMotion" << "animateTransform" << "circle" << "clipPath" << "color-profile"
    << "cursor" << "defs" << "desc" << "ellipse" << "feBlend" << "feColorMatrix"
    << "feComponentTransfer" << "feComposite" << "feConvolveMatrix" << "feDiffuseLighting"
    << "feDisplacementMap" << "feDistantLight" << "feFlood" << "feFuncA" << "feFuncB" << "feFuncG"
    << "feFuncR" << "feGaussianBlur" << "feImage" << "feMerge" << "feMergeNode" << "feMorphology"
    << "feOffset" << "fePointLight" << "feSpecularLighting" << "feSpotLight" << "feTile"
    << "feTurbulence" << "filter" << "font" << "font-face" << "font-face-format" << "font-face-name"
    << "font-face-src" << "font-face-uri" << "foreignObject" << "g" << "glyph" << "glyphRef"
    << "hkern" << "image" << "line" << "linearGradient" << "marker" << "mask" << "metadata"
    << "missing-glyph" << "mpath" << "path" << "pattern" << "polygon" << "polyline"
    << "radialGradient" << "rect" << "script" << "set" << "stop" << "style" << "svg" << "switch"
    << "symbol" << "text" << "textPath" << "title" << "tref" << "flowRoot" << "flowRegion"
    << "flowPara" << "flowSpan" << "tspan" << "use" << "view" << "vkern";

static const StringSet elementsUsingXLink = StringSet()
    << "a" << "altGlyph" << "color-profile" << "cursor" << "feImage" << "filter" << "font-face-uri"
    << "glyphRef" << "image" << "linearGradient" << "mpath" << "pattern" << "radialGradient"
    << "script" << "textPath" << "use" << "animate" << "animateColor" << "animateMotion"
    << "animateTransform" << "set" << "tref";

static const StringSet containers = StringSet()
    << "a" << "defs" << "glyph" << "g" << "marker" /*<< "mask"*/ << "missing-glyph" /*<< "pattern"*/
    << "svg" << "switch" <<  "symbol";

static const StringSet stopAttributes = StringSet()
    << "offset" << "stop-color" << "stop-opacity";

static const StringSet lengthTypes = StringSet()
    << "em" << "ex" << "px" << "in" << "cm" << "mm" << "pt" << "pc";
}

namespace CleanerAttr {
    static const char * const UsedElement = "used-element";
    static const char * const BoundingBox = "bbox";
    static const char * const BBoxTransform = "bbox-transform";
}

#endif // TOOLS_H
