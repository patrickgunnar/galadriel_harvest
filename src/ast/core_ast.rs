use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    pub static ref CORE_AST: Mutex<HashMap<String, HashMap<String, Vec<String>>>> = {
        let mut ast = HashMap::new();

        // box model declarations
        let mut box_model = HashMap::new();

        box_model.insert("width".to_string(), vec![]);
        box_model.insert("height".to_string(), vec![]);
        box_model.insert("margin".to_string(), vec![]);
        box_model.insert("marginBottom".to_string(), vec![]);
        box_model.insert("marginLeft".to_string(), vec![]);
        box_model.insert("marginRight".to_string(), vec![]);
        box_model.insert("marginTop".to_string(), vec![]);
        box_model.insert("padding".to_string(), vec![]);
        box_model.insert("paddingBottom".to_string(), vec![]);
        box_model.insert("paddingLeft".to_string(), vec![]);
        box_model.insert("paddingRight".to_string(), vec![]);
        box_model.insert("paddingTop".to_string(), vec![]);

        ast.insert("boxModel".to_string(), box_model);

        // layout declarations
        let mut layout = HashMap::new();

        layout.insert("display".to_string(), vec![]);
        layout.insert("position".to_string(), vec![]);
        layout.insert("flexDirection".to_string(), vec![]);
        layout.insert("flexWrap".to_string(), vec![]);
        layout.insert("flex".to_string(), vec![]);
        layout.insert("justifyContent".to_string(), vec![]);
        layout.insert("alignItems".to_string(), vec![]);
        layout.insert("alignContent".to_string(), vec![]);
        layout.insert("gridTemplateColumns".to_string(), vec![]);
        layout.insert("gridTemplateRows".to_string(), vec![]);
        layout.insert("gridColumn".to_string(), vec![]);
        layout.insert("gridRow".to_string(), vec![]);
        layout.insert("gridAutoColumns".to_string(), vec![]);
        layout.insert("gridAutoRows".to_string(), vec![]);
        layout.insert("gridAutoFlow".to_string(), vec![]);

        ast.insert("layout".to_string(), layout);

        // background declarations
        let mut background = HashMap::new();

        background.insert("background".to_string(), vec![]);
        background.insert("backgroundColor".to_string(), vec![]);
        background.insert("backgroundImage".to_string(), vec![]);
        background.insert("backgroundPosition".to_string(), vec![]);
        background.insert("backgroundPositionX".to_string(), vec![]);
        background.insert("backgroundPositionY".to_string(), vec![]);
        background.insert("backgroundRepeat".to_string(), vec![]);
        background.insert("backgroundClip".to_string(), vec![]);
        background.insert("backgroundOrigin".to_string(), vec![]);
        background.insert("backgroundSize".to_string(), vec![]);
        background.insert("backgroundBlendMode".to_string(), vec![]);

        ast.insert("background".to_string(), background);

        // borders declarations
        let mut borders = HashMap::new();

        borders.insert("border".to_string(), vec![]);
        borders.insert("borderBottom".to_string(), vec![]);
        borders.insert("borderBottomColor".to_string(), vec![]);
        borders.insert("borderBottomStyle".to_string(), vec![]);
        borders.insert("borderBottomWidth".to_string(), vec![]);
        borders.insert("borderColor".to_string(), vec![]);
        borders.insert("borderLeft".to_string(), vec![]);
        borders.insert("borderLeftColor".to_string(), vec![]);
        borders.insert("borderLeftStyle".to_string(), vec![]);
        borders.insert("borderLeftWidth".to_string(), vec![]);
        borders.insert("borderRight".to_string(), vec![]);
        borders.insert("borderRightColor".to_string(), vec![]);
        borders.insert("borderRightStyles".to_string(), vec![]);
        borders.insert("borderRightWidth".to_string(), vec![]);
        borders.insert("borderStyle".to_string(), vec![]);
        borders.insert("borderTop".to_string(), vec![]);
        borders.insert("borderTopColor".to_string(), vec![]);
        borders.insert("borderTopStyle".to_string(), vec![]);
        borders.insert("borderTopWidth".to_string(), vec![]);
        borders.insert("borderWidth".to_string(), vec![]);

        ast.insert("borders".to_string(), borders);

        //typography declarations
        let mut typography = HashMap::new();

        typography.insert("fontFamily".to_string(), vec![]);
        typography.insert("fontSize".to_string(), vec![]);
        typography.insert("fontStyle".to_string(), vec![]);
        typography.insert("fontVariant".to_string(), vec![]);
        typography.insert("fontWeight".to_string(), vec![]);
        typography.insert("fontSizeAdjust".to_string(), vec![]);
        typography.insert("fontStretch".to_string(), vec![]);
        typography.insert("textAlign".to_string(), vec![]);
        typography.insert("letterSpacing".to_string(), vec![]);
        typography.insert("lineHeight".to_string(), vec![]);
        typography.insert("lineBreak".to_string(), vec![]);
        typography.insert("color".to_string(), vec![]);
        typography.insert("textDecoration".to_string(), vec![]);
        typography.insert("textIndent".to_string(), vec![]);
        typography.insert("textTransform".to_string(), vec![]);
        typography.insert("unicodeBidi".to_string(), vec![]);
        typography.insert("verticalAlign".to_string(), vec![]);
        typography.insert("whiteSpace".to_string(), vec![]);
        typography.insert("wordSpacing".to_string(), vec![]);
        typography.insert("textOutline".to_string(), vec![]);
        typography.insert("textOverflow".to_string(), vec![]);
        typography.insert("textShadow".to_string(), vec![]);
        typography.insert("textWrap".to_string(), vec![]);
        typography.insert("wordBreak".to_string(), vec![]);
        typography.insert("wordWrap".to_string(), vec![]);

        ast.insert("typography".to_string(), typography);

        //transform and animation declarations
        let mut transform_and_animation = HashMap::new();

        transform_and_animation.insert("transform".to_string(), vec![]);
        transform_and_animation.insert("transformOrigin".to_string(), vec![]);
        transform_and_animation.insert("transformStyle".to_string(), vec![]);
        transform_and_animation.insert("perspective".to_string(), vec![]);
        transform_and_animation.insert("perspectiveOrigin".to_string(), vec![]);
        transform_and_animation.insert("backfaceVisibility".to_string(), vec![]);
        transform_and_animation.insert("animation".to_string(), vec![]);
        transform_and_animation.insert("animationName".to_string(), vec![]);
        transform_and_animation.insert("animationDuration".to_string(), vec![]);
        transform_and_animation.insert("animationTimingFunction".to_string(), vec![]);
        transform_and_animation.insert("animationDelay".to_string(), vec![]);
        transform_and_animation.insert("animationFillMode".to_string(), vec![]);
        transform_and_animation.insert("animationIterationCount".to_string(), vec![]);
        transform_and_animation.insert("animationDirection".to_string(), vec![]);
        transform_and_animation.insert("animationPlayState".to_string(), vec![]);

        ast.insert("transformAndAnimation".to_string(), transform_and_animation);

        // other properties declarations
        let mut other_properties = HashMap::new();

        other_properties.insert("aspectRatio".to_string(), vec![]);
        other_properties.insert("accentColor".to_string(), vec![]);
        other_properties.insert("backdropFilter".to_string(), vec![]);
        other_properties.insert("content".to_string(), vec![]);
        other_properties.insert("gap".to_string(), vec![]);
        other_properties.insert("rowGap".to_string(), vec![]);
        other_properties.insert("scale".to_string(), vec![]);
        other_properties.insert("order".to_string(), vec![]);
        other_properties.insert("pointerEvents".to_string(), vec![]);
        other_properties.insert("filter".to_string(), vec![]);
        other_properties.insert("maxHeight".to_string(), vec![]);
        other_properties.insert("maxWidth".to_string(), vec![]);
        other_properties.insert("minHeight".to_string(), vec![]);
        other_properties.insert("minWidth".to_string(), vec![]);
        other_properties.insert("borderRadius".to_string(), vec![]);
        other_properties.insert("borderTopLeftRadius".to_string(), vec![]);
        other_properties.insert("borderTopRightRadius".to_string(), vec![]);
        other_properties.insert("borderBottomLeftRadius".to_string(), vec![]);
        other_properties.insert("borderBottomRightRadius".to_string(), vec![]);
        other_properties.insert("boxDecorationBreak".to_string(), vec![]);
        other_properties.insert("boxShadow".to_string(), vec![]);
        other_properties.insert("backgroundAttachment".to_string(), vec![]);
        other_properties.insert("backgroundRepeat".to_string(), vec![]);
        other_properties.insert("backgroundSize".to_string(), vec![]);
        other_properties.insert("backgroundClip".to_string(), vec![]);
        other_properties.insert("backgroundOrigin".to_string(), vec![]);
        other_properties.insert("colorProfile".to_string(), vec![]);
        other_properties.insert("opacity".to_string(), vec![]);
        other_properties.insert("renderingIntent".to_string(), vec![]);
        other_properties.insert("font".to_string(), vec![]);
        other_properties.insert("fontSize".to_string(), vec![]);
        other_properties.insert("fontStyle".to_string(), vec![]);
        other_properties.insert("fontVariant".to_string(), vec![]);
        other_properties.insert("fontWeight".to_string(), vec![]);
        other_properties.insert("fontSizeAdjust".to_string(), vec![]);
        other_properties.insert("fontStretch".to_string(), vec![]);
        other_properties.insert("Positioning".to_string(), vec![]);
        other_properties.insert("bottom".to_string(), vec![]);
        other_properties.insert("clear".to_string(), vec![]);
        other_properties.insert("clipPath".to_string(), vec![]);
        other_properties.insert("cursor".to_string(), vec![]);
        other_properties.insert("float".to_string(), vec![]);
        other_properties.insert("left".to_string(), vec![]);
        other_properties.insert("overflow".to_string(), vec![]);
        other_properties.insert("position".to_string(), vec![]);
        other_properties.insert("right".to_string(), vec![]);
        other_properties.insert("top".to_string(), vec![]);
        other_properties.insert("visibility".to_string(), vec![]);
        other_properties.insert("zIndex".to_string(), vec![]);
        other_properties.insert("target".to_string(), vec![]);
        other_properties.insert("targetName".to_string(), vec![]);
        other_properties.insert("targetNew".to_string(), vec![]);
        other_properties.insert("targetPosition".to_string(), vec![]);
        other_properties.insert("color".to_string(), vec![]);
        other_properties.insert("direction".to_string(), vec![]);
        other_properties.insert("letterSpacing".to_string(), vec![]);
        other_properties.insert("lineHeight".to_string(), vec![]);
        other_properties.insert("lineBreak".to_string(), vec![]);
        other_properties.insert("textAlign".to_string(), vec![]);
        other_properties.insert("textDecoration".to_string(), vec![]);
        other_properties.insert("textIndent".to_string(), vec![]);
        other_properties.insert("textTransform".to_string(), vec![]);
        other_properties.insert("unicodeBidi".to_string(), vec![]);
        other_properties.insert("verticalAlign".to_string(), vec![]);
        other_properties.insert("whiteSpace".to_string(), vec![]);
        other_properties.insert("wordSpacing".to_string(), vec![]);
        other_properties.insert("textOutline".to_string(), vec![]);
        other_properties.insert("textOverflow".to_string(), vec![]);
        other_properties.insert("textShadow".to_string(), vec![]);
        other_properties.insert("textWrap".to_string(), vec![]);
        other_properties.insert("wordBreak".to_string(), vec![]);
        other_properties.insert("wordWrap".to_string(), vec![]);
        other_properties.insert("listStyle".to_string(), vec![]);
        other_properties.insert("listStyleImage".to_string(), vec![]);
        other_properties.insert("listStylePosition".to_string(), vec![]);
        other_properties.insert("listStyleType".to_string(), vec![]);
        other_properties.insert("borderCollapse".to_string(), vec![]);
        other_properties.insert("borderSpacing".to_string(), vec![]);
        other_properties.insert("captionSide".to_string(), vec![]);
        other_properties.insert("emptyCells".to_string(), vec![]);
        other_properties.insert("tableLayout".to_string(), vec![]);
        other_properties.insert("marqueeDirection".to_string(), vec![]);
        other_properties.insert("marqueePlayCount".to_string(), vec![]);
        other_properties.insert("marqueeSpeed".to_string(), vec![]);
        other_properties.insert("marqueeStyle".to_string(), vec![]);
        other_properties.insert("overflowX".to_string(), vec![]);
        other_properties.insert("overflowY".to_string(), vec![]);
        other_properties.insert("overflowStyle".to_string(), vec![]);
        other_properties.insert("rotation".to_string(), vec![]);
        other_properties.insert("boxAlign".to_string(), vec![]);
        other_properties.insert("boxDirection".to_string(), vec![]);
        other_properties.insert("boxFlex".to_string(), vec![]);
        other_properties.insert("boxFlexGroup".to_string(), vec![]);
        other_properties.insert("boxLines".to_string(), vec![]);
        other_properties.insert("boxOrdinalGroup".to_string(), vec![]);
        other_properties.insert("boxOrient".to_string(), vec![]);
        other_properties.insert("boxPack".to_string(), vec![]);
        other_properties.insert("alignmentAdjust".to_string(), vec![]);
        other_properties.insert("alignmentBaseline".to_string(), vec![]);
        other_properties.insert("baselineShift".to_string(), vec![]);
        other_properties.insert("dominantBaseline".to_string(), vec![]);
        other_properties.insert("dropInitialAfterAdjust".to_string(), vec![]);
        other_properties.insert("dropInitialAfterAlign".to_string(), vec![]);
        other_properties.insert("dropInitialBeforeAdjust".to_string(), vec![]);
        other_properties.insert("dropInitialBeforeAlign".to_string(), vec![]);
        other_properties.insert("dropInitialSize".to_string(), vec![]);
        other_properties.insert("dropInitialValue".to_string(), vec![]);
        other_properties.insert("inlineBoxAlign".to_string(), vec![]);
        other_properties.insert("lineStacking".to_string(), vec![]);
        other_properties.insert("lineStackingRuby".to_string(), vec![]);
        other_properties.insert("lineStackingShift".to_string(), vec![]);
        other_properties.insert("lineStackingStrategy".to_string(), vec![]);
        other_properties.insert("textHeight".to_string(), vec![]);
        other_properties.insert("columnCount".to_string(), vec![]);
        other_properties.insert("columnFill".to_string(), vec![]);
        other_properties.insert("columnGap".to_string(), vec![]);
        other_properties.insert("columnRule".to_string(), vec![]);
        other_properties.insert("columnRuleColor".to_string(), vec![]);
        other_properties.insert("columnRuleStyle".to_string(), vec![]);
        other_properties.insert("columnRuleWidth".to_string(), vec![]);
        other_properties.insert("columnSpan".to_string(), vec![]);
        other_properties.insert("columnWidth".to_string(), vec![]);
        other_properties.insert("columns".to_string(), vec![]);
        other_properties.insert("scrollbarColor".to_string(), vec![]);
        other_properties.insert("scrollbarWidth".to_string(), vec![]);
        other_properties.insert("scrollbarGutter".to_string(), vec![]);

        ast.insert("otherProperties".to_string(), other_properties);

        // pseudo selectors declarations
        let mut pseudo_selectors = HashMap::new();

        pseudo_selectors.insert("hover".to_string(), vec![]);
        pseudo_selectors.insert("active".to_string(), vec![]);
        pseudo_selectors.insert("focus".to_string(), vec![]);
        pseudo_selectors.insert("firstChild".to_string(), vec![]);
        pseudo_selectors.insert("lastChild".to_string(), vec![]);
        pseudo_selectors.insert("firstOfType".to_string(), vec![]);
        pseudo_selectors.insert("lastOfType".to_string(), vec![]);
        pseudo_selectors.insert("onlyChild".to_string(), vec![]);
        pseudo_selectors.insert("onlyOfType".to_string(), vec![]);
        pseudo_selectors.insert("targetPseudoClass".to_string(), vec![]);
        pseudo_selectors.insert("visited".to_string(), vec![]);
        pseudo_selectors.insert("checked".to_string(), vec![]);
        pseudo_selectors.insert("disabled".to_string(), vec![]);
        pseudo_selectors.insert("enabled".to_string(), vec![]);
        pseudo_selectors.insert("readOnly".to_string(), vec![]);
        pseudo_selectors.insert("readWrite".to_string(), vec![]);
        pseudo_selectors.insert("placeholderShown".to_string(), vec![]);
        pseudo_selectors.insert("valid".to_string(), vec![]);
        pseudo_selectors.insert("invalid".to_string(), vec![]);
        pseudo_selectors.insert("required".to_string(), vec![]);
        pseudo_selectors.insert("optional".to_string(), vec![]);
        pseudo_selectors.insert("fullscreen".to_string(), vec![]);
        pseudo_selectors.insert("focusWithin".to_string(), vec![]);
        pseudo_selectors.insert("firstLine".to_string(), vec![]);
        pseudo_selectors.insert("firstLetter".to_string(), vec![]);
        pseudo_selectors.insert("before".to_string(), vec![]);
        pseudo_selectors.insert("after".to_string(), vec![]);
        pseudo_selectors.insert("outOfRange".to_string(), vec![]);
        pseudo_selectors.insert("root".to_string(), vec![]);
        pseudo_selectors.insert("firstPage".to_string(), vec![]);
        pseudo_selectors.insert("leftPage".to_string(), vec![]);
        pseudo_selectors.insert("rightPage".to_string(), vec![]);
        pseudo_selectors.insert("empty".to_string(), vec![]);

        ast.insert("pseudoSelectors".to_string(), pseudo_selectors);

        // media query variables declarations
        let mut media_query_variables = HashMap::new();

        media_query_variables.insert("minStandardSmartphones".to_string(), vec![]);
        media_query_variables.insert("minLargeSmartphones".to_string(), vec![]);
        media_query_variables.insert("minPortraitTablets".to_string(), vec![]);
        media_query_variables.insert("minStandardDesktops".to_string(), vec![]);
        media_query_variables.insert("minLargeDesktops".to_string(), vec![]);
        media_query_variables.insert("maxLargeDesktops".to_string(), vec![]);
        media_query_variables.insert("maxStandardDesktops".to_string(), vec![]);
        media_query_variables.insert("maxPortraitTablets".to_string(), vec![]);
        media_query_variables.insert("maxLargeSmartphones".to_string(), vec![]);
        media_query_variables.insert("maxStandardSmartphones".to_string(), vec![]);

        ast.insert("mediaQueryVariables".to_string(), media_query_variables);

        Mutex::new(ast)
    };
}
