#[macro_use]
extern crate neon;
extern crate ammonia;
extern crate simple_interner;

use std::collections::{HashMap, HashSet};

use neon::prelude::*;
use ammonia::{Builder, UrlRelative, Url}; // UrlRelativeEvaluate};
use simple_interner::{Interner, Interned};

type Holder = Interner<str>;

fn build_from_arguments<'a>(cx: &'a mut FunctionContext, options: Handle<JsObject>, holder: &'a Holder) -> Result<Builder<'a>, neon::result::Throw> {
    let mut builder = Builder::new();

    macro_rules! hold {
        ($e:expr) => {{
            let value = $e.value();
            Interned::get(&holder.get_or_insert(value))
        }}
    }

    // conversions that need to be done
    // boolean -> bool
    // string | null -> Option(&str)
    // string ==> UrlRelative
    // string[] ==> HashSet<&str>
    // { [string]: string[] } ==> HashMap<&str, HashSet<&str>>
    // { [string]: { [string]: string[] }} ==> MashMap<&str, HashMap<&str, HashSet<&str>>>
    // (string, string, string): string | null ==> Fn(&str, &str, &str) -> Option(&str)
    
    /// convert!(subject, JS Type, Rust Type)
    macro_rules! set_opt {
        ($option_name:ident, JsBoolean, bool) => {{
            let prop = options.get(cx, stringify!($option_name))?;
            if prop.is_a::<JsBoolean>() {
                let val = prop.downcast_or_throw::<JsBoolean, FunctionContext>(cx)?;
                builder.$option_name(val.value());
            }
        }};
        ($option_name:ident, JsString, Option<&str>) => {{
            let prop = options.get(cx, stringify!($option_name))?;
            if prop.is_a::<JsNull>() {
                builder.$option_name(None);
            } else if prop.is_a::<JsString>() {
                let val = prop.downcast_or_throw::<JsString, FunctionContext>(cx)?;
                builder.$option_name(Some(hold!(val)));
            }
        }};
        ($option_name:ident, JsString, UrlRelative) => {{
            let prop = options.get(cx, stringify!($option_name))?;
            if prop.is_a::<JsString>() {
                let val = prop.downcast_or_throw::<JsString, FunctionContext>(cx)?;

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
                let arr = prop.downcast_or_throw::<JsArray, FunctionContext>(cx)?;
                let val = arr.get(cx, 0)?;
                let val = val.downcast_or_throw::<JsString, FunctionContext>(cx)?;

                match val.value().as_ref() {
                    "resolve-with-base" => {
                        let base = arr.get(cx, 1)?;
                        let base = base.downcast_or_throw::<JsString, FunctionContext>(cx)?;
                        
                        let url = Url::parse(&base.value()).unwrap();

                        builder.$option_name(
                            UrlRelative::RewriteWithBase(url)
                        );
                    },
                    // TODO
                    // "custom" => {
                    //     let func = arr.get(cx, 1)?;
                    //     let func = func.downcast_or_throw::<JsFunction, FunctionContext>(cx)?;
                        
                    //     let closure = |url: &str| -> Option<Cow<str>> {
                    //         let url = cx.string(url);
                    //         let returned = func.call(cx, cx.undefined(), vec![url]).unwrap();
                    //         if returned.is_a::<JsNull>() {
                    //             None
                    //         } else {
                    //             let returned = returned.downcast_or_throw::<JsString, FunctionContext>(cx).unwrap();
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
                let val = prop.downcast_or_throw::<JsArray, FunctionContext>(cx)?;
                let mut set: HashSet<&str> = HashSet::with_capacity(val.len() as usize);
                
                for x in val.to_vec(cx)? {
                    let s = x.downcast_or_throw::<JsString, FunctionContext>(cx)?;
                    set.insert(hold!(s));
                }
                
                builder.$option_name(set);
            }
        }};
        ($option_name:ident, JsObject, HashMap<&str, HashSet<&str>>) => {{
            let prop = options.get(cx, stringify!($option_name))?;
            if prop.is_a::<JsObject>() {
                let obj = prop.downcast_or_throw::<JsObject, FunctionContext>(cx)?;
                let prop_names = obj.get_own_property_names(cx)?;

                let mut map: HashMap<&str, HashSet<&str>> = HashMap::with_capacity(prop_names.len() as usize); 
                for key in prop_names.to_vec(cx)? {
                    let key = key.downcast_or_throw::<JsString, FunctionContext>(cx)?;

                    let arr = obj.get(cx, key)?;
                    let arr = arr.downcast_or_throw::<JsArray, FunctionContext>(cx)?;
                    
                    let mut set: HashSet<&str> = HashSet::with_capacity(arr.len() as usize);
                
                    for x in arr.to_vec(cx)? {
                        let s = x.downcast_or_throw::<JsString, FunctionContext>(cx)?;
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
                let obj1 = prop.downcast_or_throw::<JsObject, FunctionContext>(cx)?;
                let prop_names1 = obj1.get_own_property_names(cx)?;

                let mut map1: HashMap<&str, HashMap<&str, HashSet<&str>>> = HashMap::with_capacity(prop_names1.len() as usize); 
                for key1 in prop_names1.to_vec(cx)? {
                    let key1 = key1.downcast_or_throw::<JsString, FunctionContext>(cx)?;
                    
                    let obj2 = obj1.get(cx, key1)?;
                    let obj2 = obj2.downcast_or_throw::<JsObject, FunctionContext>(cx)?;
                    let prop_names2 = obj2.get_own_property_names(cx)?;

                    let mut map2: HashMap<&str, HashSet<&str>> = HashMap::with_capacity(prop_names2.len() as usize);
                    for key2 in prop_names2.to_vec(cx)? {
                        let key2 = key2.downcast_or_throw::<JsString, FunctionContext>(cx)?;

                        let arr = obj2.get(cx, key2)?;
                        let arr = arr.downcast_or_throw::<JsArray, FunctionContext>(cx)?;
                        
                        let mut set: HashSet<&str> = HashSet::with_capacity(arr.len() as usize);
                    
                        for x in arr.to_vec(cx)? {
                            let s = x.downcast_or_throw::<JsString, FunctionContext>(cx)?;
                            set.insert(hold!(s));
                        }

                        map2.insert(hold!(key2), set);
                    }

                    map1.insert(hold!(key1), map2);
                }

                builder.$option_name(map1);
            }
        }};
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

register_module!(mut m, {
    m.export_function("clean", clean)?;
    Ok(())
});
