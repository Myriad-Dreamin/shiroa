use std::num::NonZeroUsize;

use reflexo_typst::debug_loc::DocumentPosition;
use reflexo_typst::TypstDocument;
use serde::{Deserialize, Serialize};
use typst::foundations::{Content, NativeElement, Packed, StyleChain};
use typst::introspection::Introspector;
use typst::model::HeadingElem;
use typst::syntax::Span;

/// A heading in the outline panel.
#[derive(Debug, Clone)]
pub(crate) struct HeadingNode {
    body: Content,
    span: Span,
    position: DocumentPosition,
    level: NonZeroUsize,
    bookmarked: bool,
    children: Vec<HeadingNode>,
}

/// Construct the outline for the document.
pub(crate) fn get_outline(introspector: &Introspector) -> Option<Vec<HeadingNode>> {
    let mut tree: Vec<HeadingNode> = vec![];
    // Stores the level of the topmost skipped ancestor of the next bookmarked
    // heading. A skipped heading is a heading with 'bookmarked: false', that
    // is, it is not added to the PDF outline, and so is not in the tree.
    // Therefore, its next descendant must be added at its level, which is
    // enforced in the manner shown below.
    let mut last_skipped_level = None;
    let elements = introspector.query(&HeadingElem::elem().select());
    for elem in elements.iter() {
        let heading = elem.to_packed::<HeadingElem>().unwrap();
        let leaf = HeadingNode::leaf(introspector, heading);

        if leaf.bookmarked {
            let mut children = &mut tree;

            // Descend the tree through the latest bookmarked heading of each
            // level until either:
            // - you reach a node whose children would be brothers of this
            // heading (=> add the current heading as a child of this node);
            // - you reach a node with no children (=> this heading probably
            // skipped a few nesting levels in Typst, or one or more ancestors
            // of this heading weren't bookmarked, so add it as a child of this
            // node, which is its deepest bookmarked ancestor);
            // - or, if the latest heading(s) was(/were) skipped
            // ('bookmarked: false'), then stop if you reach a node whose
            // children would be brothers of the latest skipped heading
            // of lowest level (=> those skipped headings would be ancestors
            // of the current heading, so add it as a 'brother' of the least
            // deep skipped ancestor among them, as those ancestors weren't
            // added to the bookmark tree, and the current heading should not
            // be mistakenly added as a descendant of a brother of that
            // ancestor.)
            //
            // That is, if you had a bookmarked heading of level N, a skipped
            // heading of level N, a skipped heading of level N + 1, and then
            // a bookmarked heading of level N + 2, that last one is bookmarked
            // as a level N heading (taking the place of its topmost skipped
            // ancestor), so that it is not mistakenly added as a descendant of
            // the previous level N heading.
            //
            // In other words, a heading can be added to the bookmark tree
            // at most as deep as its topmost skipped direct ancestor (if it
            // exists), or at most as deep as its actual nesting level in Typst
            // (not exceeding whichever is the most restrictive depth limit
            // of those two).
            while children.last().is_some_and(|last| {
                last_skipped_level.map_or(true, |l| last.level < l) && last.level < leaf.level
            }) {
                children = &mut children.last_mut().unwrap().children;
            }

            // Since this heading was bookmarked, the next heading, if it is a
            // child of this one, won't have a skipped direct ancestor (indeed,
            // this heading would be its most direct ancestor, and wasn't
            // skipped). Therefore, it can be added as a child of this one, if
            // needed, following the usual rules listed above.
            last_skipped_level = None;
            children.push(leaf);
        } else if last_skipped_level.map_or(true, |l| leaf.level < l) {
            // Only the topmost / lowest-level skipped heading matters when you
            // have consecutive skipped headings (since none of them are being
            // added to the bookmark tree), hence the condition above.
            // This ensures the next bookmarked heading will be placed
            // at most as deep as its topmost skipped ancestors. Deeper
            // ancestors do not matter as the nesting structure they create
            // won't be visible in the PDF outline.
            last_skipped_level = Some(leaf.level);
        }
    }

    (!tree.is_empty()).then_some(tree)
}

impl HeadingNode {
    fn leaf(introspector: &Introspector, element: &Packed<HeadingElem>) -> Self {
        let position = {
            let loc = element.location().unwrap();
            let pos = introspector.position(loc);
            DocumentPosition {
                page_no: pos.page.into(),
                x: pos.point.x.to_pt() as f32,
                y: pos.point.y.to_pt() as f32,
            }
        };

        HeadingNode {
            level: element.resolve_level(StyleChain::default()),
            position,
            // 'bookmarked' set to 'auto' falls back to the value of 'outlined'.
            bookmarked: element
                .bookmarked(StyleChain::default())
                .unwrap_or_else(|| element.outlined(StyleChain::default())),
            body: element.body.clone(),
            span: element.span(),
            children: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Outline {
    pub items: Vec<OutlineItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutlineItem {
    /// Plain text title.
    pub title: String,
    /// Span id in hex-format.
    pub span: Option<String>,
    /// The resolved position in the document.
    pub position: Option<DocumentPosition>,
    /// The children of the outline item.
    pub children: Vec<OutlineItem>,
}

pub fn outline(interner: &mut SpanInternerImpl, document: &TypstDocument) -> Outline {
    let outline = get_outline(&document.introspector);
    let mut items = Vec::with_capacity(outline.as_ref().map_or(0, Vec::len));

    for heading in outline.iter().flatten() {
        outline_item(interner, heading, &mut items);
    }

    Outline { items }
}

fn outline_item(interner: &mut SpanInternerImpl, src: &HeadingNode, res: &mut Vec<OutlineItem>) {
    let body = src.body.clone();
    let title = body.plain_text().trim().to_owned();

    let mut children = Vec::with_capacity(src.children.len());
    for child in src.children.iter() {
        outline_item(interner, child, &mut children);
    }

    // use body's span first, otherwise use the element's span.
    let span = src.span;
    let span = if span.is_detached() {
        src.body.span()
    } else {
        span
    };

    let span = interner.intern(span);

    res.push(OutlineItem {
        title,
        span: Some(span.to_hex()),
        position: Some(src.position),
        children,
    });
}

// use std::num::NonZeroUsize;

// use serde::{Deserialize, Serialize};
// use typst::{
//     foundations::{Content, Smart},
//     introspection::Introspector,
// };
// use reflexo_typst::TypstDocument;

// use super::debug_loc::DocumentPosition;

// /// A heading in the outline panel.
// #[derive(Debug, Clone)]
// pub(crate) struct HeadingNode {
//     element: Content,
//     position: DocumentPosition,
//     level: NonZeroUsize,
//     bookmarked: bool,
//     children: Vec<HeadingNode>,
// }

// /// Construct the outline for the document.
// pub(crate) fn get_outline(introspector: &mut Introspector) ->
// Option<Vec<HeadingNode>> {     let mut tree: Vec<HeadingNode> = vec![];

//     // Stores the level of the topmost skipped ancestor of the next
// bookmarked     // heading. A skipped heading is a heading with 'bookmarked:
// false', that     // is, it is not added to the PDF outline, and so is not in
// the tree.     // Therefore, its next descendant must be added at its level,
// which is     // enforced in the manner shown below.
//     let mut last_skipped_level = None;
//     let selector =
// typst::eval::LANG_ITEMS.get().unwrap().heading_elem.select();     for heading
// in introspector.query(&selector).iter() {         let leaf =
// HeadingNode::leaf(introspector, (**heading).clone());

//         if leaf.bookmarked {
//             let mut children = &mut tree;

//             // Descend the tree through the latest bookmarked heading of each
//             // level until either:
//             // - you reach a node whose children would be brothers of this
//             // heading (=> add the current heading as a child of this node);
//             // - you reach a node with no children (=> this heading probably
//             // skipped a few nesting levels in Typst, or one or more
// ancestors             // of this heading weren't bookmarked, so add it as a
// child of this             // node, which is its deepest bookmarked ancestor);
//             // - or, if the latest heading(s) was(/were) skipped
//             // ('bookmarked: false'), then stop if you reach a node whose
//             // children would be brothers of the latest skipped heading
//             // of lowest level (=> those skipped headings would be ancestors
//             // of the current heading, so add it as a 'brother' of the least
//             // deep skipped ancestor among them, as those ancestors weren't
//             // added to the bookmark tree, and the current heading should not
//             // be mistakenly added as a descendant of a brother of that
//             // ancestor.)
//             //
//             // That is, if you had a bookmarked heading of level N, a skipped
//             // heading of level N, a skipped heading of level N + 1, and then
//             // a bookmarked heading of level N + 2, that last one is
// bookmarked             // as a level N heading (taking the place of its
// topmost skipped             // ancestor), so that it is not mistakenly added
// as a descendant of             // the previous level N heading.
//             //
//             // In other words, a heading can be added to the bookmark tree
//             // at most as deep as its topmost skipped direct ancestor (if it
//             // exists), or at most as deep as its actual nesting level in
// Typst             // (not exceeding whichever is the most restrictive depth
// limit             // of those two).
//             while children.last().map_or(false, |last| {
//                 last_skipped_level.map_or(true, |l| last.level < l) &&
// last.level < leaf.level             }) {
//                 children = &mut children.last_mut().unwrap().children;
//             }

//             // Since this heading was bookmarked, the next heading, if it is
// a             // child of this one, won't have a skipped direct ancestor
// (indeed,             // this heading would be its most direct ancestor, and
// wasn't             // skipped). Therefore, it can be added as a child of this
// one, if             // needed, following the usual rules listed above.
//             last_skipped_level = None;
//             children.push(leaf);
//         } else if last_skipped_level.map_or(true, |l| leaf.level < l) {
//             // Only the topmost / lowest-level skipped heading matters when
// you             // have consecutive skipped headings (since none of them are
// being             // added to the bookmark tree), hence the condition above.
//             // This ensures the next bookmarked heading will be placed
//             // at most as deep as its topmost skipped ancestors. Deeper
//             // ancestors do not matter as the nesting structure they create
//             // won't be visible in the PDF outline.
//             last_skipped_level = Some(leaf.level);
//         }
//     }

//     (!tree.is_empty()).then_some(tree)
// }

// impl HeadingNode {
//     fn leaf(introspector: &mut Introspector, element: Content) -> Self {
//         let position = {
//             let loc = element.location().unwrap();
//             introspector.position(loc).into()
//         };

//         HeadingNode {
//             level: element.expect_field::<NonZeroUsize>("level"),
//             position,
//             // 'bookmarked' set to 'auto' falls back to the value of
// 'outlined'.             bookmarked: element
//                 .expect_field::<Smart<bool>>("bookmarked")
//                 .unwrap_or_else(|| element.expect_field::<bool>("outlined")),
//             element,
//             children: Vec::new(),
//         }
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Outline {
//     pub items: Vec<OutlineItem>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct OutlineItem {
//     /// Plain text title.
//     pub title: String,
//     /// Span id in hex-format.
//     pub span: Option<String>,
//     /// The resolved position in the document.
//     pub position: Option<DocumentPosition>,
//     /// The children of the outline item.
//     pub children: Vec<OutlineItem>,
// }

// pub fn outline(document: &TypstDocument) -> Outline {
//     let mut introspector = Introspector::new(&document.pages);
//     let outline = get_outline(&mut introspector);
//     let mut items = Vec::with_capacity(outline.as_ref().map_or(0, Vec::len));

//     for heading in outline.iter().flatten() {
//         outline_item(heading, &mut items);
//     }

//     Outline { items }
// }

// fn outline_item(src: &HeadingNode, res: &mut Vec<OutlineItem>) {
//     let body = src.element.expect_field::<Content>("body");
//     let title = body.plain_text().trim().to_owned();

//     let mut children = Vec::with_capacity(src.children.len());
//     for child in src.children.iter() {
//         outline_item(child, &mut children);
//     }

//     // use body's span first, otherwise use the element's span.
//     let span = body.span();
//     let span = if span.is_detached() {
//         src.element.span()
//     } else {
//         span
//     };

//     res.push(OutlineItem {
//         title,
//         span: Some(format!("{:x}", span_id_to_u64(&span))),
//         position: Some(src.position),
//         children,
//     });
// }
use std::{ops::DerefMut, sync::Arc};

use indexmap::IndexSet;
use reflexo_typst::debug_loc::SourceSpan;
use tokio::sync::RwLock;

#[derive(Debug)]
pub enum InternQuery<T> {
    Ok(Option<T>),
    UseAfterFree,
}

pub struct InternId {
    lifetime: u32,
    id: u32,
}

impl InternId {
    pub fn new(lifetime: usize, id: usize) -> Self {
        Self {
            lifetime: lifetime as u32,
            id: id as u32,
        }
    }

    fn to_u64(&self) -> u64 {
        (self.lifetime as u64) << 32 | self.id as u64
    }

    fn from_u64(id: u64) -> Self {
        Self {
            lifetime: (id >> 32) as u32,
            id: (id & 0xffffffff) as u32,
        }
    }

    pub fn to_hex(&self) -> String {
        format!("{:x}", self.to_u64())
    }

    pub fn from_hex(hex: &str) -> Self {
        Self::from_u64(u64::from_str_radix(hex, 16).unwrap())
    }
}

/// Span interner
///
/// Interns spans and returns an intern id. Intern id can be converted to a
/// span. Clone of the interner is cheap, and the clone shares the same interned
/// spans.
#[derive(Clone, Default)]
pub struct SpanInterner {
    inner: Arc<RwLock<SpanInternerImpl>>,
}

impl SpanInterner {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(unused)]
    pub async fn reset(&self) {
        self.inner.write().await.reset();
    }

    pub async fn span_by_str(&self, str: &str) -> InternQuery<SourceSpan> {
        self.inner.read().await.span_by_str(str)
    }

    #[allow(unused)]
    pub async fn span(&self, id: InternId) -> InternQuery<SourceSpan> {
        self.inner.read().await.span(id)
    }

    #[allow(unused)]
    pub async fn intern(&self, span: SourceSpan) -> InternId {
        self.inner.write().await.intern(span)
    }

    pub async fn with_writer<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut SpanInternerImpl) -> R,
    {
        f(self.inner.write().await.deref_mut())
    }
}

pub struct SpanInternerImpl {
    lifetime: usize,
    span2id: IndexSet<(usize, SourceSpan)>,
}

impl Default for SpanInternerImpl {
    fn default() -> Self {
        Self::new()
    }
}

const GARAGE_COLLECT_THRESHOLD: usize = 30;

impl SpanInternerImpl {
    pub fn new() -> Self {
        Self {
            lifetime: 1,
            span2id: IndexSet::new(),
        }
    }

    pub fn reset(&mut self) {
        self.lifetime += 1;
        self.span2id
            .retain(|(id, _)| self.lifetime - id < GARAGE_COLLECT_THRESHOLD);
    }

    pub fn span_by_str(&self, str: &str) -> InternQuery<SourceSpan> {
        self.span(InternId::from_hex(str))
    }

    pub fn span(&self, id: InternId) -> InternQuery<SourceSpan> {
        if (id.lifetime as usize + GARAGE_COLLECT_THRESHOLD) <= self.lifetime {
            InternQuery::UseAfterFree
        } else {
            InternQuery::Ok(
                self.span2id
                    .get_index(id.id as usize)
                    .map(|(_, span)| span)
                    .copied(),
            )
        }
    }

    pub fn intern(&mut self, span: SourceSpan) -> InternId {
        let item = (self.lifetime, span);
        let (idx, _) = self.span2id.insert_full(item);
        // combine lifetime

        InternId::new(self.lifetime, idx)
    }
}
