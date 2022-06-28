// This file is part of Sumy <https://github.com/gemrest/sumy>.
// Copyright (C) 2022-2022 Fuwn <contact@fuwn.me>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.
//
// Copyright (C) 2022-2022 Fuwn <contact@fuwn.me>
// SPDX-License-Identifier: GPL-3.0-only

#![deny(
  clippy::all,
  clippy::nursery,
  clippy::pedantic,
  future_incompatible,
  nonstandard_style,
  rust_2018_idioms,
  unsafe_code,
  unused,
  warnings
)]
#![doc = include_str!("../README.md")]
#![recursion_limit = "128"]

use germ::convert::{self, from_string};
use neon::prelude::*;

fn request(mut cx: FunctionContext<'_>) -> JsResult<'_, JsObject> {
  let object = cx.empty_object();
  let url = cx.argument::<JsString>(0)?.value(&mut cx);

  match &url::Url::parse(&url) {
    Ok(url) =>
      match germ::request::request(url) {
        Ok(request) => {
          let size =
            cx.number(request.size().to_string().parse::<f64>().unwrap_or(0.0));
          let meta = cx.string(request.meta());
          let status = cx.string(request.status().to_string());
          let content = cx.string(
            request.content().clone().unwrap_or_else(|| "".to_string()),
          );

          object.set(&mut cx, "status", status)?;
          object.set(&mut cx, "meta", meta)?;
          object.set(&mut cx, "size", size)?;
          object.set(&mut cx, "content", content)?;
        }
        Err(_) => return Err(cx.throw_error("Request failed")?),
      },
    Err(_) => return Err(cx.throw_error("Invalid URL")?),
  }

  Ok(object)
}

fn gemini_to_html(mut cx: FunctionContext<'_>) -> JsResult<'_, JsString> {
  let gemini = cx.argument::<JsString>(0)?.value(&mut cx);

  Ok(cx.string(from_string(&gemini, &convert::Target::HTML)))
}

fn gemini_to_md(mut cx: FunctionContext<'_>) -> JsResult<'_, JsString> {
  // let ast = Ast::from_string(&input.value(&mut cx));
  let gemini = cx.argument::<JsString>(0)?.value(&mut cx);

  Ok(cx.string(from_string(&gemini, &convert::Target::Markdown)))
}

#[neon::main]
fn main(mut cx: ModuleContext<'_>) -> NeonResult<()> {
  cx.export_function("geminiToHtml", gemini_to_html)?;
  cx.export_function("geminiToMarkdown", gemini_to_md)?;
  cx.export_function("request", request)?;

  Ok(())
}
