// Copyright © 2015, Connor Hilarides
// Licensed under the MIT License <LICENSE.md>
//! DirectX Typography Services public API definitions.
use ctypes::{c_void};
use shared::basetsd::{INT32, INT16, UINT64, UINT32, UINT16, UINT8};
use shared::minwindef::{FLOAT, BOOL, BYTE, FILETIME};
use shared::windef::{HMONITOR, RECT, HDC, COLORREF, SIZE};
use shared::winerror::{SEVERITY_ERROR};
use um::d2d1::{ID2D1SimplifiedGeometrySink};
use um::dcommon::{DWRITE_MEASURING_MODE};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::wingdi::{LOGFONTW};
use um::winnt::{HRESULT, WCHAR};

ENUM!{enum DWRITE_FONT_FILE_TYPE {
    DWRITE_FONT_FILE_TYPE_UNKNOWN,
    DWRITE_FONT_FILE_TYPE_CFF,
    DWRITE_FONT_FILE_TYPE_TRUETYPE,
    DWRITE_FONT_FILE_TYPE_OPENTYPE_COLLECTION,
    DWRITE_FONT_FILE_TYPE_TYPE1_PFM,
    DWRITE_FONT_FILE_TYPE_TYPE1_PFB,
    DWRITE_FONT_FILE_TYPE_VECTOR,
    DWRITE_FONT_FILE_TYPE_BITMAP,
    DWRITE_FONT_FILE_TYPE_TRUETYPE_COLLECTION = DWRITE_FONT_FILE_TYPE_OPENTYPE_COLLECTION,
}}
ENUM!{enum DWRITE_FONT_FACE_TYPE {
    DWRITE_FONT_FACE_TYPE_CFF,
    DWRITE_FONT_FACE_TYPE_TRUETYPE,
    DWRITE_FONT_FACE_TYPE_OPENTYPE_COLLECTION,
    DWRITE_FONT_FACE_TYPE_TYPE1,
    DWRITE_FONT_FACE_TYPE_VECTOR,
    DWRITE_FONT_FACE_TYPE_BITMAP,
    DWRITE_FONT_FACE_TYPE_UNKNOWN,
    DWRITE_FONT_FACE_TYPE_RAW_CFF,
    DWRITE_FONT_FACE_TYPE_TRUETYPE_COLLECTION = DWRITE_FONT_FACE_TYPE_OPENTYPE_COLLECTION,
}}
ENUM!{enum DWRITE_FONT_SIMULATIONS {
    DWRITE_FONT_SIMULATIONS_NONE = 0x0000,
    DWRITE_FONT_SIMULATIONS_BOLD = 0x0001,
    DWRITE_FONT_SIMULATIONS_OBLIQUE = 0x0002,
}}
ENUM!{enum DWRITE_FONT_WEIGHT {
    DWRITE_FONT_WEIGHT_THIN = 100,
    DWRITE_FONT_WEIGHT_EXTRA_LIGHT = 200,
    DWRITE_FONT_WEIGHT_ULTRA_LIGHT = 200,
    DWRITE_FONT_WEIGHT_LIGHT = 300,
    DWRITE_FONT_WEIGHT_SEMI_LIGHT = 350,
    DWRITE_FONT_WEIGHT_NORMAL = 400,
    DWRITE_FONT_WEIGHT_REGULAR = 400,
    DWRITE_FONT_WEIGHT_MEDIUM = 500,
    DWRITE_FONT_WEIGHT_DEMI_BOLD = 600,
    DWRITE_FONT_WEIGHT_SEMI_BOLD = 600,
    DWRITE_FONT_WEIGHT_BOLD = 700,
    DWRITE_FONT_WEIGHT_EXTRA_BOLD = 800,
    DWRITE_FONT_WEIGHT_ULTRA_BOLD = 800,
    DWRITE_FONT_WEIGHT_BLACK = 900,
    DWRITE_FONT_WEIGHT_HEAVY = 900,
    DWRITE_FONT_WEIGHT_EXTRA_BLACK = 950,
    DWRITE_FONT_WEIGHT_ULTRA_BLACK = 950,
}}
ENUM!{enum DWRITE_FONT_STRETCH {
    DWRITE_FONT_STRETCH_UNDEFINED = 0,
    DWRITE_FONT_STRETCH_ULTRA_CONDENSED = 1,
    DWRITE_FONT_STRETCH_EXTRA_CONDENSED = 2,
    DWRITE_FONT_STRETCH_CONDENSED = 3,
    DWRITE_FONT_STRETCH_SEMI_CONDENSED = 4,
    DWRITE_FONT_STRETCH_NORMAL = 5,
    DWRITE_FONT_STRETCH_MEDIUM = 5,
    DWRITE_FONT_STRETCH_SEMI_EXPANDED = 6,
    DWRITE_FONT_STRETCH_EXPANDED = 7,
    DWRITE_FONT_STRETCH_EXTRA_EXPANDED = 8,
    DWRITE_FONT_STRETCH_ULTRA_EXPANDED = 9,
}}
ENUM!{enum DWRITE_FONT_STYLE {
    DWRITE_FONT_STYLE_NORMAL,
    DWRITE_FONT_STYLE_OBLIQUE,
    DWRITE_FONT_STYLE_ITALIC,
}}
ENUM!{enum DWRITE_INFORMATIONAL_STRING_ID {
    DWRITE_INFORMATIONAL_STRING_NONE,
    DWRITE_INFORMATIONAL_STRING_COPYRIGHT_NOTICE,
    DWRITE_INFORMATIONAL_STRING_VERSION_STRINGS,
    DWRITE_INFORMATIONAL_STRING_TRADEMARK,
    DWRITE_INFORMATIONAL_STRING_MANUFACTURER,
    DWRITE_INFORMATIONAL_STRING_DESIGNER,
    DWRITE_INFORMATIONAL_STRING_DESIGNER_URL,
    DWRITE_INFORMATIONAL_STRING_DESCRIPTION,
    DWRITE_INFORMATIONAL_STRING_FONT_VENDOR_URL,
    DWRITE_INFORMATIONAL_STRING_LICENSE_DESCRIPTION,
    DWRITE_INFORMATIONAL_STRING_LICENSE_INFO_URL,
    DWRITE_INFORMATIONAL_STRING_WIN32_FAMILY_NAMES,
    DWRITE_INFORMATIONAL_STRING_WIN32_SUBFAMILY_NAMES,
    DWRITE_INFORMATIONAL_STRING_PREFERRED_FAMILY_NAMES,
    DWRITE_INFORMATIONAL_STRING_PREFERRED_SUBFAMILY_NAMES,
    DWRITE_INFORMATIONAL_STRING_SAMPLE_TEXT,
    DWRITE_INFORMATIONAL_STRING_FULL_NAME,
    DWRITE_INFORMATIONAL_STRING_POSTSCRIPT_NAME,
    DWRITE_INFORMATIONAL_STRING_POSTSCRIPT_CID_NAME,
    DWRITE_INFORMATIONAL_STRING_WWS_FAMILY_NAME,
    DWRITE_INFORMATIONAL_STRING_DESIGN_SCRIPT_LANGUAGE_TAG,
    DWRITE_INFORMATIONAL_STRING_SUPPORTED_SCRIPT_LANGUAGE_TAG,
}}
STRUCT!{struct DWRITE_FONT_METRICS {
    designUnitsPerEm: UINT16,
    ascent: UINT16,
    descent: UINT16,
    lineGap: INT16,
    capHeight: UINT16,
    xHeight: UINT16,
    underlinePosition: INT16,
    underlineThickness: UINT16,
    strikethroughPosition: INT16,
    strikethroughThickness: UINT16,
}}
STRUCT!{struct DWRITE_GLYPH_METRICS {
    leftSideBearing: INT32,
    advanceWidth: UINT32,
    rightSideBearing: INT32,
    topSideBearing: INT32,
    advanceHeight: UINT32,
    bottomSideBearing: INT32,
    verticalOriginY: INT32,
}}
STRUCT!{struct DWRITE_GLYPH_OFFSET {
    advanceOffset: FLOAT,
    ascenderOffset: FLOAT,
}}
ENUM!{enum DWRITE_FACTORY_TYPE {
    DWRITE_FACTORY_TYPE_SHARED,
    DWRITE_FACTORY_TYPE_ISOLATED,
}}
RIDL!{#[uuid(0x727cad4e, 0xd6af, 0x4c9e, 0x8a, 0x08, 0xd6, 0x95, 0xb1, 0x1c, 0xaa, 0x49)]
interface IDWriteFontFileLoader(IDWriteFontFileLoaderVtbl): IUnknown(IUnknownVtbl) {
    fn CreateStreamFromKey(
        &self,
        fontFileReferenceKey: *const c_void,
        fontFileReferenceKeySize: UINT32,
        fontFileStream: *mut *mut IDWriteFontFileStream
    ) -> HRESULT
}}

ENUM!{enum DWRITE_OUTLINE_THRESHOLD {
    DWRITE_OUTLINE_THRESHOLD_ANTIALIASED,
    DWRITE_OUTLINE_THRESHOLD_ALIASED,
}}
STRUCT!{struct DWRITE_FONT_METRICS1 {
    designUnitsPerEm: UINT16,
    ascent: UINT16,
    descent: UINT16,
    lineGap: INT16,
    capHeight: UINT16,
    xHeight: UINT16,
    underlinePosition: INT16,
    underlineThickness: UINT16,
    strikethroughPosition: INT16,
    strikethroughThickness: UINT16,
    glyphBoxLeft: INT16,
    glyphBoxTop: INT16,
    glyphBoxRight: INT16,
    glyphBoxBottom: INT16,
    subscriptPositionX: INT16,
    subscriptPositionY: INT16,
    subscriptSizeX: INT16,
    subscriptSizeY: INT16,
    superscriptPositionX: INT16,
    superscriptPositionY: INT16,
    superscriptSizeX: INT16,
    superscriptSizeY: INT16,
    hasTypographicMetrics: BOOL,
}}
STRUCT!{struct DWRITE_UNICODE_RANGE {
    first: UINT32,
    last: UINT32,
}}
STRUCT!{struct DWRITE_CARET_METRICS {
    slopeRise: INT16,
    slopeRun: INT16,
    offset: INT16,
}}
#[inline]
pub fn DWRITE_MAKE_OPENTYPE_TAG(a: u8, b: u8, c: u8, d: u8) -> u32 {
    ((d as u32) << 24) | ((c as u32) << 16) | ((b as u32) << 8) | (a as u32)
}
RIDL!{interface IDWriteLocalFontFileLoader(IDWriteLocalFontFileLoaderVtbl):
        IDWriteFontFileLoader(IDWriteFontFileLoaderVtbl) {
    fn GetFilePathLengthFromKey(
        &self, fontFileReferenceKey: *const c_void, fontFileReferenceKeySize: UINT32,
        filePathLength: *mut UINT32
    ) -> HRESULT,
    fn GetFilePathFromKey(
        &self, fontFileReferenceKey: *const c_void, fontFileReferenceKeySize: UINT32,
        filePath: *mut WCHAR,
        filePathSize: UINT32
    ) -> HRESULT,
    fn GetLastWriteTimeFromKey(
        &self, fontFileReferenceKey: *const c_void, fontFileReferenceKeySize: UINT32,
        lastWriteTime: *mut FILETIME
    ) -> HRESULT
}}
RIDL!{interface IDWriteFontFileStream(IDWriteFontFileStreamVtbl): IUnknown(IUnknownVtbl) {
    fn ReadFileFragment(
        &self, fragmentStart: *mut *const c_void, fileOffset: UINT64,
        fragmentSize: UINT64, fragmentContext: *mut *mut c_void
    ) -> HRESULT,
    fn ReleaseFileFragment(&self, fragmentContext: *mut c_void) -> (),
    fn GetFileSize(&self, fileSize: *mut UINT64) -> HRESULT,
    fn GetLastWriteTime(&self, lastWriteTime: *mut UINT64) -> HRESULT
}}
RIDL!{interface IDWriteFontFile(IDWriteFontFileVtbl): IUnknown(IUnknownVtbl) {
    fn GetReferenceKey(
        &self, fontFileReferenceKey: *mut *const c_void,
        fontFileReferenceKeySize: *mut UINT32
    ) -> HRESULT,
    fn GetLoader(&self, fontFileLoader: *mut *mut IDWriteFontFileLoader) -> HRESULT,
    fn Analyze(
        &self, isSupportedFontType: *mut BOOL, fontFileType: *mut DWRITE_FONT_FILE_TYPE,
        fontFaceType: *mut DWRITE_FONT_FACE_TYPE, numberOfFaces: *mut UINT32
    ) -> HRESULT
}}
ENUM!{enum DWRITE_PIXEL_GEOMETRY {
    DWRITE_PIXEL_GEOMETRY_FLAT,
    DWRITE_PIXEL_GEOMETRY_RGB,
    DWRITE_PIXEL_GEOMETRY_BGR,
}}
ENUM!{enum DWRITE_RENDERING_MODE {
    DWRITE_RENDERING_MODE_DEFAULT,
    DWRITE_RENDERING_MODE_ALIASED,
    DWRITE_RENDERING_MODE_GDI_CLASSIC,
    DWRITE_RENDERING_MODE_GDI_NATURAL,
    DWRITE_RENDERING_MODE_NATURAL,
    DWRITE_RENDERING_MODE_NATURAL_SYMMETRIC,
    DWRITE_RENDERING_MODE_OUTLINE,
    DWRITE_RENDERING_MODE_CLEARTYPE_GDI_CLASSIC = DWRITE_RENDERING_MODE_GDI_CLASSIC,
    DWRITE_RENDERING_MODE_CLEARTYPE_GDI_NATURAL = DWRITE_RENDERING_MODE_GDI_NATURAL,
    DWRITE_RENDERING_MODE_CLEARTYPE_NATURAL = DWRITE_RENDERING_MODE_NATURAL,
    DWRITE_RENDERING_MODE_CLEARTYPE_NATURAL_SYMMETRIC = DWRITE_RENDERING_MODE_NATURAL_SYMMETRIC,
}}
STRUCT!{struct DWRITE_MATRIX {
    m11: FLOAT,
    m12: FLOAT,
    m21: FLOAT,
    m22: FLOAT,
    dx: FLOAT,
    dy: FLOAT,
}}
RIDL!{interface IDWriteRenderingParams(IDWriteRenderingParamsVtbl): IUnknown(IUnknownVtbl) {
    fn GetGamma(&self) -> FLOAT,
    fn GetEnhancedContrast(&self) -> FLOAT,
    fn GetClearTypeLevel(&self) -> FLOAT,
    fn GetPixelGeometry(&self) -> DWRITE_PIXEL_GEOMETRY,
    fn GetRenderingMode(&self) -> DWRITE_RENDERING_MODE
}}
pub type IDWriteGeometrySink = ID2D1SimplifiedGeometrySink;
RIDL!{interface IDWriteFontFace(IDWriteFontFaceVtbl): IUnknown(IUnknownVtbl) {
    fn GetType(&self) -> DWRITE_FONT_FACE_TYPE,
    fn GetFiles(
        &self, numberOfFiles: *mut UINT32, fontFiles: *mut *mut IDWriteFontFile
    ) -> HRESULT,
    fn GetIndex(&self) -> UINT32,
    fn GetSimulations(&self) -> DWRITE_FONT_SIMULATIONS,
    fn IsSymbolFont(&self) -> BOOL,
    fn GetMetrics(&self, fontFaceMetrics: *mut DWRITE_FONT_METRICS) -> (),
    fn GetGlyphCount(&self) -> UINT16,
    fn GetDesignGlyphMetrics(
        &self, glyphIndices: *const UINT16, glyphCount: UINT32,
        glyphMetrics: *mut DWRITE_GLYPH_METRICS, isSideways: BOOL
    ) -> HRESULT,
    fn GetGlyphIndices(
        &self, codePoints: *const UINT32, codePointCount: UINT32,
        glyphIndices: *mut UINT16
    ) -> HRESULT,
    fn TryGetFontTable(
        &self, openTypeTableTag: UINT32, tableData: *mut *const c_void,
        tableSize: *mut UINT32, tableContext: *mut *mut c_void, exists: *mut BOOL
    ) -> HRESULT,
    fn ReleaseFontTable(
        &self, tableContext: *mut c_void
    ) -> HRESULT,
    fn GetGlyphRunOutline(
        &self, emSize: FLOAT, glyphIndices: *const UINT16, glyphAdvances: *const FLOAT,
        glyphOffsets: *const DWRITE_GLYPH_OFFSET, glyphCount: UINT32, isSideways: BOOL,
        isRightToLeft: BOOL, geometrySink: *mut IDWriteGeometrySink
    ) -> HRESULT,
    fn GetRecommendedRenderingMode(
        &self, emSize: FLOAT, pixelsPerDip: FLOAT, measuringMode: DWRITE_MEASURING_MODE,
        renderingParams: *mut IDWriteRenderingParams, renderingMode: *mut DWRITE_RENDERING_MODE
    ) -> HRESULT,
    fn GetGdiCompatibleMetrics(
        &self, emSize: FLOAT, pixelsPerDip: FLOAT, transform: *const DWRITE_MATRIX,
        fontFaceMetrics: *mut DWRITE_FONT_METRICS
    ) -> HRESULT,
    fn GetGdiCompatibleGlyphMetrics(
        &self, enSize: FLOAT, pixelsPerDip: FLOAT, transform: *const DWRITE_MATRIX,
        useGdiNatrual: BOOL, glyphIndices: *const UINT16, glyphCount: UINT32,
        glyphMetrics: *mut DWRITE_GLYPH_METRICS, isSideways: BOOL
    ) -> HRESULT
}}
RIDL!{interface IDWriteFontFace1(IDWriteFontFace1Vtbl): IDWriteFontFace(IDWriteFontFaceVtbl) {
    fn GetMetrics(&self, fontFaceMetrics: *mut DWRITE_FONT_METRICS1) -> (),
    fn GetGdiCompatibleMetrics(
        &self, emSize: FLOAT, pixelsPerDip: FLOAT, transform: *const DWRITE_MATRIX,
        fontFaceMetrics: *mut DWRITE_FONT_METRICS1
    ) -> HRESULT,
    fn GetCaretMetrics(&self, caretMetrics: *mut DWRITE_CARET_METRICS) -> (),
    fn GetUnicodeRanges(&self, maxRangeCount: UINT32, unicodeRanges: *mut DWRITE_UNICODE_RANGE,
                        actualRangeCount: *mut UINT32
    ) -> HRESULT,
    fn IsMonoSpacedFont(&self) -> BOOL,
    fn GetDesignGlyphAdvances(
        &self, glyphCount: UINT32, glyphIndices: *const UINT16,
        glyphAdvances: *mut INT32, isSideways: BOOL
    ) -> HRESULT,
    fn GetGdiCompatibleGlyphAdvance(
        &self, emSize: FLOAT, pixelsPerDip: FLOAT, transform: *const DWRITE_MATRIX,
        useGdiNatural: BOOL, isSideways: BOOL, glyphCount: UINT32,
        glyphIndices: *const UINT16, glyphAdvances: *mut INT32
    ) -> HRESULT,
    fn GetKerningPairAdjustments(
        &self, glyphCount: UINT32, glyphIndices: *const UINT16,
        glyphAdvanceAdjustments: *mut INT32
    ) -> HRESULT,
    fn HasKerningPairs(&self) -> BOOL,
    fn GetRecommendedRenderingMode(
        &self, fontEmSize: FLOAT, dpiX: FLOAT, dpiY: FLOAT,
        transform: *const DWRITE_MATRIX, isSideways: BOOL,
        outlineThreshold: DWRITE_OUTLINE_THRESHOLD,
        measuringMode: DWRITE_MEASURING_MODE,
        renderingMode: *mut DWRITE_RENDERING_MODE
    ) -> HRESULT,
    fn GetVerticalGlyphVariants(
        &self, nominalGlyphIndices: *const UINT16, verticalGlyphIndices: *mut UINT16
    ) -> HRESULT,
    fn HasVerticalGlyphVariants(&self) -> BOOL
}}
RIDL!{interface IDWriteFontCollectionLoader(IDWriteFontCollectionLoaderVtbl):
        IUnknown(IUnknownVtbl) {
    fn CreateEnumeratorFromKey(
        &self, factory: *mut IDWriteFactory, collectionKey: *const c_void,
        collectionKeySize: UINT32, fontFileEnumerator: *mut *mut IDWriteFontFileEnumerator
    ) -> HRESULT
}}
RIDL!{interface IDWriteFontFileEnumerator(IDWriteFontFileEnumeratorVtbl): IUnknown(IUnknownVtbl) {
    fn MoveNext(&self, hasCurrentFile: *mut BOOL) -> HRESULT,
    fn GetCurrentFontFile(&self, fontFile: *mut *mut IDWriteFontFile) -> HRESULT
}}
RIDL!{interface IDWriteLocalizedStrings(IDWriteLocalizedStringsVtbl): IUnknown(IUnknownVtbl) {
    fn GetCount(&self) -> UINT32,
    fn FindLocaleName(
        &self, localeName: *const WCHAR, index: *mut UINT32, exists: *mut BOOL
    ) -> HRESULT,
    fn GetLocaleNameLength(&self, index: UINT32, length: *mut UINT32) -> HRESULT,
    fn GetLocaleName(
        &self, index: UINT32, localeName: *mut WCHAR, size: UINT32
    ) -> HRESULT,
    fn GetStringLength(&self, index: UINT32, length: *mut UINT32) -> HRESULT,
    fn GetString(
        &self, index: UINT32, stringBuffer: *mut WCHAR, size: UINT32
    ) -> HRESULT
}}
RIDL!{interface IDWriteFontCollection(IDWriteFontCollectionVtbl): IUnknown(IUnknownVtbl) {
    fn GetFontFamilyCount(&self) -> UINT32,
    fn GetFontFamily(
        &self, index: UINT32, fontFamily: *mut *mut IDWriteFontFamily
    ) -> HRESULT,
    fn FindFamilyName(
        &self, familyName: *const WCHAR, index: *mut UINT32, exists: *mut BOOL
    ) -> HRESULT,
    fn GetFontFromFontFace(
        &self, fontFace: *mut IDWriteFontFace, font: *mut *mut IDWriteFont
    ) -> HRESULT
}}
RIDL!{interface IDWriteFontList(IDWriteFontListVtbl): IUnknown(IUnknownVtbl) {
    fn GetFontCollection(&self, fontCollection: *mut *mut IDWriteFontCollection) -> HRESULT,
    fn GetFontCount(&self) -> UINT32,
    fn GetFont(&self, index: UINT32, font: *mut *mut IDWriteFont) -> HRESULT
}}
RIDL!{interface IDWriteFontFamily(IDWriteFontFamilyVtbl): IDWriteFontList(IDWriteFontListVtbl) {
    fn GetFamilyNames(&self, names: *mut *mut IDWriteLocalizedStrings) -> HRESULT,
    fn GetFirstMatchingFont(
        &self, weight: DWRITE_FONT_WEIGHT, stretch: DWRITE_FONT_STRETCH,
        style: DWRITE_FONT_STYLE, matchingFont: *mut *mut IDWriteFont
    ) -> HRESULT,
    fn GetMatchingFonts(
        &self, weight: DWRITE_FONT_WEIGHT, stretch: DWRITE_FONT_STRETCH,
        style: DWRITE_FONT_STYLE, matchingFonts: *mut *mut IDWriteFontList
    ) -> HRESULT
}}
RIDL!{interface IDWriteFont(IDWriteFontVtbl): IUnknown(IUnknownVtbl) {
    fn GetFontFamily(&self, fontFamily: *mut *mut IDWriteFontFamily) -> HRESULT,
    fn GetWeight(&self) -> DWRITE_FONT_WEIGHT,
    fn GetStretch(&self) -> DWRITE_FONT_STRETCH,
    fn GetStyle(&self) -> DWRITE_FONT_STYLE,
    fn IsSymbolFont(&self) -> BOOL,
    fn GetFaceNames(&self, names: *mut *mut IDWriteLocalizedStrings) -> HRESULT,
    fn GetInformationalStrings(
        &self, informationalStringId: DWRITE_INFORMATIONAL_STRING_ID,
        informationalStrings: *mut *mut IDWriteLocalizedStrings, exists: *mut BOOL
    ) -> HRESULT,
    fn GetSimulations(&self) -> DWRITE_FONT_SIMULATIONS,
    fn GetMetrics(&self, fontMetrics: *mut DWRITE_FONT_METRICS) -> (),
    fn HasCharacter(&self, unicodeValue: UINT32, exists: *mut BOOL) -> HRESULT,
    fn CreateFontFace(&self, fontFace: *mut *mut IDWriteFontFace) -> HRESULT
}}
ENUM!{enum DWRITE_READING_DIRECTION {
    DWRITE_READING_DIRECTION_LEFT_TO_RIGHT = 0,
    DWRITE_READING_DIRECTION_RIGHT_TO_LEFT = 1,
    DWRITE_READING_DIRECTION_TOP_TO_BOTTOM = 2,
    DWRITE_READING_DIRECTION_BOTTOM_TO_TOP = 3,
}}
ENUM!{enum DWRITE_FLOW_DIRECTION {
    DWRITE_FLOW_DIRECTION_TOP_TO_BOTTOM = 0,
    DWRITE_FLOW_DIRECTION_BOTTOM_TO_TOP = 1,
    DWRITE_FLOW_DIRECTION_LEFT_TO_RIGHT = 2,
    DWRITE_FLOW_DIRECTION_RIGHT_TO_LEFT = 3,
}}
ENUM!{enum DWRITE_TEXT_ALIGNMENT {
    DWRITE_TEXT_ALIGNMENT_LEADING,
    DWRITE_TEXT_ALIGNMENT_TRAILING,
    DWRITE_TEXT_ALIGNMENT_CENTER,
    DWRITE_TEXT_ALIGNMENT_JUSTIFIED,
}}
ENUM!{enum DWRITE_PARAGRAPH_ALIGNMENT {
    DWRITE_PARAGRAPH_ALIGNMENT_NEAR,
    DWRITE_PARAGRAPH_ALIGNMENT_FAR,
    DWRITE_PARAGRAPH_ALIGNMENT_CENTER,
}}
ENUM!{enum DWRITE_WORD_WRAPPING {
    DWRITE_WORD_WRAPPING_WRAP = 0,
    DWRITE_WORD_WRAPPING_NO_WRAP = 1,
    DWRITE_WORD_WRAPPING_EMERGENCY_BREAK = 2,
    DWRITE_WORD_WRAPPING_WHOLE_WORD = 3,
    DWRITE_WORD_WRAPPING_CHARACTER = 4,
}}
ENUM!{enum DWRITE_LINE_SPACING_METHOD {
    DWRITE_LINE_SPACING_METHOD_DEFAULT,
    DWRITE_LINE_SPACING_METHOD_UNIFORM,
    DWRITE_LINE_SPACING_METHOD_PROPORTIONAL,
}}
ENUM!{enum DWRITE_TRIMMING_GRANULARITY {
    DWRITE_TRIMMING_GRANULARITY_NONE,
    DWRITE_TRIMMING_GRANULARITY_CHARACTER,
    DWRITE_TRIMMING_GRANULARITY_WORD,
}}
ENUM!{enum DWRITE_FONT_FEATURE_TAG {
    DWRITE_FONT_FEATURE_TAG_ALTERNATIVE_FRACTIONS = 0x63726661, // 'afrc'
    DWRITE_FONT_FEATURE_TAG_PETITE_CAPITALS_FROM_CAPITALS = 0x63703263, // 'c2pc'
    DWRITE_FONT_FEATURE_TAG_SMALL_CAPITALS_FROM_CAPITALS = 0x63733263, // 'c2sc'
    DWRITE_FONT_FEATURE_TAG_CONTEXTUAL_ALTERNATES = 0x746c6163, // 'calt'
    DWRITE_FONT_FEATURE_TAG_CASE_SENSITIVE_FORMS = 0x65736163, // 'case'
    DWRITE_FONT_FEATURE_TAG_GLYPH_COMPOSITION_DECOMPOSITION = 0x706d6363, // 'ccmp'
    DWRITE_FONT_FEATURE_TAG_CONTEXTUAL_LIGATURES = 0x67696c63, // 'clig'
    DWRITE_FONT_FEATURE_TAG_CAPITAL_SPACING = 0x70737063, // 'cpsp'
    DWRITE_FONT_FEATURE_TAG_CONTEXTUAL_SWASH = 0x68777363, // 'cswh'
    DWRITE_FONT_FEATURE_TAG_CURSIVE_POSITIONING = 0x73727563, // 'curs'
    DWRITE_FONT_FEATURE_TAG_DEFAULT = 0x746c6664, // 'dflt'
    DWRITE_FONT_FEATURE_TAG_DISCRETIONARY_LIGATURES = 0x67696c64, // 'dlig'
    DWRITE_FONT_FEATURE_TAG_EXPERT_FORMS = 0x74707865, // 'expt'
    DWRITE_FONT_FEATURE_TAG_FRACTIONS = 0x63617266, // 'frac'
    DWRITE_FONT_FEATURE_TAG_FULL_WIDTH = 0x64697766, // 'fwid'
    DWRITE_FONT_FEATURE_TAG_HALF_FORMS = 0x666c6168, // 'half'
    DWRITE_FONT_FEATURE_TAG_HALANT_FORMS = 0x6e6c6168, // 'haln'
    DWRITE_FONT_FEATURE_TAG_ALTERNATE_HALF_WIDTH = 0x746c6168, // 'halt'
    DWRITE_FONT_FEATURE_TAG_HISTORICAL_FORMS = 0x74736968, // 'hist'
    DWRITE_FONT_FEATURE_TAG_HORIZONTAL_KANA_ALTERNATES = 0x616e6b68, // 'hkna'
    DWRITE_FONT_FEATURE_TAG_HISTORICAL_LIGATURES = 0x67696c68, // 'hlig'
    DWRITE_FONT_FEATURE_TAG_HALF_WIDTH = 0x64697768, // 'hwid'
    DWRITE_FONT_FEATURE_TAG_HOJO_KANJI_FORMS = 0x6f6a6f68, // 'hojo'
    DWRITE_FONT_FEATURE_TAG_JIS04_FORMS = 0x3430706a, // 'jp04'
    DWRITE_FONT_FEATURE_TAG_JIS78_FORMS = 0x3837706a, // 'jp78'
    DWRITE_FONT_FEATURE_TAG_JIS83_FORMS = 0x3338706a, // 'jp83'
    DWRITE_FONT_FEATURE_TAG_JIS90_FORMS = 0x3039706a, // 'jp90'
    DWRITE_FONT_FEATURE_TAG_KERNING = 0x6e72656b, // 'kern'
    DWRITE_FONT_FEATURE_TAG_STANDARD_LIGATURES = 0x6167696c, // 'liga'
    DWRITE_FONT_FEATURE_TAG_LINING_FIGURES = 0x6d756e6c, // 'lnum'
    DWRITE_FONT_FEATURE_TAG_LOCALIZED_FORMS = 0x6c636f6c, // 'locl'
    DWRITE_FONT_FEATURE_TAG_MARK_POSITIONING = 0x6b72616d, // 'mark'
    DWRITE_FONT_FEATURE_TAG_MATHEMATICAL_GREEK = 0x6b72676d, // 'mgrk'
    DWRITE_FONT_FEATURE_TAG_MARK_TO_MARK_POSITIONING = 0x6b6d6b6d, // 'mkmk'
    DWRITE_FONT_FEATURE_TAG_ALTERNATE_ANNOTATION_FORMS = 0x746c616e, // 'nalt'
    DWRITE_FONT_FEATURE_TAG_NLC_KANJI_FORMS = 0x6b636c6e, // 'nlck'
    DWRITE_FONT_FEATURE_TAG_OLD_STYLE_FIGURES = 0x6d756e6f, // 'onum'
    DWRITE_FONT_FEATURE_TAG_ORDINALS = 0x6e64726f, // 'ordn'
    DWRITE_FONT_FEATURE_TAG_PROPORTIONAL_ALTERNATE_WIDTH = 0x746c6170, // 'palt'
    DWRITE_FONT_FEATURE_TAG_PETITE_CAPITALS = 0x70616370, // 'pcap'
    DWRITE_FONT_FEATURE_TAG_PROPORTIONAL_FIGURES = 0x6d756e70, // 'pnum'
    DWRITE_FONT_FEATURE_TAG_PROPORTIONAL_WIDTHS = 0x64697770, // 'pwid'
    DWRITE_FONT_FEATURE_TAG_QUARTER_WIDTHS = 0x64697771, // 'qwid'
    DWRITE_FONT_FEATURE_TAG_REQUIRED_LIGATURES = 0x67696c72, // 'rlig'
    DWRITE_FONT_FEATURE_TAG_RUBY_NOTATION_FORMS = 0x79627572, // 'ruby'
    DWRITE_FONT_FEATURE_TAG_STYLISTIC_ALTERNATES = 0x746c6173, // 'salt'
    DWRITE_FONT_FEATURE_TAG_SCIENTIFIC_INFERIORS = 0x666e6973, // 'sinf'
    DWRITE_FONT_FEATURE_TAG_SMALL_CAPITALS = 0x70636d73, // 'smcp'
    DWRITE_FONT_FEATURE_TAG_SIMPLIFIED_FORMS = 0x6c706d73, // 'smpl'
    DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_1 = 0x31307373, // 'ss01'
    DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_2 = 0x32307373, // 'ss02'
    DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_3 = 0x33307373, // 'ss03'
    DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_4 = 0x34307373, // 'ss04'
    DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_5 = 0x35307373, // 'ss05'
    DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_6 = 0x36307373, // 'ss06'
    DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_7 = 0x37307373, // 'ss07'
    DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_8 = 0x38307373, // 'ss08'
    DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_9 = 0x39307373, // 'ss09'
    DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_10 = 0x30317373, // 'ss10'
    DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_11 = 0x31317373, // 'ss11'
    DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_12 = 0x32317373, // 'ss12'
    DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_13 = 0x33317373, // 'ss13'
    DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_14 = 0x34317373, // 'ss14'
    DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_15 = 0x35317373, // 'ss15'
    DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_16 = 0x36317373, // 'ss16'
    DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_17 = 0x37317373, // 'ss17'
    DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_18 = 0x38317373, // 'ss18'
    DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_19 = 0x39317373, // 'ss19'
    DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_20 = 0x30327373, // 'ss20'
    DWRITE_FONT_FEATURE_TAG_SUBSCRIPT = 0x73627573, // 'subs'
    DWRITE_FONT_FEATURE_TAG_SUPERSCRIPT = 0x73707573, // 'sups'
    DWRITE_FONT_FEATURE_TAG_SWASH = 0x68737773, // 'swsh'
    DWRITE_FONT_FEATURE_TAG_TITLING = 0x6c746974, // 'titl'
    DWRITE_FONT_FEATURE_TAG_TRADITIONAL_NAME_FORMS = 0x6d616e74, // 'tnam'
    DWRITE_FONT_FEATURE_TAG_TABULAR_FIGURES = 0x6d756e74, // 'tnum'
    DWRITE_FONT_FEATURE_TAG_TRADITIONAL_FORMS = 0x64617274, // 'trad'
    DWRITE_FONT_FEATURE_TAG_THIRD_WIDTHS = 0x64697774, // 'twid'
    DWRITE_FONT_FEATURE_TAG_UNICASE = 0x63696e75, // 'unic'
    DWRITE_FONT_FEATURE_TAG_VERTICAL_WRITING = 0x74726576, // 'vert'
    DWRITE_FONT_FEATURE_TAG_VERTICAL_ALTERNATES_AND_ROTATION = 0x32747276, // 'vrt2'
    DWRITE_FONT_FEATURE_TAG_SLASHED_ZERO = 0x6f72657a, // 'zero'
}}
STRUCT!{struct DWRITE_TEXT_RANGE {
    startPosition: UINT32,
    length: UINT32,
}}
STRUCT!{struct DWRITE_FONT_FEATURE {
    nameTag: DWRITE_FONT_FEATURE_TAG,
    parameter: UINT32,
}}
STRUCT!{struct DWRITE_TYPOGRAPHIC_FEATURES {
    features: *mut DWRITE_FONT_FEATURE,
    featureCount: UINT32,
}}
STRUCT!{struct DWRITE_TRIMMING {
    granularity: DWRITE_TRIMMING_GRANULARITY,
    delimiter: UINT32,
    delimiterCount: UINT32,
}}
RIDL!{interface IDWriteTextFormat(IDWriteTextFormatVtbl): IUnknown(IUnknownVtbl) {
    fn SetTextAlignment(&self, textAlignment: DWRITE_TEXT_ALIGNMENT) -> HRESULT,
    fn SetParagraphAlignment(
        &self, paragraphAlignment: DWRITE_PARAGRAPH_ALIGNMENT
    ) -> HRESULT,
    fn SetWordWrapping(&self, wordWrapping: DWRITE_WORD_WRAPPING) -> HRESULT,
    fn SetReadingDirection(&self, readingDirection: DWRITE_READING_DIRECTION) -> HRESULT,
    fn SetFlowDirection(&self, flowDirection: DWRITE_FLOW_DIRECTION) -> HRESULT,
    fn SetIncrementalTabStop(&self, incrementalTabStop: FLOAT) -> HRESULT,
    fn SetTrimming(
        &self, trimmingOptions: *const DWRITE_TRIMMING, trimmingSign: *mut IDWriteInlineObject
    ) -> HRESULT,
    fn SetLineSpacing(
        &self, lineSpacingMethod: DWRITE_LINE_SPACING_METHOD, lineSpacing: FLOAT,
        baseLine: FLOAT
    ) -> HRESULT,
    fn GetTextAlignment(&self) -> DWRITE_TEXT_ALIGNMENT,
    fn GetParagraphAlignment(&self) -> DWRITE_PARAGRAPH_ALIGNMENT,
    fn GetWordWrapping(&self) -> DWRITE_WORD_WRAPPING,
    fn GetReadingDirection(&self) -> DWRITE_READING_DIRECTION,
    fn GetFlowDirection(&self) -> DWRITE_FLOW_DIRECTION,
    fn GetIncrementalTabStop(&self) -> FLOAT,
    fn GetTrimming(
        &self, trimmingOptions: *mut DWRITE_TRIMMING,
        trimmingSign: *mut *mut IDWriteInlineObject
    ) -> HRESULT,
    fn GetLineSpacing(
        &self, lineSpacingMethod: *mut DWRITE_LINE_SPACING_METHOD, lineSpacing: *mut FLOAT,
        baseline: *mut FLOAT
    ) -> HRESULT,
    fn GetFontCollection(&self, fontCollection: *mut *mut IDWriteFontCollection) -> HRESULT,
    fn GetFontFamilyNameLength(&self) -> UINT32,
    fn GetFontFamilyName(&self, fontFamilyName: *mut WCHAR, nameSize: UINT32) -> HRESULT,
    fn GetFontWeight(&self) -> DWRITE_FONT_WEIGHT,
    fn GetFontStyle(&self) -> DWRITE_FONT_STYLE,
    fn GetFontStretch(&self) -> DWRITE_FONT_STRETCH,
    fn GetFontSize(&self) -> FLOAT,
    fn GetLocaleNameLength(&self) -> UINT32,
    fn GetLocaleName(&self, localeName: *mut WCHAR, nameSize: UINT32) -> HRESULT
}}
RIDL!{interface IDWriteTypography(IDWriteTypographyVtbl): IUnknown(IUnknownVtbl) {
    fn AddFontFeature(&self, fontFeature: DWRITE_FONT_FEATURE) -> HRESULT,
    fn GetFontFeatureCount(&self) -> UINT32,
    fn GetFontFeature(
        &self, fontFeatureIndex: UINT32, fontFeature: *mut DWRITE_FONT_FEATURE
    ) -> HRESULT
}}
ENUM!{enum DWRITE_SCRIPT_SHAPES {
    DWRITE_SCRIPT_SHAPES_DEFAULT = 0,
    DWRITE_SCRIPT_SHAPES_NO_VISUAL = 1,
}}
STRUCT!{struct DWRITE_SCRIPT_ANALYSIS {
    script: UINT16,
    shapes: DWRITE_SCRIPT_SHAPES,
}}
ENUM!{enum DWRITE_BREAK_CONDITION {
    DWRITE_BREAK_CONDITION_NEUTRAL,
    DWRITE_BREAK_CONDITION_CAN_BREAK,
    DWRITE_BREAK_CONDITION_MAY_NOT_BREAK,
    DWRITE_BREAK_CONDITION_MUST_BREAK,
}}
STRUCT!{struct DWRITE_LINE_BREAKPOINT {
    bit_fields: UINT8,
}}
BITFIELD!{DWRITE_LINE_BREAKPOINT bit_fields: UINT8 [
    breakConditionBefore set_breakConditionBefore[0..2],
    breakConditionAfter set_breakConditionAfter[2..4],
    isWhitespace set_isWhitespace[4..5],
    isSoftHyphen set_isSoftHyphen[5..6],
    padding set_padding[6..8],
]}
ENUM!{enum DWRITE_NUMBER_SUBSTITUTION_METHOD {
    DWRITE_NUMBER_SUBSTITUTION_METHOD_FROM_CULTURE,
    DWRITE_NUMBER_SUBSTITUTION_METHOD_CONTEXTUAL,
    DWRITE_NUMBER_SUBSTITUTION_METHOD_NONE,
    DWRITE_NUMBER_SUBSTITUTION_METHOD_NATIONAL,
    DWRITE_NUMBER_SUBSTITUTION_METHOD_TRADITIONAL,
}}
RIDL!{interface IDWriteNumberSubstitution(IDWriteNumberSubstitutionVtbl): IUnknown(IUnknownVtbl) {
}}
STRUCT!{struct DWRITE_SHAPING_TEXT_PROPERTIES {
    bit_fields: UINT16,
}}
BITFIELD!{DWRITE_SHAPING_TEXT_PROPERTIES bit_fields: UINT16 [
    isShapedAlone set_isShapedAlone[0..1],
    reserved set_reserved[1..16],
]}
STRUCT!{struct DWRITE_SHAPING_GLYPH_PROPERTIES {
    bit_fields: UINT16,
}}
BITFIELD!{DWRITE_SHAPING_GLYPH_PROPERTIES bit_fields: UINT16 [
    justification set_justification[0..4],
    isClusterStart set_isClusterStart[4..5],
    isDiacritic set_isDiacritic[5..6],
    isZeroWidthSpace set_isZeroWidthSpace[6..7],
    reserved set_reserved[7..16],
]}
RIDL!{interface IDWriteTextAnalysisSource(IDWriteTextAnalysisSourceVtbl): IUnknown(IUnknownVtbl) {
    fn GetTextAtPosition(
        &self, textPosition: UINT32, textString: *mut *const WCHAR,
        textLength: *mut UINT32
    ) -> HRESULT,
    fn GetTextBeforePosition(
        &self, textPosition: UINT32, textString: *mut *const WCHAR,
        textLength: *mut UINT32
    ) -> HRESULT,
    fn GetParagraphReadingDirection(&self) -> DWRITE_READING_DIRECTION,
    fn GetLocaleName(
        &self, textPosition: UINT32, textLength: *mut UINT32,
        localeName: *mut *const WCHAR
    ) -> HRESULT,
    fn GetNumberSubstitution(
        &self, textPosition: UINT32, textLength: *mut UINT32,
        numberSubstitution: *mut *mut IDWriteNumberSubstitution
    ) -> HRESULT
}}
RIDL!{interface IDWriteTextAnalysisSink(IDWriteTextAnalysisSinkVtbl): IUnknown(IUnknownVtbl) {
    fn SetScriptAnalysis(
        &self, textPosition: UINT32, textLength: UINT32,
        scriptAnalysis: *const DWRITE_SCRIPT_ANALYSIS
    ) -> HRESULT,
    fn SetLineBreakpoints(
        &self, textPosition: UINT32, textLength: UINT32,
        lineBreakpoints: *const DWRITE_LINE_BREAKPOINT
    ) -> HRESULT,
    fn SetBidiLevel(
        &self, textPosition: UINT32, textLength: UINT32, explicitLevel: UINT8,
        resolvedLevel: UINT8
    ) -> HRESULT,
    fn SetNumberSubstitution(
        &self, textPosition: UINT32, textLength: UINT32,
        numberSubstitution: *mut IDWriteNumberSubstitution
    ) -> HRESULT
}}
RIDL!{interface IDWriteTextAnalyzer(IDWriteTextAnalyzerVtbl): IUnknown(IUnknownVtbl) {
    fn AnalyzeScript(
        &self, analysisSource: *mut IDWriteTextAnalysisSource, textPosition: UINT32,
        textLength: UINT32, analysisSink: *mut IDWriteTextAnalysisSink
    ) -> HRESULT,
    fn AnalyzeBidi(
        &self, analysisSource: *mut IDWriteTextAnalysisSource, textPosition: UINT32,
        textLength: UINT32, analysisSink: *mut IDWriteTextAnalysisSink
    ) -> HRESULT,
    fn AnalyzeNumberSubstitution(
        &self, analysisSource: *mut IDWriteTextAnalysisSource, textPosition: UINT32,
        textLength: UINT32, analysisSink: *mut IDWriteTextAnalysisSink
    ) -> HRESULT,
    fn AnalyzeLineBreakpoints(
        &self, analysisSource: *mut IDWriteTextAnalysisSource, textPosition: UINT32,
        textLength: UINT32, analysisSink: *mut IDWriteTextAnalysisSink
    ) -> HRESULT,
    fn GetGlyphs(
        &self, textString: *const WCHAR, textLength: UINT32,
        fontFace: *mut IDWriteFontFace, isSideways: BOOL, isRightToLeft: BOOL,
        scriptAnalysis: *const DWRITE_SCRIPT_ANALYSIS, localeName: *const WCHAR,
        numberSubstitution: *mut IDWriteNumberSubstitution,
        features: *mut *const DWRITE_TYPOGRAPHIC_FEATURES, featureRangeLengths: *const UINT32,
        featureRanges: UINT32, maxGlyphCount: UINT32, clusterMap: *mut UINT16,
        textProps: *mut DWRITE_SHAPING_TEXT_PROPERTIES, glyphIndices: *mut UINT16,
        glyphProps: *mut DWRITE_SHAPING_GLYPH_PROPERTIES, actualGlyphCount: *mut UINT32
    ) -> HRESULT,
    fn GetGlyphPlacements(
        &self, textString: *const WCHAR, clusterMap: *const UINT16,
        textProps: *mut DWRITE_SHAPING_TEXT_PROPERTIES, textLength: UINT32,
        glyphIndices: *const UINT16, glyphProps: *const DWRITE_SHAPING_GLYPH_PROPERTIES,
        glyphCount: UINT32, fontFace: *mut IDWriteFontFace, fontEmSize: FLOAT,
        isSideways: BOOL, isRightToLeft: BOOL, scriptAnalysis: *const DWRITE_SCRIPT_ANALYSIS,
        localeName: *const WCHAR, features: *mut *const DWRITE_TYPOGRAPHIC_FEATURES,
        featureRangeLengths: *const UINT32, featureRanges: UINT32, glyphAdvances: *mut FLOAT,
        glyphOffsets: *mut DWRITE_GLYPH_OFFSET
    ) -> HRESULT,
    fn GetGdiCompatibleGlyphPlacements(
        &self, textString: *const WCHAR, clusterMap: *const UINT16,
        textProps: *mut DWRITE_SHAPING_TEXT_PROPERTIES, textLength: UINT32,
        glyphIndices: *const UINT16, glyphProps: *const DWRITE_SHAPING_GLYPH_PROPERTIES,
        glyphCount: UINT32, fontFace: *mut IDWriteFontFace, fontEmSize: FLOAT,
        pixelsPerDip: FLOAT, transform: *const DWRITE_MATRIX, useGdiNatrual: BOOL,
        isSideways: BOOL, isRightToLeft: BOOL, scriptAnalysis: *const DWRITE_SCRIPT_ANALYSIS,
        localeName: *const WCHAR, features: *mut *const DWRITE_TYPOGRAPHIC_FEATURES,
        featureRangeLengths: *const UINT32, featureRanges: UINT32, glyphAdvances: *mut FLOAT,
        glyphOffsets: *mut DWRITE_GLYPH_OFFSET
    ) -> HRESULT
}}
STRUCT!{struct DWRITE_GLYPH_RUN {
    fontFace: *mut IDWriteFontFace,
    fontEmSize: FLOAT,
    glyphCount: UINT32,
    glyphIndices: *const UINT16,
    glyphAdvances: *const FLOAT,
    glyphOffsets: *const DWRITE_GLYPH_OFFSET,
    isSideways: BOOL,
    bidiLevel: UINT32,
}}
STRUCT!{struct DWRITE_GLYPH_RUN_DESCRIPTION {
    localeName: *const WCHAR,
    string: *const WCHAR,
    stringLength: UINT32,
    clusterMap: *const UINT16,
    textPosition: UINT32,
}}
STRUCT!{struct DWRITE_UNDERLINE {
    width: FLOAT,
    thickness: FLOAT,
    offset: FLOAT,
    runHeight: FLOAT,
    readingDirection: DWRITE_READING_DIRECTION,
    flowDirection: DWRITE_FLOW_DIRECTION,
    localeName: *const WCHAR,
    measuringMode: DWRITE_MEASURING_MODE,
}}
STRUCT!{struct DWRITE_STRIKETHROUGH {
    width: FLOAT,
    thickness: FLOAT,
    offset: FLOAT,
    readingDirection: DWRITE_READING_DIRECTION,
    flowDirection: DWRITE_FLOW_DIRECTION,
    localeName: *const WCHAR,
    measuringMode: DWRITE_MEASURING_MODE,
}}
STRUCT!{struct DWRITE_LINE_METRICS {
    length: UINT32,
    trailingWhitespaceLength: UINT32,
    newlineLength: UINT32,
    height: FLOAT,
    baseline: FLOAT,
    isTrimmed: BOOL,
}}
STRUCT!{struct DWRITE_CLUSTER_METRICS {
    width: FLOAT,
    length: UINT16,
    bit_fields: UINT16,
}}
BITFIELD!{DWRITE_CLUSTER_METRICS bit_fields: UINT16 [
    canWrapLineAfter set_canWrapLineAfter[0..1],
    isWhitespace set_isWhitespace[1..2],
    isNewline set_isNewline[2..3],
    isSoftHyphen set_isSoftHyphen[3..4],
    isRightToLeft set_isRightToLeft[4..5],
    padding set_padding[5..16],
]}
STRUCT!{struct DWRITE_TEXT_METRICS {
    left: FLOAT,
    top: FLOAT,
    width: FLOAT,
    widthIncludingTrailingWhitespace: FLOAT,
    height: FLOAT,
    layoutWidth: FLOAT,
    layoutHeight: FLOAT,
    maxBidiReorderingDepth: UINT32,
    lineCount: UINT32,
}}
STRUCT!{struct DWRITE_INLINE_OBJECT_METRICS {
    width: FLOAT,
    height: FLOAT,
    baseline: FLOAT,
    supportsSideways: BOOL,
}}
STRUCT!{struct DWRITE_OVERHANG_METRICS {
    left: FLOAT,
    top: FLOAT,
    right: FLOAT,
    bottom: FLOAT,
}}
STRUCT!{struct DWRITE_HIT_TEST_METRICS {
    textPosition: UINT32,
    length: UINT32,
    left: FLOAT,
    top: FLOAT,
    width: FLOAT,
    height: FLOAT,
    bidiLevel: UINT32,
    isText: BOOL,
    isTrimmed: BOOL,
}}
RIDL!{interface IDWriteInlineObject(IDWriteInlineObjectVtbl): IUnknown(IUnknownVtbl) {
    fn Draw(
        &self, clientDrawingContext: *mut c_void, renderer: *mut IDWriteTextRenderer,
        originX: FLOAT, originY: FLOAT, isSideways: BOOL, isRightToLeft: BOOL,
        clientDrawingEffect: *mut IUnknown
    ) -> HRESULT,
    fn GetMetrics(&self, metrics: *mut DWRITE_INLINE_OBJECT_METRICS) -> HRESULT,
    fn GetOverhangMetrics(&self, overhangs: *mut DWRITE_OVERHANG_METRICS) -> HRESULT,
    fn GetBreakConditions(
        &self, breakConditionBefore: *mut DWRITE_BREAK_CONDITION,
        breakConditionAfter: *mut DWRITE_BREAK_CONDITION
    ) -> HRESULT
}}
RIDL!{interface IDWritePixelSnapping(IDWritePixelSnappingVtbl): IUnknown(IUnknownVtbl) {
    fn IsPixelSnappingDisabled(
        &self, clientDrawingContext: *mut c_void, isDisabled: *mut BOOL
    ) -> HRESULT,
    fn GetCurrentTransform(
        &self, clientDrawingContext: *mut c_void, transform: *mut DWRITE_MATRIX
    ) -> HRESULT,
    fn GetPixelsPerDip(
        &self, clientDrawingContext: *mut c_void, pixelsPerDip: *mut FLOAT
    ) -> HRESULT
}}
RIDL!{interface IDWriteTextRenderer(IDWriteTextRendererVtbl):
        IDWritePixelSnapping(IDWritePixelSnappingVtbl) {
    fn DrawGlyphRun(
        &self, clientDrawingContext: *mut c_void, baselineOriginX: FLOAT,
        baselineOriginY: FLOAT, measuringMode: DWRITE_MEASURING_MODE,
        glyphRun: *const DWRITE_GLYPH_RUN,
        glyphRunDescription: *const DWRITE_GLYPH_RUN_DESCRIPTION,
        clientDrawingEffect: *mut IUnknown
    ) -> HRESULT,
    fn DrawUnderline(
        &self, clientDrawingContext: *mut c_void, baselineOriginX: FLOAT,
        baselineOriginY: FLOAT, underline: *const DWRITE_UNDERLINE,
        clientDrawingEffect: *mut IUnknown
    ) -> HRESULT,
    fn DrawStrikethrough(
        &self, clientDrawingContext: *mut c_void, baselineOriginX: FLOAT,
        baselineOriginY: FLOAT, strikethrough: *const DWRITE_STRIKETHROUGH,
        clientDrawingEffect: *mut IUnknown
    ) -> HRESULT,
    fn DrawInlineObject(
        &self, clientDrawingContext: *mut c_void, baselineOriginX: FLOAT,
        baselineOriginY: FLOAT, inlineObject: *mut IDWriteInlineObject,
        isSideways: BOOL, isRightToLeft: BOOL, clientDrawingEffect: *mut IUnknown
    ) -> HRESULT
}}
RIDL!{interface IDWriteTextLayout(IDWriteTextLayoutVtbl):
        IDWriteTextFormat(IDWriteTextFormatVtbl) {
    fn SetMaxWidth(&self, maxWidth: FLOAT) -> HRESULT,
    fn SetMaxHeight(&self, maxHeight: FLOAT) -> HRESULT,
    fn SetFontCollection(
        &self, fontCollection: *mut IDWriteFontCollection, textRange: DWRITE_TEXT_RANGE
    ) -> HRESULT,
    fn SetFontFamilyName(
        &self, fontFamilyName: *const WCHAR, textRange: DWRITE_TEXT_RANGE
    ) -> HRESULT,
    fn SetFontWeight(
        &self, fontWeight: DWRITE_FONT_WEIGHT, textRange: DWRITE_TEXT_RANGE
    ) -> HRESULT,
    fn SetFontStyle(
        &self, fontStyle: DWRITE_FONT_STYLE, textRange: DWRITE_TEXT_RANGE
    ) -> HRESULT,
    fn SetFontStretch(
        &self, fontStretch: DWRITE_FONT_STRETCH, textRange: DWRITE_TEXT_RANGE
    ) -> HRESULT,
    fn SetFontSize(&self, fontSize: FLOAT, textRange: DWRITE_TEXT_RANGE) -> HRESULT,
    fn SetUnderline(&self, hasUnderline: BOOL, textRange: DWRITE_TEXT_RANGE) -> HRESULT,
    fn SetStrikethrough(
        &self, hasStrikethrough: BOOL, textRange: DWRITE_TEXT_RANGE
    ) -> HRESULT,
    fn SetDrawingEffect(
        &self, drawingEffect: *mut IUnknown, textRange: DWRITE_TEXT_RANGE
    ) -> HRESULT,
    fn SetInlineObject(
        &self, inlineObject: *mut IDWriteInlineObject, textRange: DWRITE_TEXT_RANGE
    ) -> HRESULT,
    fn SetTypography(
        &self, typography: *mut IDWriteTypography, textRange: DWRITE_TEXT_RANGE
    ) -> HRESULT,
    fn SetLocaleName(
        &self, localeName: *const WCHAR, textRange: DWRITE_TEXT_RANGE
    ) -> HRESULT,
    fn GetMaxWidth(&self) -> FLOAT,
    fn GetMaxHeight(&self) -> FLOAT,
    fn GetFontCollection(
        &self, currentPosition: UINT32, fontCollection: *mut *mut IDWriteFontCollection,
        textRange: *mut DWRITE_TEXT_RANGE
    ) -> HRESULT,
    fn GetFontFamilyNameLength(
        &self, currentPosition: UINT32, nameLength: *mut UINT32,
        textRange: *mut DWRITE_TEXT_RANGE
    ) -> HRESULT,
    fn GetFontFamilyName(
        &self, currentPosition: UINT32, fontFamilyName: *mut WCHAR,
        nameSize: UINT32, textRange: *mut DWRITE_TEXT_RANGE
    ) -> HRESULT,
    fn GetFontWeight(
        &self, currentPosition: UINT32, fontWeight: *mut DWRITE_FONT_WEIGHT,
        textRange: *mut DWRITE_TEXT_RANGE
    ) -> HRESULT,
    fn GetFontStyle(
        &self, currentPosition: UINT32, fontStyle: *mut DWRITE_FONT_STYLE,
        textRange: *mut DWRITE_TEXT_RANGE
    ) -> HRESULT,
    fn GetFontStretch(
        &self, currentPosition: UINT32, fontStretch: *mut DWRITE_FONT_STRETCH,
        textRange: *mut DWRITE_TEXT_RANGE
    ) -> HRESULT,
    fn GetFontSize(
        &self, currentPosition: UINT32, fontSize: *mut FLOAT,
        textRange: *mut DWRITE_TEXT_RANGE
    ) -> HRESULT,
    fn GetUnderline(
        &self, currentPosition: UINT32, hasUnderline: *mut BOOL,
        textRange: *mut DWRITE_TEXT_RANGE
    ) -> HRESULT,
    fn GetStrikethrough(
        &self, currentPosition: UINT32, hasStrikethrough: *mut BOOL,
        textRange: *mut DWRITE_TEXT_RANGE
    ) -> HRESULT,
    fn GetDrawingEffect(
        &self, currentPosition: UINT32, drawingEffect: *mut *mut IUnknown,
        textRange: *mut DWRITE_TEXT_RANGE
    ) -> HRESULT,
    fn GetInlineObject(
        &self, currentPosition: UINT32, inlineObject: *mut *mut IDWriteInlineObject,
        textRange: *mut DWRITE_TEXT_RANGE
    ) -> HRESULT,
    fn GetTypography(
        &self, currentPosition: UINT32, typography: *mut *mut IDWriteTypography,
        textRange: *mut DWRITE_TEXT_RANGE
    ) -> HRESULT,
    fn GetLocaleNameLength(
        &self, currentPosition: UINT32, nameLength: *mut UINT32,
        textRange: *mut DWRITE_TEXT_RANGE
    ) -> HRESULT,
    fn GetLocaleName(
        &self, currentPosition: UINT32, localeName: *mut WCHAR, nameSize: UINT32,
        textRange: *mut DWRITE_TEXT_RANGE
    ) -> HRESULT,
    fn Draw(
        &self, clientDrawingContext: *mut c_void, renderer: *mut IDWriteTextRenderer,
        originX: FLOAT, originY: FLOAT
    ) -> HRESULT,
    fn GetLineMetrics(
        &self, lineMetrics: *mut DWRITE_LINE_METRICS, maxLineCount: UINT32,
        actualLineCount: *mut UINT32
    ) -> HRESULT,
    fn GetMetrics(&self, textMetrics: *mut DWRITE_TEXT_METRICS) -> HRESULT,
    fn GetOverhangMetrics(&self, overhangs: *mut DWRITE_OVERHANG_METRICS) -> HRESULT,
    fn GetClusterMetrics(
        &self, clusterMetrics: *mut DWRITE_CLUSTER_METRICS, maxClusterCount: UINT32,
        actualClusterCount: *mut UINT32
    ) -> HRESULT,
    fn DetermineMinWidth(&self, minWidth: *mut FLOAT) -> HRESULT,
    fn HitTestPoint(
        &self, pointX: FLOAT, pointY: FLOAT, isTrailingHit: *mut BOOL,
        isInside: *mut BOOL, hitTestMetrics: *mut DWRITE_HIT_TEST_METRICS
    ) -> HRESULT,
    fn HitTestTextPosition(
        &self, textPosition: UINT32, isTrailingHit: BOOL, pointX: *mut FLOAT,
        pointY: *mut FLOAT, hitTestMetrics: *mut DWRITE_HIT_TEST_METRICS
    ) -> HRESULT,
    fn HitTestTextRange(
        &self, textPosition: UINT32, textLength: UINT32, originX: FLOAT,
        originY: FLOAT, hitTestMetrics: *mut DWRITE_HIT_TEST_METRICS,
        maxHitTestMetricsCount: UINT32, actualHitTestMetricsCount: *mut UINT32
    ) -> HRESULT
}}
RIDL!{interface IDWriteBitmapRenderTarget(IDWriteBitmapRenderTargetVtbl): IUnknown(IUnknownVtbl) {
    fn DrawGlyphRun(
        &self, baselineOriginX: FLOAT, baselineOriginY: FLOAT,
        measuringMode: DWRITE_MEASURING_MODE, glyphRun: *const DWRITE_GLYPH_RUN,
        renderingParams: *mut IDWriteRenderingParams, textColor: COLORREF,
        blackBoxRect: *mut RECT
    ) -> HRESULT,
    fn GetMemoryDC(&self) -> HDC,
    fn GetPixelsPerDip(&self) -> FLOAT,
    fn SetPixelsPerDip(&self, pixelsPerDip: FLOAT) -> HRESULT,
    fn GetCurrentTransform(&self, transform: *mut DWRITE_MATRIX) -> HRESULT,
    fn SetCurrentTransform(&self, transform: *const DWRITE_MATRIX) -> HRESULT,
    fn GetSize(&self, size: *mut SIZE) -> HRESULT,
    fn Resize(&self, width: UINT32, height: UINT32) -> HRESULT
}}
RIDL!{interface IDWriteGdiInterop(IDWriteGdiInteropVtbl): IUnknown(IUnknownVtbl) {
    fn CreateFontFromLOGFONT(
        &self, logFont: *const LOGFONTW, font: *mut *mut IDWriteFont
    ) -> HRESULT,
    fn ConvertFontToLOGFONT(
        &self, font: *mut IDWriteFont, logFont: *mut LOGFONTW, isSystemFont: *mut BOOL
    ) -> HRESULT,
    fn ConvertFontFaceToLOGFONT(
        &self, font: *mut IDWriteFontFace, logFont: *mut LOGFONTW
    ) -> HRESULT,
    fn CreateFontFaceFromHdc(
        &self, hdc: HDC, fontFace: *mut *mut IDWriteFontFace
    ) -> HRESULT,
    fn CreateBitmapRenderTarget(
        &self, hdc: HDC, width: UINT32, height: UINT32,
        renderTarget: *mut *mut IDWriteBitmapRenderTarget
    ) -> HRESULT
}}
ENUM!{enum DWRITE_TEXTURE_TYPE {
    DWRITE_TEXTURE_ALIASED_1x1 = 0,
    DWRITE_TEXTURE_CLEARTYPE_3x1 = 1,
}}
pub const DWRITE_ALPHA_MAX: BYTE = 255;
RIDL!{interface IDWriteGlyphRunAnalysis(IDWriteGlyphRunAnalysisVtbl): IUnknown(IUnknownVtbl) {
    fn GetAlphaTextureBounds(
        &self, textureType: DWRITE_TEXTURE_TYPE, textureBounds: *mut RECT
    ) -> HRESULT,
    fn CreateAlphaTexture(
        &self, textureType: DWRITE_TEXTURE_TYPE, textureBounds: *const RECT,
        alphaValues: *mut BYTE, bufferSize: UINT32
    ) -> HRESULT,
    fn GetAlphaBlendParams(
        &self, renderingParams: *mut IDWriteRenderingParams, blendGamma: *mut FLOAT,
        blendEnhancedContrast: *mut FLOAT, blendClearTypeLevel: *mut FLOAT
    ) -> HRESULT
}}
RIDL!{interface IDWriteFactory(IDWriteFactoryVtbl): IUnknown(IUnknownVtbl) {
    fn GetSystemFontCollection(
        &self, fontCollection: *mut *mut IDWriteFontCollection, checkForUpdates: BOOL
    ) -> HRESULT,
    fn CreateCustomFontCollection(
        &self, collectionLoader: *mut IDWriteFontCollectionLoader,
        collectionKey: *const c_void, collectionKeySize: UINT32,
        fontCollection: *mut *mut IDWriteFontCollection
    ) -> HRESULT,
    fn RegisterFontCollectionLoader(
        &self, fontCollectionLoader: *mut IDWriteFontCollectionLoader
    ) -> HRESULT,
    fn UnregisterFontCollectionLoader(
        &self, fontCollectionLoader: *mut IDWriteFontCollectionLoader
    ) -> HRESULT,
    fn CreateFontFileReference(
        &self, filePath: *const WCHAR, lastWriteTime: *const FILETIME,
        fontFile: *mut *mut IDWriteFontFile
    ) -> HRESULT,
    fn CreateCustomFontFileReference(
        &self, fontFileReferenceKey: *const c_void, fontFileReferenceKeySize: UINT32,
        fontFileLoader: *mut IDWriteFontFileLoader, fontFile: *mut *mut IDWriteFontFile
    ) -> HRESULT,
    fn CreateFontFace(
        &self, fontFaceType: DWRITE_FONT_FACE_TYPE, numberOfFiles: UINT32,
        fontFiles: *const *mut IDWriteFontFile, faceIndex: UINT32,
        fontFaceSimulationFlags: DWRITE_FONT_SIMULATIONS, fontFace: *mut *mut IDWriteFontFace
    ) -> HRESULT,
    fn CreateRenderingParams(
        &self, renderingParams: *mut *mut IDWriteRenderingParams
    ) -> HRESULT,
    fn CreateMonitorRenderingParams(
        &self, monitor: HMONITOR, renderingParams: *mut *mut IDWriteRenderingParams
    ) -> HRESULT,
    fn CreateCustomRenderingParams(
        &self, gamma: FLOAT, enhancedContrast: FLOAT, clearTypeLevel: FLOAT,
        pixelGeometry: DWRITE_PIXEL_GEOMETRY, renderingMode: DWRITE_RENDERING_MODE,
        renderingParams: *mut *mut IDWriteRenderingParams
    ) -> HRESULT,
    fn RegisterFontFileLoader(
        &self, fontFileLoader: *mut IDWriteFontFileLoader
    ) -> HRESULT,
    fn UnregisterFontFileLoader(
        &self, fontFileLoader: *mut IDWriteFontFileLoader
    ) -> HRESULT,
    fn CreateTextFormat(
        &self, fontFamilyName: *const WCHAR, fontCollection: *mut IDWriteFontCollection,
        fontWeight: DWRITE_FONT_WEIGHT, fontStyle: DWRITE_FONT_STYLE,
        fontStretch: DWRITE_FONT_STRETCH, fontSize: FLOAT, localeName: *const WCHAR,
        textFormat: *mut *mut IDWriteTextFormat
    ) -> HRESULT,
    fn CreateTypography(&self, typography: *mut *mut IDWriteTypography) -> HRESULT,
    fn GetGdiInterop(&self, gdiInterop: *mut *mut IDWriteGdiInterop) -> HRESULT,
    fn CreateTextLayout(
        &self, string: *const WCHAR, stringLength: UINT32,
        textFormat: *mut IDWriteTextFormat, maxWidth: FLOAT, maxHeight: FLOAT,
        textLayout: *mut *mut IDWriteTextLayout
    ) -> HRESULT,
    fn CreateGdiCompatibleTextLayout(
        &self, string: *const WCHAR, stringLength: UINT32,
        textFormat: *mut IDWriteTextFormat, layoutWidth: FLOAT, layoutHeight: FLOAT,
        pixelsPerDip: FLOAT, transform: *const DWRITE_MATRIX, useGdiNatrual: BOOL,
        textLayout: *mut *mut IDWriteTextLayout
    ) -> HRESULT,
    fn CreateEllipsisTrimmingSign(
        &self, textFormat: *mut IDWriteTextFormat, trimmingSign: *mut *mut IDWriteInlineObject
    ) -> HRESULT,
    fn CreateTextAnalyzer(&self, textAnalyzer: *mut *mut IDWriteTextAnalyzer) -> HRESULT,
    fn CreateNumberSubstitution(
        &self, substitutionMethod: DWRITE_NUMBER_SUBSTITUTION_METHOD,
        localeName: *const WCHAR, ignoreUserOverride: BOOL,
        numberSubstitution: *mut *mut IDWriteNumberSubstitution
    ) -> HRESULT,
    fn CreateGlyphRunAnalysis(
        &self, glyphRun: *const DWRITE_GLYPH_RUN, pixelsPerDip: FLOAT,
        transform: *const DWRITE_MATRIX, renderingMode: DWRITE_RENDERING_MODE,
        measuringMode: DWRITE_MEASURING_MODE, baselineOriginX: FLOAT,
        baselineOriginY: FLOAT, glyphRunAnalysis: *mut *mut IDWriteGlyphRunAnalysis
    ) -> HRESULT
}}
pub const FACILITY_DWRITE: HRESULT = 0x898;
pub const DWRITE_ERR_BASE: HRESULT = 0x5000;
#[inline]
pub fn MAKE_DWRITE_HR(severity: HRESULT, code: HRESULT) -> HRESULT {
    MAKE_HRESULT!(severity, FACILITY_DWRITE, DWRITE_ERR_BASE + code)
}
#[inline]
pub fn MAKE_DWRITE_HR_ERR(code: HRESULT) -> HRESULT {
    MAKE_DWRITE_HR(SEVERITY_ERROR, code)
}
