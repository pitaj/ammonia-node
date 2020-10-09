use std::collections::{HashMap, HashSet};
use std::str;

use neon::prelude::*;
use ammonia::{Builder, UrlRelative, Url};
use simple_interner::{Interner, Interned};

type Holder = Interner<str>;

fn build_from_arguments<'a, T: neon::object::This>(
    cx: &mut CallContext<T>,
    options: Handle<JsObject>,
    holder: &'a Holder
) -> Result<Builder<'a>, neon::result::Throw> {
    let mut builder = Builder::new();

    /// Intern the value if it isn't already, and return a reference from the interned set
    macro_rules! hold {
        ($e:expr) => {{
            let value = $e.value();
            Interned::get(&holder.get_or_insert(value))
        }}
    }

    /// Utility for repeated or generic conversion and setting of options
    /// `set_opt!(subject, JS Type, Rust Type)`
    /// Any strings passed into the options are interned in `holder`
    macro_rules! set_opt {
        // `set_opt!(option, JsBoolean, bool)`
        // Set a boolean-type option, applies to the following:
        // - `strip_comments`
        ($option_name:ident, JsBoolean, bool) => {{
            let prop = options.get(cx, stringify!($option_name))?;
            if prop.is_a::<JsBoolean>() {
                let val = prop.downcast_or_throw::<JsBoolean, _>(cx)?;
                builder.$option_name(val.value());
            }
        }};
        // `set_opt!(option, JsString, Option<&str>)`
        // Set a string-type option, applies to the following:
        // - `link_rel`
        // - `id_prefix`
        ($option_name:ident, JsString, Option<&str>) => {{
            let prop = options.get(cx, stringify!($option_name))?;
            // null -> None
            if prop.is_a::<JsNull>() {
                builder.$option_name(None);
            } else if prop.is_a::<JsString>() {
                let val = prop.downcast_or_throw::<JsString, _>(cx)?;
                builder.$option_name(Some(hold!(val)));
            }
        }};
        // `set_opt!(option, JsArray, HashSet<&str>)`
        // Set an array-of-strings-type option, applies to the following:
        // - `tags`
        // - `clean_content_tags`
        // - `generic_attributes`
        // - `url_schemes`
        ($option_name:ident, JsArray, HashSet<&str>) => {{
            let prop = options.get(cx, stringify!($option_name))?;
            if prop.is_a::<JsArray>() {
                let val = prop.downcast_or_throw::<JsArray, _>(cx)?;

                // Convert string[] to HashSet<&str>
                let mut set: HashSet<&str> = HashSet::with_capacity(val.len() as usize);
                for x in val.to_vec(cx)? {
                    let s = x.downcast_or_throw::<JsString, _>(cx)?;
                    set.insert(hold!(s));
                }
                
                builder.$option_name(set);
            }
        }};
        // `set_opt!(option, JsObject, HashMap<&str, HashSet<&str>>)`
        // Set a map-of-arrays-type option, applies to the following:
        // - `tag_attributes`
        // - `allowed_classes`
        ($option_name:ident, JsObject, HashMap<&str, HashSet<&str>>) => {{
            let prop = options.get(cx, stringify!($option_name))?;
            if prop.is_a::<JsObject>() {
                let obj = prop.downcast_or_throw::<JsObject, _>(cx)?;
                
                // Convert { [string]: string[] } to HashMap<&str, HashSet<&str>>
                let prop_names = obj.get_own_property_names(cx)?;
                let mut map: HashMap<&str, HashSet<&str>> = HashMap::with_capacity(prop_names.len() as usize); 
                for key in prop_names.to_vec(cx)? {
                    let key = key.downcast_or_throw::<JsString, _>(cx)?;

                    let arr = obj.get(cx, key)?;
                    let arr = arr.downcast_or_throw::<JsArray, _>(cx)?;
                    
                    // Convert string[] to HashSet<&str>
                    let mut set: HashSet<&str> = HashSet::with_capacity(arr.len() as usize);
                    for x in arr.to_vec(cx)? {
                        let s = x.downcast_or_throw::<JsString, _>(cx)?;
                        set.insert(hold!(s));
                    }

                    map.insert(hold!(key), set);
                }

                builder.$option_name(map);
            }
        }};
        // `set_opt!(option, JsObject, HashMap<&str, HashMap<&str, HashSet<&str>>>)`
        // Set a map-of-arrays-type option, applies to the following:
        // - `tag_attribute_values`
        ($option_name:ident, JsObject, HashMap<&str, HashMap<&str, HashSet<&str>>>) => {{
            let prop = options.get(cx, stringify!($option_name))?;
            if prop.is_a::<JsObject>() {
                let obj1 = prop.downcast_or_throw::<JsObject, _>(cx)?;
                
                // Convert { [string]: { [string]: string[] } } to HashMap<&str, HashMap<&str, HashSet<&str>>>
                let prop_names1 = obj1.get_own_property_names(cx)?;
                let mut map1: HashMap<&str, HashMap<&str, HashSet<&str>>> = HashMap::with_capacity(prop_names1.len() as usize); 
                for key1 in prop_names1.to_vec(cx)? {
                    let key1 = key1.downcast_or_throw::<JsString, _>(cx)?;
                    
                    let obj2 = obj1.get(cx, key1)?;
                    let obj2 = obj2.downcast_or_throw::<JsObject, _>(cx)?;
                    
                    // Convert { [string]: string[] } to HashMap<&str, HashSet<&str>>
                    let prop_names2 = obj2.get_own_property_names(cx)?;
                    let mut map2: HashMap<&str, HashSet<&str>> = HashMap::with_capacity(prop_names2.len() as usize);
                    for key2 in prop_names2.to_vec(cx)? {
                        let key2 = key2.downcast_or_throw::<JsString, _>(cx)?;

                        let arr = obj2.get(cx, key2)?;
                        let arr = arr.downcast_or_throw::<JsArray, _>(cx)?;
                        
                        // Convert string[] to HashSet<&str>
                        let mut set: HashSet<&str> = HashSet::with_capacity(arr.len() as usize);
                        for x in arr.to_vec(cx)? {
                            let s = x.downcast_or_throw::<JsString, _>(cx)?;
                            set.insert(hold!(s));
                        }

                        map2.insert(hold!(key2), set);
                    }

                    map1.insert(hold!(key1), map2);
                }

                builder.$option_name(map1);
            }
        }};
        // TODO
        // (string, string, string): string | null ==> Fn(&str, &str, &str) -> Option(&str)
    }

    set_opt!(tags, JsArray, HashSet<&str>);
    set_opt!(clean_content_tags, JsArray, HashSet<&str>);
    set_opt!(tag_attributes, JsObject, HashMap<&str, HashSet<&str>>);
    set_opt!(tag_attribute_values, JsObject, HashMap<&str, HashMap<&str, HashSet<&str>>>);
    set_opt!(generic_attributes, JsArray, HashSet<&str>);
    set_opt!(url_schemes, JsArray, HashSet<&str>);
    // TODO
    // set_opt!(attribute_filter, JsFunction, Fn(&str, &str, &str) -> &str);
    set_opt!(link_rel, JsString, Option<&str>);
    set_opt!(allowed_classes, JsObject, HashMap<&str, HashSet<&str>>);
    set_opt!(strip_comments, JsBoolean, bool);
    set_opt!(id_prefix, JsString, Option<&str>);

    // Handle `url_relative` option
    // Special case, isn't applicable to any other options
    let prop = options.get(cx, "url_relative")?;
    if prop.is_a::<JsString>() {
        let val = prop.downcast_or_throw::<JsString, _>(cx)?;

        // Convert string to simple variants
        match val.value().as_ref() {
            "deny" => {
                builder.url_relative(UrlRelative::Deny);
            },
            "pass-through" => {
                builder.url_relative(UrlRelative::PassThrough);
            },
            _ => cx.throw_type_error("Invalid `url_relative` option")?,
        }
    } else if prop.is_a::<JsArray>() {
        let arr = prop.downcast_or_throw::<JsArray, _>(cx)?;
        let val = arr.get(cx, 0)?;
        let val = val.downcast_or_throw::<JsString, _>(cx)?;

        match val.value().as_ref() {
            // Convert ["resolve-with-base", url: string] to RewriteWithBase(Url)
            "resolve-with-base" => {
                let base = arr.get(cx, 1)?;
                let base = base.downcast_or_throw::<JsString, _>(cx)?;
                
                let url = Url::parse(&base.value()).unwrap();

                builder.url_relative(
                    UrlRelative::RewriteWithBase(url)
                );
            },
            // TODO
            // "custom" => {
            //     let func = arr.get(cx, 1)?;
            //     let func = func.downcast_or_throw::<JsFunction, _>(cx)?;
                
            //     let closure = |url: &str| -> Option<Cow<str>> {
            //         let url = cx.string(url);
            //         let returned = func.call(cx, cx.undefined(), vec![url]).unwrap();
            //         if returned.is_a::<JsNull>() {
            //             None
            //         } else {
            //             let returned = returned.downcast_or_throw::<JsString, _>(cx).unwrap();
            //             Some(returned.value().into())
            //         }
            //     };

            //     let val = UrlRelative::Custom(Box::new(closure));
            //     builder.$option_name(val);
            // },
            _ => cx.throw_type_error("Unknown `url_relative` option.
    Only `deny`, `pass-through`, and `resolve-with-base` are supported.")?,
        }
    }

    Ok(builder)
}

/// Use the default options of ammonia
fn default(mut cx: FunctionContext) -> JsResult<JsBuffer> {
    let input = cx.argument::<JsBuffer>(0)?;

    // Clean the input text
    let cleaned = {
        let guard = cx.lock();
        let input: &[u8] = input.borrow(&guard).as_slice();

        // Input buffer must be UTF-8
        let input: &str = unsafe { str::from_utf8_unchecked(input) };
        ammonia::clean(input)
    };

    // TODO: reuse input buffer?
    // Create a new buffer to return
    let cleaned = {
        let buffer = unsafe { JsBuffer::uninitialized(&mut cx, cleaned.len() as u32)? };
        let guard = cx.lock();
        let bytes: &mut [u8] = buffer.borrow(&guard).as_mut_slice();
        bytes.copy_from_slice(&cleaned.as_bytes());
        buffer
    };

    Ok(cleaned)
}

/// Pass in options directly instead of creating a class instance
fn clean(mut cx: FunctionContext) -> JsResult<JsBuffer> {
    let input = cx.argument::<JsBuffer>(0)?;
    let options = cx.argument::<JsObject>(1)?;

    // Clean the input text
    let cleaned = {
        let guard = cx.lock();
        let input: &[u8] = input.borrow(&guard).as_slice();

        // Input buffer must be UTF-8
        let input: &str = unsafe { str::from_utf8_unchecked(input) };
        let holder = Interner::new();
        let builder = build_from_arguments(&mut cx, options, &holder)?;

        builder.clean(input).to_string()
    };

    // TODO: reuse input buffer?
    // Create a new buffer to return
    let cleaned = {
        let buffer = unsafe { JsBuffer::uninitialized(&mut cx, cleaned.len() as u32)? };
        let guard = cx.lock();
        let bytes: &mut [u8] = buffer.borrow(&guard).as_mut_slice();
        bytes.copy_from_slice(&cleaned.as_bytes());
        buffer
    };

    Ok(cleaned)
}

/// This datatype holds pointers to the Holder and Builder
/// This _should be_ safe because builder only holds refs to slices in holder
pub struct CleanerBox {
    holder: *mut Holder,
    builder: Builder<'static>,
}
impl CleanerBox {
    fn new<T: neon::object::This>(
        mut cx: CallContext<T>,
        options: Handle<JsObject>
    ) -> Result<CleanerBox, neon::result::Throw> {
        let holder_p = Box::into_raw(Box::new(Interner::new()));

        let holder = unsafe { holder_p.as_ref().unwrap() };
        let builder = build_from_arguments(&mut cx, options, holder)?;

        Ok(CleanerBox {
            holder: holder_p,
            builder,
        })
    }

    fn clean(&self, input: &str) -> String {
        let doc = self.builder.clean(input);
        doc.to_string()
    }
}
impl Drop for CleanerBox {
    fn drop(&mut self) {
        unsafe {
            Box::from_raw(self.holder);
        }
    }
}

declare_types! {
    pub class JsCleaner for CleanerBox {
        init(mut cx) {
            let options = cx.argument::<JsObject>(0)?;

            CleanerBox::new(cx, options)
        }

        method clean(mut cx) {
            let input = cx.argument::<JsBuffer>(0)?;

            // Clean the input text
            let cleaned = {
                let this = cx.this();
                let guard = cx.lock();
                let input: &[u8] = input.borrow(&guard).as_slice();

                // Input buffer must be UTF-8
                let input: &str = unsafe { str::from_utf8_unchecked(input) };
                let cleaner = this.borrow(&guard);
                cleaner.clean(input)
            };

            // TODO: reuse input buffer?
            // Create a new buffer to return
            let cleaned = {
                let buffer = unsafe { JsBuffer::uninitialized(&mut cx, cleaned.len() as u32)? };
                let guard = cx.lock();
                let bytes: &mut [u8] = buffer.borrow(&guard).as_mut_slice();
                bytes.copy_from_slice(&cleaned.as_bytes());
                buffer
            };

            Ok(cleaned.upcast())
        }
    }
}

register_module!(mut m, {
    m.export_function("clean", clean)?;
    m.export_function("default", default)?;
    m.export_class::<JsCleaner>("Cleaner")?;
    Ok(())
});
