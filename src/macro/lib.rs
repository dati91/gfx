// Copyright 2014 The Gfx-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![crate_name = "gfx_macro"]
#![comment = "Helper macros for gfx-rs"]
#![license = "ASL2"]
#![crate_type = "dylib"]

#![feature(macro_rules, plugin_registrar)]

//! Macro extensions crate.
//! Implements `shaders!` macro as well as `shader_param` attribute

pub mod shade;

#[macro_export]
macro_rules! shaders {
    (GLSL_120: $v:expr $($t:tt)*) => {
        ::gfx::ShaderSource {
            glsl_120: ::gfx::StaticBytes($v),
            ..shaders!($($t)*)
        }
    };
    (GLSL_150: $v:expr $($t:tt)*) => {
        ::gfx::ShaderSource {
            glsl_150: ::gfx::StaticBytes($v),
            ..shaders!($($t)*)
        }
    };
    () => {
        ::gfx::NOT_PROVIDED
    }
}