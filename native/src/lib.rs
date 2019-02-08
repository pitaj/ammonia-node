#[macro_use]
extern crate neon;
extern crate ammonia;
extern crate simple_interner;

use std::collections::{HashMap, HashSet};

use neon::prelude::*;
use ammonia::{Builder, UrlRelative, Url}; // UrlRelativeEvaluate};
use simple_interner::{Interner, Interned};

type Holder = Interner<str>;

fn build_from_arguments<'a, T: neon::object::This>(
    cx: &mut CallContext<T>,
    options: Handle<JsObject>,
    holder: &'a Holder
) -> Result<Builder<'a>, neon::result::Throw> {
    let mut builder = Builder::new();

    macro_rules! hold {
        ($e:expr) => {{
            let value = $e.value();
            Interned::get(&holder.get_or_insert(value))
        }}
    }
    
    /// convert!(subject, JS Type, Rust Type)
    macro_rules! set_opt {
        ($option_name:ident, JsBoolean, bool) => {{
            let prop = options.get(cx, stringify!($option_name))?;
            if prop.is_a::<JsBoolean>() {
                let val = prop.downcast_or_throw::<JsBoolean, _>(cx)?;
                builder.$option_name(val.value());
            }
        }};
        ($option_name:ident, JsString, Option<&str>) => {{
            let prop = options.get(cx, stringify!($option_name))?;
            if prop.is_a::<JsNull>() {
                builder.$option_name(None);
            } else if prop.is_a::<JsString>() {
                let val = prop.downcast_or_throw::<JsString, _>(cx)?;
                builder.$option_name(Some(hold!(val)));
            }
        }};
        ($option_name:ident, JsString, UrlRelative) => {{
            let prop = options.get(cx, stringify!($option_name))?;
            if prop.is_a::<JsString>() {
                let val = prop.downcast_or_throw::<JsString, _>(cx)?;

                match val.value().as_ref() {
                    "deny" => {
                        builder.$option_name(UrlRelative::Deny);
                    },
                    "pass-through" => {
                        builder.$option_name(UrlRelative::PassThrough);
                    },
                    _ => cx.throw_type_error("Invalid `url_relative` option")?,
                }
            } else if prop.is_a::<JsArray>() {
                let arr = prop.downcast_or_throw::<JsArray, _>(cx)?;
                let val = arr.get(cx, 0)?;
                let val = val.downcast_or_throw::<JsString, _>(cx)?;

                match val.value().as_ref() {
                    "resolve-with-base" => {
                        let base = arr.get(cx, 1)?;
                        let base = base.downcast_or_throw::<JsString, _>(cx)?;
                        
                        let url = Url::parse(&base.value()).unwrap();

                        builder.$option_name(
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
                    _ => unimplemented!(),
                }
            }
        }};
        ($option_name:ident, JsArray, HashSet<&str>) => {{
            let prop = options.get(cx, stringify!($option_name))?;
            if prop.is_a::<JsArray>() {
                let val = prop.downcast_or_throw::<JsArray, _>(cx)?;
                let mut set: HashSet<&str> = HashSet::with_capacity(val.len() as usize);
                
                for x in val.to_vec(cx)? {
                    let s = x.downcast_or_throw::<JsString, _>(cx)?;
                    set.insert(hold!(s));
                }
                
                builder.$option_name(set);
            }
        }};
        ($option_name:ident, JsObject, HashMap<&str, HashSet<&str>>) => {{
            let prop = options.get(cx, stringify!($option_name))?;
            if prop.is_a::<JsObject>() {
                let obj = prop.downcast_or_throw::<JsObject, _>(cx)?;
                let prop_names = obj.get_own_property_names(cx)?;

                let mut map: HashMap<&str, HashSet<&str>> = HashMap::with_capacity(prop_names.len() as usize); 
                for key in prop_names.to_vec(cx)? {
                    let key = key.downcast_or_throw::<JsString, _>(cx)?;

                    let arr = obj.get(cx, key)?;
                    let arr = arr.downcast_or_throw::<JsArray, _>(cx)?;
                    
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
        ($option_name:ident, JsObject, HashMap<&str, HashMap<&str, HashSet<&str>>>) => {{
            let prop = options.get(cx, stringify!($option_name))?;
            if prop.is_a::<JsObject>() {
                let obj1 = prop.downcast_or_throw::<JsObject, _>(cx)?;
                let prop_names1 = obj1.get_own_property_names(cx)?;

                let mut map1: HashMap<&str, HashMap<&str, HashSet<&str>>> = HashMap::with_capacity(prop_names1.len() as usize); 
                for key1 in prop_names1.to_vec(cx)? {
                    let key1 = key1.downcast_or_throw::<JsString, _>(cx)?;
                    
                    let obj2 = obj1.get(cx, key1)?;
                    let obj2 = obj2.downcast_or_throw::<JsObject, _>(cx)?;
                    let prop_names2 = obj2.get_own_property_names(cx)?;

                    let mut map2: HashMap<&str, HashSet<&str>> = HashMap::with_capacity(prop_names2.len() as usize);
                    for key2 in prop_names2.to_vec(cx)? {
                        let key2 = key2.downcast_or_throw::<JsString, _>(cx)?;

                        let arr = obj2.get(cx, key2)?;
                        let arr = arr.downcast_or_throw::<JsArray, _>(cx)?;
                        
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
    set_opt!(url_relative, JsString, UrlRelative);
    // TODO
    // set_opt!(attribute_filter, JsFunction, Fn(&str, &str, &str) -> &str);
    set_opt!(link_rel, JsString, Option<&str>);
    set_opt!(allowed_classes, JsObject, HashMap<&str, HashSet<&str>>);
    set_opt!(strip_comments, JsBoolean, bool);
    set_opt!(id_prefix, JsString, Option<&str>);

    Ok(builder)
}

fn clean(mut cx: FunctionContext) -> JsResult<JsString> {
    let input = cx.argument::<JsString>(0)?;
    let options = cx.argument::<JsObject>(1)?;

    let cleaned = {
        let holder = Interner::new();
        let builder = build_from_arguments(&mut cx, options, &holder)?;

        let doc = builder.clean(&input.value());
        doc.to_string()
    };
    
    Ok(cx.string(cleaned))
}

/// This datatype holds pointers to the Holder and Builder
/// This _should be_ safe because builder only holds refs to slices in holder
pub struct CleanerBox {
    holder: *mut Holder,
    builder: *mut Builder<'static>,
}
impl CleanerBox {
    fn new<T: neon::object::This>(
        mut cx: CallContext<T>,
        options: Handle<JsObject>
    ) -> Result<CleanerBox, neon::result::Throw> {
        let holder_p = Box::into_raw(Box::new(Interner::new()));

        let holder = unsafe { holder_p.as_ref().unwrap() };
        let builder = build_from_arguments(&mut cx, options, holder)?;

        let builder_p = Box::into_raw(Box::new(builder));

        Ok(CleanerBox {
            holder: holder_p,
            builder: builder_p as *mut Builder<'static>,
        })
    }

    fn clean(&self, input: String) -> String {
        let builder_p = self.builder;

        let builder = unsafe {
            builder_p.as_ref().unwrap()
        };

        let cleaned = {
            let doc = builder.clean(&input);
            doc.to_string()
        };

        cleaned
    }
}
impl Drop for CleanerBox {
    fn drop(&mut self) {
        let builder_p = self.builder;
        let holder_p = self.holder;

        unsafe {
            Box::from_raw(builder_p);
            Box::from_raw(holder_p);
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
            let input: String = cx.argument::<JsString>(0)?.value();

            let cleaned = {
                let this = cx.this();
                let guard = cx.lock();
                let cleaner = this.borrow(&guard);
                cleaner.clean(input)
            };

            Ok(cx.string(cleaned).upcast())
        }
    }
}

register_module!(mut m, {
    m.export_function("clean", clean)?;
    m.export_class::<JsCleaner>("Cleaner")?;
    Ok(())
});
