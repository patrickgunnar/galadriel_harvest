use lazy_static::lazy_static;
use std::collections::HashMap;

// static CSS properties
lazy_static! {
    pub static ref STATIC_CORE: HashMap<String, HashMap<String, Vec<String>>> = {
        let mut map: HashMap<String, Vec<(String, String)> > = HashMap::new();

        // set CSS properties
        let mut display: HashMap<String, Vec<(String, String)> > = HashMap::new();

        display.insert("$panel-hidden".to_string(), vec![("display".to_string(), "none".to_string())]);
        display.insert("$panel-block".to_string(), vec![("display".to_string(), "block".to_string())]);
        display.insert("$panel-flex".to_string(), vec![("display".to_string(), "flex".to_string())]);
        display.insert("$panel-inline".to_string(), vec![("display".to_string(), "inline".to_string())]);
        display.insert("$panel-table".to_string(), vec![("display".to_string(), "table".to_string())]);
        display.insert("$panel-grid".to_string(), vec![("display".to_string(), "grid".to_string())]);
        display.insert("$panel-inline-block".to_string(), vec![("display".to_string(), "inline-block".to_string())]);
        display.insert("$panel-inline-flex".to_string(), vec![("display".to_string(), "inline-flex".to_string())]);
        display.insert("$panel-inline-table".to_string(), vec![("display".to_string(), "inline-table".to_string())]);
        display.insert("$panel-inline-grid".to_string(), vec![("display".to_string(), "inline-grid".to_string())]);
        display.insert("$panel-flow-root".to_string(), vec![("display".to_string(), "flow-root".to_string())]);
        display.insert("$panel-contents".to_string(), vec![("display".to_string(), "contents".to_string())]);
        display.insert("$panel-list-item".to_string(), vec![("display".to_string(), "list-item".to_string())]);
        display.insert("$panel-header-group".to_string(), vec![("display".to_string(), "table-header-group".to_string())]);
        display.insert("$panel-footer-group".to_string(), vec![("display".to_string(), "table-footer-group".to_string())]);
        display.insert("$panel-column-group".to_string(), vec![("display".to_string(), "table-column-group".to_string())]);
        display.insert("$panel-row-group".to_string(), vec![("display".to_string(), "table-row-group".to_string())]);
        display.insert("$panel-table-row".to_string(), vec![("display".to_string(), "table-row".to_string())]);
        display.insert("$panel-table-cell".to_string(), vec![("display".to_string(), "table-cell".to_string())]);
        display.insert("$panel-table-column".to_string(), vec![("display".to_string(), "table-column".to_string())]);
        display.insert("$panel-table-caption".to_string(), vec![("display".to_string(), "table-caption".to_string())]);

        map.insert("display".to_string(), display);

        let mut position: HashMap<String, Vec<(String, String)> > = HashMap::new();

        position.insert("$set-relative".to_string(), vec![("position".to_string(), "relative".to_string())]);
        position.insert("$set-absolute".to_string(), vec![("position".to_string(), "absolute".to_string())]);
        position.insert("$set-static".to_string(), vec![("position".to_string(), "static".to_string())]);
        position.insert("$set-fixed".to_string(), vec![("position".to_string(), "fixed".to_string())]);
        position.insert("$set-sticky".to_string(), vec![("position".to_string(), "sticky".to_string())]);

        map.insert("position".to_string(), position);

        let mut float: HashMap<String, Vec<(String, String)>> = HashMap::new();

        float.insert("$levitate-left".to_string(), vec![("float".to_string(), "left".to_string())]);
        float.insert("$levitate-right".to_string(), vec![("float".to_string(), "right".to_string())]);
        float.insert("$levitate-none".to_string(), vec![("float".to_string(), "none".to_string())]);

        map.insert("float".to_string(), float);

        let mut visibility: HashMap<String, Vec<(String, String)>> = HashMap::new();

        visibility.insert("$exposure-visible".to_string(), vec![("visibility".to_string(), "visible".to_string())]);
        visibility.insert("$exposure-hidden".to_string(), vec![("visibility".to_string(), "hidden".to_string())]);
        visibility.insert("$exposure-collapse".to_string(), vec![("visibility".to_string(), "collapse".to_string())]);

        map.insert("visibility".to_string(), visibility);

        let mut clear: HashMap<String, Vec<(String, String)>> = HashMap::new();

        clear.insert("$plain-left".to_string(), vec![("clear".to_string(), "left".to_string())]);
        clear.insert("$plain-right".to_string(), vec![("clear".to_string(), "right".to_string())]);
        clear.insert("$plain-both".to_string(), vec![("clear".to_string(), "both".to_string())]);
        clear.insert("$plain-none".to_string(), vec![("clear".to_string(), "none".to_string())]);

        map.insert("clear".to_string(), clear);

        let mut overflow: HashMap<String, Vec<(String, String)>> = HashMap::new();

        overflow.insert("$excess-visible".to_string(), vec![("overflow".to_string(), "visible".to_string())]);
        overflow.insert("$excess-hidden".to_string(), vec![("overflow".to_string(), "hidden".to_string())]);
        overflow.insert("$excess-scroll".to_string(), vec![("overflow".to_string(), "scroll".to_string())]);
        overflow.insert("$excess-auto".to_string(), vec![("overflow".to_string(), "auto".to_string())]);

        map.insert("overflow".to_string(), overflow);

        let mut overflow_y: HashMap<String, Vec<(String, String)>> = HashMap::new();

        overflow_y.insert("$excess-y-visible".to_string(), vec![("overflow-y".to_string(), "visible".to_string())]);
        overflow_y.insert("$excess-y-hidden".to_string(), vec![("overflow-y".to_string(), "hidden".to_string())]);
        overflow_y.insert("$excess-y-scroll".to_string(), vec![("overflow-y".to_string(), "scroll".to_string())]);
        overflow_y.insert("$excess-y-auto".to_string(), vec![("overflow-y".to_string(), "auto".to_string())]);
        overflow_y.insert("$excess-y-clip".to_string(), vec![("overflow-y".to_string(), "clip".to_string())]);

        map.insert("overflowY".to_string(), overflow_y);

        let mut overflow_x: HashMap<String, Vec<(String, String)>> = HashMap::new();

        overflow_x.insert("$excess-x-visible".to_string(), vec![("overflow-x".to_string(), "visible".to_string())]);
        overflow_x.insert("$excess-x-hidden".to_string(), vec![("overflow-x".to_string(), "hidden".to_string())]);
        overflow_x.insert("$excess-x-scroll".to_string(), vec![("overflow-x".to_string(), "scroll".to_string())]);
        overflow_x.insert("$excess-x-auto".to_string(), vec![("overflow-x".to_string(), "auto".to_string())]);
        overflow_x.insert("$excess-x-clip".to_string(), vec![("overflow-x".to_string(), "clip".to_string())]);

        map.insert("overflowX".to_string(), overflow_x);

        let mut overflow_wrap: HashMap<String, Vec<(String, String)>> = HashMap::new();

        overflow_wrap.insert("$excess-wrap-normal".to_string(), vec![("overflow-wrap".to_string(), "normal".to_string())]);
        overflow_wrap.insert("$excess-wrap-break-word".to_string(), vec![("overflow-wrap".to_string(), "break-word".to_string())]);

        map.insert("overflowWrap".to_string(), overflow_wrap);

        let mut white_space: HashMap<String, Vec<(String, String)>> = HashMap::new();

        white_space.insert("$white-field-normal".to_string(), vec![("white-space".to_string(), "normal".to_string())]);
        white_space.insert("$white-field-nowrap".to_string(), vec![("white-space".to_string(), "nowrap".to_string())]);
        white_space.insert("$white-field-pre".to_string(), vec![("white-space".to_string(), "pre".to_string())]);
        white_space.insert("$white-field-break-spaces".to_string(), vec![("white-space".to_string(), "break-spaces".to_string())]);
        white_space.insert("$white-field-pre-line".to_string(), vec![("white-space".to_string(), "pre-line".to_string())]);
        white_space.insert("$white-field-pre-wrap".to_string(), vec![("white-space".to_string(), "pre-wrap".to_string())]);

        map.insert("whiteSpace".to_string(), white_space);

        let mut list_style_type: HashMap<String, Vec<(String, String)>> = HashMap::new();

        list_style_type.insert("$series-style-none".to_string(), vec![("list-style-type".to_string(), "none".to_string())]);
        list_style_type.insert("$series-style-disc".to_string(), vec![("list-style-type".to_string(), "disc".to_string())]);
        list_style_type.insert("$series-style-circle".to_string(), vec![("list-style-type".to_string(), "circle".to_string())]);
        list_style_type.insert("$series-style-square".to_string(), vec![("list-style-type".to_string(), "square".to_string())]);

        map.insert("listStyleType".to_string(), list_style_type);

        let mut text_align: HashMap<String, Vec<(String, String)>> = HashMap::new();

        text_align.insert("$arrange-text-left".to_string(), vec![("text-align".to_string(), "left".to_string())]);
        text_align.insert("$arrange-text-right".to_string(), vec![("text-align".to_string(), "right".to_string())]);
        text_align.insert("$arrange-text-center".to_string(), vec![("text-align".to_string(), "center".to_string())]);
        text_align.insert("$arrange-text-justify".to_string(), vec![("text-align".to_string(), "justify".to_string())]);

        map.insert("textAlign".to_string(), text_align);

        let mut vertical_align: HashMap<String, Vec<(String, String)>> = HashMap::new();

        vertical_align.insert("$set-vertical-baseline".to_string(), vec![("vertical-align".to_string(), "baseline".to_string())]);
        vertical_align.insert("$set-vertical-top".to_string(), vec![("vertical-align".to_string(), "top".to_string())]);
        vertical_align.insert("$set-vertical-middle".to_string(), vec![("vertical-align".to_string(), "middle".to_string())]);
        vertical_align.insert("$set-vertical-bottom".to_string(), vec![("vertical-align".to_string(), "bottom".to_string())]);

        map.insert("verticalAlign".to_string(), vertical_align);

        let mut word_break: HashMap<String, Vec<(String, String)>> = HashMap::new();

        word_break.insert("$word-rupture-normal".to_string(), vec![("word-break".to_string(), "normal".to_string())]);
        word_break.insert("$word-rupture-break-all".to_string(), vec![("word-break".to_string(), "break-all".to_string())]);

        map.insert("wordBreak".to_string(), word_break);

        let mut font_weight: HashMap<String, Vec<(String, String)>> = HashMap::new();

        font_weight.insert("$font-density-normal".to_string(), vec![("font-weight".to_string(), "normal".to_string())]);
        font_weight.insert("$font-density-bold".to_string(), vec![("font-weight".to_string(), "bold".to_string())]);
        font_weight.insert("$font-density-lighter".to_string(), vec![("font-weight".to_string(), "lighter".to_string())]);
        font_weight.insert("$font-density-bolder".to_string(), vec![("font-weight".to_string(), "bolder".to_string())]);
        font_weight.insert("$font-density-100".to_string(), vec![("font-weight".to_string(), "100".to_string())]);
        font_weight.insert("$font-density-200".to_string(), vec![("font-weight".to_string(), "200".to_string())]);
        font_weight.insert("$font-density-300".to_string(), vec![("font-weight".to_string(), "300".to_string())]);
        font_weight.insert("$font-density-400".to_string(), vec![("font-weight".to_string(), "400".to_string())]);
        font_weight.insert("$font-density-500".to_string(), vec![("font-weight".to_string(), "500".to_string())]);
        font_weight.insert("$font-density-600".to_string(), vec![("font-weight".to_string(), "600".to_string())]);
        font_weight.insert("$font-density-700".to_string(), vec![("font-weight".to_string(), "700".to_string())]);
        font_weight.insert("$font-density-800".to_string(), vec![("font-weight".to_string(), "800".to_string())]);
        font_weight.insert("$font-density-900".to_string(), vec![("font-weight".to_string(), "900".to_string())]);

        map.insert("fontWeight".to_string(), font_weight);

        let mut text_decoration: HashMap<String, Vec<(String, String)>> = HashMap::new();

        text_decoration.insert("$text-dressing-none".to_string(), vec![("text-decoration".to_string(), "none".to_string())]);
        text_decoration.insert("$text-dressing-underline".to_string(), vec![("text-decoration".to_string(), "underline".to_string())]);
        text_decoration.insert("$text-dressing-overline".to_string(), vec![("text-decoration".to_string(), "overline".to_string())]);
        text_decoration.insert("$text-dressing-line-through".to_string(), vec![("text-decoration".to_string(), "line-through".to_string())]);

        map.insert("textDecoration".to_string(), text_decoration);

        let mut box_sizing: HashMap<String, Vec<(String, String)> > = HashMap::new();

        box_sizing.insert("$box-scale-content-box".to_string(), vec![("box-sizing".to_string(), "content-box".to_string())]);
        box_sizing.insert("$box-scale-border-box".to_string(), vec![("box-sizing".to_string(), "border-box".to_string())]);
        box_sizing.insert("$box-scale-inherit".to_string(), vec![("box-sizing".to_string(), "inherit".to_string())]);
        box_sizing.insert("$box-scale-initial".to_string(), vec![("box-sizing".to_string(), "initial".to_string())]);
        box_sizing.insert("$box-scale-unset".to_string(), vec![("box-sizing".to_string(), "unset".to_string())]);

        map.insert("boxSizing".to_string(), box_sizing);

        let mut cursor: HashMap<String, Vec<(String, String)>> = HashMap::new();

        cursor.insert("$controller-default".to_string(), vec![("cursor".to_string(), "default".to_string())]);
        cursor.insert("$controller-auto".to_string(), vec![("cursor".to_string(), "auto".to_string())]);
        cursor.insert("$controller-pointer".to_string(), vec![("cursor".to_string(), "pointer".to_string())]);
        cursor.insert("$controller-text".to_string(), vec![("cursor".to_string(), "text".to_string())]);
        cursor.insert("$controller-move".to_string(), vec![("cursor".to_string(), "move".to_string())]);
        cursor.insert("$controller-wait".to_string(), vec![("cursor".to_string(), "wait".to_string())]);
        cursor.insert("$controller-not-allowed".to_string(), vec![("cursor".to_string(), "not-allowed".to_string())]);
        cursor.insert("$controller-help".to_string(), vec![("cursor".to_string(), "help".to_string())]);
        cursor.insert("$controller-crosshair".to_string(), vec![("cursor".to_string(), "crosshair".to_string())]);
        cursor.insert("$controller-zoom-in".to_string(), vec![("cursor".to_string(), "zoom-in".to_string())]);
        cursor.insert("$controller-zoom-out".to_string(), vec![("cursor".to_string(), "zoom-out".to_string())]);
        cursor.insert("$controller-grab".to_string(), vec![("cursor".to_string(), "grab".to_string())]);

        map.insert("cursor".to_string(), cursor);

        let mut pointer_events: HashMap<String, Vec<(String, String)>> = HashMap::new();

        pointer_events.insert("$event-indicator-auto".to_string(), vec![("pointer-events".to_string(), "auto".to_string())]);
        pointer_events.insert("$event-indicator-none".to_string(), vec![("pointer-events".to_string(), "none".to_string())]);

        map.insert("pointerEvents".to_string(), pointer_events);

        let mut outline_style: HashMap<String, Vec<(String, String)>> = HashMap::new();

        outline_style.insert("$outline-mode-none".to_string(), vec![("outline-style".to_string(), "none".to_string())]);
        outline_style.insert("$outline-mode-auto".to_string(), vec![("outline-style".to_string(), "auto".to_string())]);
        outline_style.insert("$outline-mode-dotted".to_string(), vec![("outline-style".to_string(), "dotted".to_string())]);
        outline_style.insert("$outline-mode-dashed".to_string(), vec![("outline-style".to_string(), "dashed".to_string())]);
        outline_style.insert("$outline-mode-solid".to_string(), vec![("outline-style".to_string(), "solid".to_string())]);
        outline_style.insert("$outline-mode-double".to_string(), vec![("outline-style".to_string(), "double".to_string())]);
        outline_style.insert("$outline-mode-groove".to_string(), vec![("outline-style".to_string(), "groove".to_string())]);
        outline_style.insert("$outline-mode-ridge".to_string(), vec![("outline-style".to_string(), "ridge".to_string())]);
        outline_style.insert("$outline-mode-inset".to_string(), vec![("outline-style".to_string(), "inset".to_string())]);
        outline_style.insert("$outline-mode-outset".to_string(), vec![("outline-style".to_string(), "outset".to_string())]);

        map.insert("outlineStyles".to_string(), outline_style);

        let mut box_shadow: HashMap<String, Vec<(String, String)>> = HashMap::new();

        box_shadow.insert("$container-shadow-none".to_string(), vec![("box-shadow".to_string(), "none".to_string())]);

        map.insert("boxShadow".to_string(), box_shadow);

        let mut text_transform: HashMap<String, Vec<(String, String)> > = HashMap::new();

        text_transform.insert("$text-mutate-none".to_string(), vec![("text-transform".to_string(), "none".to_string())]);
        text_transform.insert("$text-mutate-uppercase".to_string(), vec![("text-transform".to_string(), "uppercase".to_string())]);
        text_transform.insert("$text-mutate-lowercase".to_string(), vec![("text-transform".to_string(), "lowercase".to_string())]);
        text_transform.insert("$text-mutate-capitalize".to_string(), vec![("text-transform".to_string(), "capitalize".to_string())]);

        map.insert("textTransform".to_string(), text_transform);

        let mut transition_property: HashMap<String, Vec<(String, String)>> = HashMap::new();

        transition_property.insert("$transition-state-all".to_string(), vec![("transition-property".to_string(), "all".to_string())]);
        transition_property.insert("$transition-state-none".to_string(), vec![("transition-property".to_string(), "none".to_string())]);

        map.insert("transitionProperty".to_string(), transition_property);

        let mut transition_timing: HashMap<String, Vec<(String, String)>> = HashMap::new();

        transition_timing.insert("$passage-timing-ease".to_string(), vec![("transition-timing-function".to_string(), "ease".to_string())]);
        transition_timing.insert("$passage-timing-linear".to_string(), vec![("transition-timing-function".to_string(), "linear".to_string())]);
        transition_timing.insert("$passage-timing-ease-in".to_string(), vec![("transition-timing-function".to_string(), "ease-in".to_string())]);
        transition_timing.insert("$passage-timing-ease-out".to_string(), vec![("transition-timing-function".to_string(), "ease-out".to_string())]);
        transition_timing.insert("$passage-timing-ease-in-out".to_string(), vec![("transition-timing-function".to_string(), "ease-in-out".to_string())]);
        transition_timing.insert("$passage-timing-step-start".to_string(), vec![("transition-timing-function".to_string(), "step-start".to_string())]);
        transition_timing.insert("$passage-timing-step-end".to_string(), vec![("transition-timing-function".to_string(), "step-end".to_string())]);

        map.insert("transitionTimingFunction".to_string(), transition_timing);

        let mut flex_direction: HashMap<String, Vec<(String, String)>> = HashMap::new();

        flex_direction.insert("$flex-orientation-row".to_string(), vec![("flex-direction".to_string(), "row".to_string())]);
        flex_direction.insert("$flex-orientation-row-reverse".to_string(), vec![("flex-direction".to_string(), "row-reverse".to_string())]);
        flex_direction.insert("$flex-orientation-column".to_string(), vec![("flex-direction".to_string(), "column".to_string())]);
        flex_direction.insert("$flex-orientation-column-reverse".to_string(), vec![("flex-direction".to_string(), "column-reverse".to_string())]);

        map.insert("flexDirection".to_string(), flex_direction);

        let mut flex_wrap: HashMap<String, Vec<(String, String)>> = HashMap::new();

        flex_wrap.insert("$flex-enclose-nowrap".to_string(), vec![("flex-wrap".to_string(), "nowrap".to_string())]);
        flex_wrap.insert("$flex-enclose-wrap".to_string(), vec![("flex-wrap".to_string(), "wrap".to_string())]);
        flex_wrap.insert("$flex-enclose-wrap-reverse".to_string(), vec![("flex-wrap".to_string(), "wrap-reverse".to_string())]);

        map.insert("flexWrap".to_string(), flex_wrap);

        let mut justify_content: HashMap<String, Vec<(String, String)>> = HashMap::new();

        justify_content.insert("$organize-content-center".to_string(), vec![("justify-content".to_string(), "center".to_string())]);
        justify_content.insert("$organize-content-flex-start".to_string(), vec![("justify-content".to_string(), "flex-start".to_string())]);
        justify_content.insert("$organize-content-flex-end".to_string(), vec![("justify-content".to_string(), "flex-end".to_string())]);
        justify_content.insert("$organize-content-space-between".to_string(), vec![("justify-content".to_string(), "space-between".to_string())]);
        justify_content.insert("$organize-content-space-around".to_string(), vec![("justify-content".to_string(), "space-around".to_string())]);
        justify_content.insert("$organize-content-space-evenly".to_string(), vec![("justify-content".to_string(), "space-evenly".to_string())]);
        justify_content.insert("$organize-content-normal".to_string(), vec![("justify-content".to_string(), "normal".to_string())]);
        justify_content.insert("$organize-content-start".to_string(), vec![("justify-content".to_string(), "start".to_string())]);
        justify_content.insert("$organize-content-end".to_string(), vec![("justify-content".to_string(), "end".to_string())]);
        justify_content.insert("$organize-content-stretch".to_string(), vec![("justify-content".to_string(), "stretch".to_string())]);
        justify_content.insert("$organize-content-left".to_string(), vec![("justify-content".to_string(), "left".to_string())]);
        justify_content.insert("$organize-content-right".to_string(), vec![("justify-content".to_string(), "right".to_string())]);

        map.insert("justifyContent".to_string(), justify_content);

        let mut justify_self: HashMap<String, Vec<(String, String)>> = HashMap::new();

        justify_self.insert("$organize-self-center".to_string(), vec![("justify-self".to_string(), "center".to_string())]);
        justify_self.insert("$organize-self-flex-start".to_string(), vec![("justify-self".to_string(), "flex-start".to_string())]);
        justify_self.insert("$organize-self-flex-end".to_string(), vec![("justify-self".to_string(), "flex-end".to_string())]);
        justify_self.insert("$organize-self-self-start".to_string(), vec![("justify-self".to_string(), "self-start".to_string())]);
        justify_self.insert("$organize-self-self-end".to_string(), vec![("justify-self".to_string(), "self-end".to_string())]);
        justify_self.insert("$organize-self-normal".to_string(), vec![("justify-self".to_string(), "normal".to_string())]);
        justify_self.insert("$organize-self-start".to_string(), vec![("justify-self".to_string(), "start".to_string())]);
        justify_self.insert("$organize-self-end".to_string(), vec![("justify-self".to_string(), "end".to_string())]);
        justify_self.insert("$organize-self-stretch".to_string(), vec![("justify-self".to_string(), "stretch".to_string())]);
        justify_self.insert("$organize-self-left".to_string(), vec![("justify-self".to_string(), "left".to_string())]);
        justify_self.insert("$organize-self-right".to_string(), vec![("justify-self".to_string(), "right".to_string())]);
        justify_self.insert("$organize-self-auto".to_string(), vec![("justify-self".to_string(), "auto".to_string())]);
        justify_self.insert("$organize-self-baseline".to_string(), vec![("justify-self".to_string(), "baseline".to_string())]);

        map.insert("justifySelf".to_string(), justify_self);

        let mut justify_items: HashMap<String, Vec<(String, String)>> = HashMap::new();

        justify_items.insert("$organize-items-center".to_string(), vec![("justify-items".to_string(), "center".to_string())]);
        justify_items.insert("$organize-items-flex-start".to_string(), vec![("justify-items".to_string(), "flex-start".to_string())]);
        justify_items.insert("$organize-items-flex-end".to_string(), vec![("justify-items".to_string(), "flex-end".to_string())]);
        justify_items.insert("$organize-items-self-start".to_string(), vec![("justify-items".to_string(), "self-start".to_string())]);
        justify_items.insert("$organize-items-self-end".to_string(), vec![("justify-items".to_string(), "self-end".to_string())]);
        justify_items.insert("$organize-items-normal".to_string(), vec![("justify-items".to_string(), "normal".to_string())]);
        justify_items.insert("$organize-items-start".to_string(), vec![("justify-items".to_string(), "start".to_string())]);
        justify_items.insert("$organize-items-end".to_string(), vec![("justify-items".to_string(), "end".to_string())]);
        justify_items.insert("$organize-items-stretch".to_string(), vec![("justify-items".to_string(), "stretch".to_string())]);
        justify_items.insert("$organize-items-left".to_string(), vec![("justify-items".to_string(), "left".to_string())]);
        justify_items.insert("$organize-items-right".to_string(), vec![("justify-items".to_string(), "right".to_string())]);
        justify_items.insert("$organize-items-baseline".to_string(), vec![("justify-items".to_string(), "baseline".to_string())]);

        map.insert("justifyItems".to_string(), justify_items);

        let mut align_items: HashMap<String, Vec<(String, String)>> = HashMap::new();

        align_items.insert("$adjust-center".to_string(), vec![("align-items".to_string(), "center".to_string())]);
        align_items.insert("$adjust-flex-start".to_string(), vec![("align-items".to_string(), "flex-start".to_string())]);
        align_items.insert("$adjust-flex-end".to_string(), vec![("align-items".to_string(), "flex-end".to_string())]);
        align_items.insert("$adjust-stretch".to_string(), vec![("align-items".to_string(), "stretch".to_string())]);
        align_items.insert("$adjust-baseline".to_string(), vec![("align-items".to_string(), "baseline".to_string())]);
        align_items.insert("$adjust-normal".to_string(), vec![("align-items".to_string(), "normal".to_string())]);
        align_items.insert("$adjust-start".to_string(), vec![("align-items".to_string(), "start".to_string())]);
        align_items.insert("$adjust-end".to_string(), vec![("align-items".to_string(), "end".to_string())]);
        align_items.insert("$adjust-self-start".to_string(), vec![("align-items".to_string(), "self-start".to_string())]);
        align_items.insert("$adjust-self-end".to_string(), vec![("align-items".to_string(), "self-end".to_string())]);

        map.insert("alignItems".to_string(), align_items);

        let mut align_self: HashMap<String, Vec<(String, String)>> = HashMap::new();

        align_self.insert("$place-self-auto".to_string(), vec![("align-self".to_string(), "auto".to_string())]);
        align_self.insert("$place-self-flex-start".to_string(), vec![("align-self".to_string(), "flex-start".to_string())]);
        align_self.insert("$place-self-flex-end".to_string(), vec![("align-self".to_string(), "flex-end".to_string())]);
        align_self.insert("$place-self-center".to_string(), vec![("align-self".to_string(), "center".to_string())]);
        align_self.insert("$place-self-baseline".to_string(), vec![("align-self".to_string(), "baseline".to_string())]);
        align_self.insert("$place-self-stretch".to_string(), vec![("align-self".to_string(), "stretch".to_string())]);
        align_self.insert("$place-self-normal".to_string(), vec![("align-self".to_string(), "normal".to_string())]);
        align_self.insert("$place-self-start".to_string(), vec![("align-self".to_string(), "start".to_string())]);
        align_self.insert("$place-self-end".to_string(), vec![("align-self".to_string(), "end".to_string())]);
        align_self.insert("$place-self-self-start".to_string(), vec![("align-self".to_string(), "self-start".to_string())]);
        align_self.insert("$place-self-self-end".to_string(), vec![("align-self".to_string(), "self-end".to_string())]);

        map.insert("alignSelf".to_string(), align_self);

        let mut align_content: HashMap<String, Vec<(String, String)>> = HashMap::new();

        align_content.insert("$match-content-stretch".to_string(), vec![("align-content".to_string(), "stretch".to_string())]);
        align_content.insert("$match-content-flex-start".to_string(), vec![("align-content".to_string(), "flex-start".to_string())]);
        align_content.insert("$match-content-flex-end".to_string(), vec![("align-content".to_string(), "flex-end".to_string())]);
        align_content.insert("$match-content-center".to_string(), vec![("align-content".to_string(), "center".to_string())]);
        align_content.insert("$match-content-space-between".to_string(), vec![("align-content".to_string(), "space-between".to_string())]);
        align_content.insert("$match-content-space-around".to_string(), vec![("align-content".to_string(), "space-around".to_string())]);
        align_content.insert("$match-content-space-evenly".to_string(), vec![("align-content".to_string(), "space-evenly".to_string())]);
        align_content.insert("$match-content-start".to_string(), vec![("align-content".to_string(), "start".to_string())]);
        align_content.insert("$match-content-end".to_string(), vec![("align-content".to_string(), "end".to_string())]);
        align_content.insert("$match-content-normal".to_string(), vec![("align-content".to_string(), "normal".to_string())]);
        align_content.insert("$match-content-baseline".to_string(), vec![("align-content".to_string(), "baseline".to_string())]);

        map.insert("alignContent".to_string(), align_content);

        let mut text_justify: HashMap<String, Vec<(String, String)> > = HashMap::new();

        text_justify.insert("$text-balance-none".to_string(), vec![("text-justify".to_string(), "none".to_string())]);
        text_justify.insert("$text-balance-auto".to_string(), vec![("text-justify".to_string(), "auto".to_string())]);
        text_justify.insert("$text-balance-inter-word".to_string(), vec![("text-justify".to_string(), "inter-word".to_string())]);
        text_justify.insert("$text-balance-inter-character".to_string(), vec![("text-justify".to_string(), "inter-character".to_string())]);

        map.insert("textJustify".to_string(), text_justify);

        let mut text_overflow: HashMap<String, Vec<(String, String)>> = HashMap::new();

        text_overflow.insert("$text-exceed-ellipsis".to_string(), vec![("text-overflow".to_string(), "ellipsis".to_string())]);
        text_overflow.insert("$text-exceed-clip".to_string(), vec![("text-overflow".to_string(), "clip".to_string())]);

        map.insert("textOverflow".to_string(), text_overflow);

        let mut box_decoration_break: HashMap<String, Vec<(String, String)>> = HashMap::new();

        box_decoration_break.insert("$box-ornament-break-slice".to_string(), vec![("box-decoration-break".to_string(), "slice".to_string())]);
        box_decoration_break.insert("$box-ornament-break-clone".to_string(), vec![("box-decoration-break".to_string(), "clone".to_string())]);

        map.insert("boxDecorationBreak".to_string(), box_decoration_break);

        let mut table_layout: HashMap<String, Vec<(String, String)>> = HashMap::new();

        table_layout.insert("$table-scheme-auto".to_string(), vec![("table-layout".to_string(), "auto".to_string())]);
        table_layout.insert("$table-scheme-fixed".to_string(), vec![("table-layout".to_string(), "fixed".to_string())]);

        map.insert("tableLayout".to_string(), table_layout);

        let mut caption_side: HashMap<String, Vec<(String, String)>> = HashMap::new();

        caption_side.insert("$caption-facet-top".to_string(), vec![("caption-side".to_string(), "top".to_string())]);
        caption_side.insert("$caption-facet-bottom".to_string(), vec![("caption-side".to_string(), "bottom".to_string())]);
        caption_side.insert("$caption-facet-block-start".to_string(), vec![("caption-side".to_string(), "block-start".to_string())]);
        caption_side.insert("$caption-facet-block-end".to_string(), vec![("caption-side".to_string(), "block-end".to_string())]);
        caption_side.insert("$caption-facet-inline-start".to_string(), vec![("caption-side".to_string(), "inline-start".to_string())]);
        caption_side.insert("$caption-facet-inline-end".to_string(), vec![("caption-side".to_string(), "inline-end".to_string())]);

        map.insert("captionSide".to_string(), caption_side);

        let mut quotes: HashMap<String, Vec<(String, String)>> = HashMap::new();

        quotes.insert("$quotation-auto".to_string(), vec![("quotes".to_string(), "auto".to_string())]);
        quotes.insert("$quotation-none".to_string(), vec![("quotes".to_string(), "none".to_string())]);
        quotes.insert("$quotation-french-marks".to_string(), vec![("quotes".to_string(), "'«' '»'".to_string())]);
        quotes.insert("$quotation-french-marks-guillemet-marks".to_string(), vec![("quotes".to_string(), "'«' '»' '‹' '›'".to_string())]);

        map.insert("quotes".to_string(), quotes);

        let mut column_count: HashMap<String, Vec<(String, String)>> = HashMap::new();

        column_count.insert("$tower-count-auto".to_string(), vec![("column-count".to_string(), "auto".to_string())]);

        map.insert("columnCount".to_string(), column_count);

        let mut column_gap: HashMap<String, Vec<(String, String)>> = HashMap::new();

        column_gap.insert("$pedestal-gap-normal".to_string(), vec![("column-gap".to_string(), "normal".to_string())]);

        map.insert("columnGap".to_string(), column_gap);

        let mut aspect_ratio: HashMap<String, Vec<(String, String)>> = HashMap::new();

        aspect_ratio.insert("$proportion-square".to_string(), vec![("aspect-ratio".to_string(), "1/1".to_string())]);
        aspect_ratio.insert("$proportion-auto".to_string(), vec![("aspect-ratio".to_string(), "auto".to_string())]);
        aspect_ratio.insert("$proportion-landscape".to_string(), vec![("aspect-ratio".to_string(), "16/9".to_string())]);
        aspect_ratio.insert("$proportion-portrait".to_string(), vec![("aspect-ratio".to_string(), "4/3".to_string())]);
        aspect_ratio.insert("$proportion-traditional-photo".to_string(), vec![("aspect-ratio".to_string(), "3/2".to_string())]);

        map.insert("aspectRatio".to_string(), aspect_ratio);

        let mut object_position: HashMap<String, Vec<(String, String)>> = HashMap::new();

        object_position.insert("$object-spot-top".to_string(), vec![("object-position".to_string(), "top".to_string())]);
        object_position.insert("$object-spot-bottom".to_string(), vec![("object-position".to_string(), "bottom".to_string())]);
        object_position.insert("$object-spot-left".to_string(), vec![("object-position".to_string(), "left".to_string())]);
        object_position.insert("$object-spot-right".to_string(), vec![("object-position".to_string(), "right".to_string())]);
        object_position.insert("$object-spot-center".to_string(), vec![("object-position".to_string(), "center".to_string())]);

        map.insert("objectPosition".to_string(), object_position);

        let mut object_fit: HashMap<String, Vec<(String, String)>> = HashMap::new();

        object_fit.insert("$object-blend-contain".to_string(), vec![("object-fit".to_string(), "contain".to_string())]);
        object_fit.insert("$object-blend-cover".to_string(), vec![("object-fit".to_string(), "cover".to_string())]);
        object_fit.insert("$object-blend-fill".to_string(), vec![("object-fit".to_string(), "fill".to_string())]);
        object_fit.insert("$object-blend-none".to_string(), vec![("object-fit".to_string(), "none".to_string())]);
        object_fit.insert("$object-blend-scale-down".to_string(), vec![("object-fit".to_string(), "scale-down".to_string())]);

        map.insert("objectFit".to_string(), object_fit);

        map
    };
}
