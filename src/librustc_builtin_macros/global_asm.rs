//! Module-level assembly support.
//!
//! The macro defined here allows you to specify "top-level",
//! "file-scoped", or "module-level" assembly. These synonyms
//! all correspond to LLVM's module-level inline assembly instruction.
//!
//! For example, `global_asm!("some assembly here")` codegens to
//! LLVM's `module asm "some assembly here"`. All of LLVM's caveats
//! therefore apply.

use rustc_ast::ast;
use rustc_ast::ptr::P;
use rustc_ast::token;
use rustc_ast::tokenstream::TokenStream;
use rustc_errors::DiagnosticBuilder;
use rustc_expand::base::{self, *};
use rustc_span::source_map::respan;
use rustc_span::Span;
use smallvec::smallvec;

pub fn expand_global_asm<'cx>(
    cx: &'cx mut ExtCtxt<'_>,
    sp: Span,
    tts: TokenStream,
) -> Box<dyn base::MacResult + 'cx> {
    match parse_global_asm(cx, sp, tts) {
        Ok(Some(global_asm)) => MacEager::items(smallvec![P(ast::Item {
            ident: ast::Ident::invalid(),
            attrs: Vec::new(),
            id: ast::DUMMY_NODE_ID,
            kind: ast::ItemKind::GlobalAsm(P(global_asm)),
            vis: respan(sp.shrink_to_lo(), ast::VisibilityKind::Inherited),
            span: cx.with_def_site_ctxt(sp),
            tokens: None,
        })]),
        Ok(None) => DummyResult::any(sp),
        Err(mut err) => {
            err.emit();
            DummyResult::any(sp)
        }
    }
}

fn parse_global_asm<'a>(
    cx: &mut ExtCtxt<'a>,
    sp: Span,
    tts: TokenStream,
) -> Result<Option<ast::GlobalAsm>, DiagnosticBuilder<'a>> {
    let mut p = cx.new_parser_from_tts(tts);

    if p.token == token::Eof {
        let mut err = cx.struct_span_err(sp, "macro requires a string literal as an argument");
        err.span_label(sp, "string literal required");
        return Err(err);
    }

    let expr = p.parse_expr()?;
    let (asm, _) = match expr_to_string(cx, expr, "inline assembly must be a string literal") {
        Some((s, st)) => (s, st),
        None => return Ok(None),
    };

    Ok(Some(ast::GlobalAsm { asm }))
}
