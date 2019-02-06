#[macro_use]
extern crate neon;
extern crate ammonia;
// #[macro_use]
// extern crate lazy_static;
// extern crate internship;

mod string_cache;

// use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
// use std::sync::atomic::{AtomicUsize, Ordering};

use neon::prelude::*;
use ammonia::{Builder, UrlRelative, UrlRelativeEvaluate, Url};

// lazy_static! {
//     static ref CACHE: HashMap<usize, Builder<'static>> = HashMap::new();
//     static ref LAST_ID: AtomicUsize = AtomicUsize::new(0);
// }

// pub struct Cleaner<'a> {
//     strings: Vec<String>,
//     builder: Builder<'a>,
// }

// declare_types! {
//     pub class JsCleaner for Cleaner {
//         init(mut cx) {
//             let options = cx.argument::<JsObject>(0)?;

//             let mut cleaner: Cleaner<'r> = Cleaner {
//                 strings: Vec::new(),
//                 builder: Builder::new(),
//             };

//             // FIXME: this is really hacky
//             macro_rules! leak {
//                 ($e:expr) => {{
//                     cleaner.strings.push($e.value());
//                     let s: &str = &cleaner.strings[cleaner.strings.len() - 1];
//                     s
//                 }}
//             }

//             // conversions that need to be done
//             // boolean -> bool
//             // string | null -> Option(&str)
//             // string ==> UrlRelative
//             // string[] ==> HashSet<&str>
//             // { [string]: string[] } ==> HashMap<&str, HashSet<&str>>
//             // { [string]: { [string]: string[] }} ==> MashMap<&str, HashMap<&str, HashSet<&str>>>
//             // (string, string, string): string | null ==> Fn(&str, &str, &str) -> Option(&str)
            
//             /// convert!(subject, JS Type, Rust Type)
//             macro_rules! set_opt {
//                 ($option_name:ident, JsBoolean, bool) => {{
//                     let prop = options.get(&mut cx, stringify!(option_name))?;
//                     if prop.is_a::<JsBoolean>() {
//                         let val = prop.downcast_or_throw::<JsBoolean, FunctionContext>(&mut cx)?;
//                         cleaner.builder.$option_name(val.value());
//                     }
//                 }};
//                 ($option_name:ident, JsString, Option<&str>) => {{
//                     let prop = options.get(&mut cx, stringify!(option_name))?;
//                     if prop.is_a::<JsNull>() {
//                         cleaner.builder.$option_name(None);
//                     } else if prop.is_a::<JsString>() {
//                         let val = prop.downcast_or_throw::<JsString, FunctionContext>(&mut cx)?;
//                         cleaner.builder.$option_name(Some(leak!(val)));
//                     }
//                 }};
//                 ($option_name:ident, JsString, UrlRelative) => {{
//                     let prop = options.get(&mut cx, stringify!(option_name))?;
//                     if prop.is_a::<JsString>() {
//                         let val = prop.downcast_or_throw::<JsString, FunctionContext>(&mut cx)?;

//                         match val.value().as_ref() {
//                             "deny" => {
//                                 cleaner.builder.$option_name(UrlRelative::Deny);
//                             },
//                             "pass-through" => {
//                                 cleaner.builder.$option_name(UrlRelative::PassThrough);
//                             },
//                             _ => cx.throw_type_error("Invalid `url_relative` option")?,
//                         }
//                     } else if prop.is_a::<JsArray>() {
//                         let arr = prop.downcast_or_throw::<JsArray, FunctionContext>(&mut cx)?;
//                         let val = arr.get(&mut cx, 0)?;
//                         let val = val.downcast_or_throw::<JsString, FunctionContext>(&mut cx)?;

//                         match val.value().as_ref() {
//                             "resolve-with-base" => {
//                                 let base = arr.get(&mut cx, 1)?;
//                                 let base = base.downcast_or_throw::<JsString, FunctionContext>(&mut cx)?;
                                
//                                 let url = Url::parse(&base.value()).unwrap();

//                                 cleaner.builder.$option_name(
//                                     UrlRelative::RewriteWithBase(url)
//                                 );
//                             },
//                             // "custom" => {
//                             //     let func = arr.get(&mut cx, 1)?;
//                             //     let func = func.downcast_or_throw::<JsFunction, FunctionContext>(&mut cx)?;
                                
//                             //     let closure = |url: &str| -> Option<Cow<str>> {
//                             //         let url = cx.string(url);
//                             //         let returned = func.call(&mut cx, cx.undefined(), vec![url]).unwrap();
//                             //         if returned.is_a::<JsNull>() {
//                             //             None
//                             //         } else {
//                             //             let returned = returned.downcast_or_throw::<JsString, FunctionContext>(&mut cx).unwrap();
//                             //             Some(returned.value().into())
//                             //         }
//                             //     };

//                             //     let val = UrlRelative::Custom(Box::new(closure));
//                             //     cleaner.builder.$option_name(val);
//                             // },
//                             _ => unimplemented!(),
//                         }
//                     }
//                 }};
//                 ($option_name:ident, JsArray, HashSet<&str>) => {{
//                     let prop = options.get(&mut cx, stringify!(option_name))?;
//                     if prop.is_a::<JsArray>() {
//                         let val = prop.downcast_or_throw::<JsArray, FunctionContext>(&mut cx)?;
//                         let mut set: HashSet<&'static str> = HashSet::with_capacity(val.len() as usize);
                        
//                         for x in val.to_vec(&mut cx)? {
//                             let s = x.downcast_or_throw::<JsString, FunctionContext>(&mut cx)?;
//                             // FIXME: this is really hacky
//                             set.insert(leak!(s));
//                         }
                        
//                         cleaner.builder.$option_name(set);
//                     }
//                 }};
//                 ($option_name:ident, JsObject, HashMap<&str, HashSet<&str>>) => {{
//                     let prop = options.get(&mut cx, stringify!(option_name))?;
//                     if prop.is_a::<JsObject>() {
//                         let obj = prop.downcast_or_throw::<JsObject, FunctionContext>(&mut cx)?;
//                         let prop_names = obj.get_own_property_names(&mut cx)?;

//                         let mut map: HashMap<&'static str, HashSet<&'static str>> = HashMap::with_capacity(prop_names.len() as usize); 
//                         for key in prop_names.to_vec(&mut cx)? {
//                             let key = key.downcast_or_throw::<JsString, FunctionContext>(&mut cx)?;

//                             let arr = obj.get(&mut cx, key)?;
//                             let arr = arr.downcast_or_throw::<JsArray, FunctionContext>(&mut cx)?;
                            
//                             let mut set: HashSet<&'static str> = HashSet::with_capacity(arr.len() as usize);
                        
//                             for x in arr.to_vec(&mut cx)? {
//                                 let s = x.downcast_or_throw::<JsString, FunctionContext>(&mut cx)?;
//                                 // FIXME: this is really hacky
//                                 set.insert(leak!(s));
//                             }

//                             map.insert(leak!(key), set);
//                         }

//                         cleaner.builder.$option_name(map);
//                     }
//                 }};
//                 ($option_name:ident, JsObject, HashMap<&str, HashMap<&str, HashSet<&str>>>) => {{
//                     let prop = options.get(&mut cx, stringify!(option_name))?;
//                     if prop.is_a::<JsObject>() {
//                         let obj1 = prop.downcast_or_throw::<JsObject, FunctionContext>(&mut cx)?;
//                         let prop_names1 = obj1.get_own_property_names(&mut cx)?;

//                         let mut map1: HashMap<&str, HashMap<&str, HashSet<&str>>> = HashMap::with_capacity(prop_names1.len() as usize); 
//                         for key1 in prop_names1.to_vec(&mut cx)? {
//                             let key1 = key1.downcast_or_throw::<JsString, FunctionContext>(&mut cx)?;
                            
//                             let obj2 = obj1.get(&mut cx, key1)?;
//                             let obj2 = obj2.downcast_or_throw::<JsObject, FunctionContext>(&mut cx)?;
//                             let prop_names2 = obj2.get_own_property_names(&mut cx)?;

//                             let mut map2: HashMap<&'static str, HashSet<&'static str>> = HashMap::with_capacity(prop_names2.len() as usize);
//                             for key2 in prop_names2.to_vec(&mut cx)? {
//                                 let key2 = key2.downcast_or_throw::<JsString, FunctionContext>(&mut cx)?;

//                                 let arr = obj2.get(&mut cx, key2)?;
//                                 let arr = arr.downcast_or_throw::<JsArray, FunctionContext>(&mut cx)?;
                                
//                                 let mut set: HashSet<&'static str> = HashSet::with_capacity(arr.len() as usize);
                            
//                                 for x in arr.to_vec(&mut cx)? {
//                                     let s = x.downcast_or_throw::<JsString, FunctionContext>(&mut cx)?;
//                                     // FIXME: this is really hacky
//                                     set.insert(leak!(s));
//                                 }

//                                 map2.insert(leak!(key2), set);
//                             }

//                             map1.insert(leak!(key1), map2);
//                         }

//                         cleaner.builder.$option_name(map1);
//                     }
//                 }};
//             }

//             set_opt!(tags, JsArray, HashSet<&str>);
//             set_opt!(clean_content_tags, JsArray, HashSet<&str>);
//             set_opt!(tag_attributes, JsObject, HashMap<&str, HashSet<&str>>);
//             set_opt!(tag_attribute_values, JsObject, HashMap<&str, HashMap<&str, HashSet<&str>>>);
//             set_opt!(generic_attributes, JsArray, HashSet<&str>);
//             set_opt!(url_schemes, JsArray, HashSet<&str>);
//             set_opt!(url_relative, JsString, UrlRelative);
//             // set_opt!(attribute_filter, JsFunction, Fn(&str, &str, &str) -> &str);
//             set_opt!(link_rel, JsString, Option<&str>);
//             set_opt!(allowed_classes, JsObject, HashMap<&str, HashSet<&str>>);
//             set_opt!(strip_comments, JsBoolean, bool);
//             set_opt!(id_prefix, JsString, Option<&str>);

//             Ok(cleaner)
//         }
//     }
// }

// fn build(mut cx: FunctionContext) -> JsResult<JsObject> {
//     let options = cx.argument::<JsObject>(0)?;

//     let mut cleaner = Cleaner {
//         strings: Vec::new(),
//         builder: Builder::new(),
//     };

//     // FIXME: this is really hacky
//     macro_rules! leak {
//         ($e:expr) => {{
//             cleaner.strings.push($e.value());
//             let s: &str = &cleaner.strings[cleaner.strings.len() - 1];
//             s
//         }}
//     }

//     // conversions that need to be done
//     // boolean -> bool
//     // string | null -> Option(&str)
//     // string ==> UrlRelative
//     // string[] ==> HashSet<&str>
//     // { [string]: string[] } ==> HashMap<&str, HashSet<&str>>
//     // { [string]: { [string]: string[] }} ==> MashMap<&str, HashMap<&str, HashSet<&str>>>
//     // (string, string, string): string | null ==> Fn(&str, &str, &str) -> Option(&str)
    
//     /// convert!(subject, JS Type, Rust Type)
//     macro_rules! set_opt {
//         ($option_name:ident, JsBoolean, bool) => {{
//             let prop = options.get(&mut cx, stringify!(option_name))?;
//             if prop.is_a::<JsBoolean>() {
//                 let val = prop.downcast_or_throw::<JsBoolean, FunctionContext>(&mut cx)?;
//                 cleaner.builder.$option_name(val.value());
//             }
//         }};
//         ($option_name:ident, JsString, Option<&str>) => {{
//             let prop = options.get(&mut cx, stringify!(option_name))?;
//             if prop.is_a::<JsNull>() {
//                 cleaner.builder.$option_name(None);
//             } else if prop.is_a::<JsString>() {
//                 let val = prop.downcast_or_throw::<JsString, FunctionContext>(&mut cx)?;
//                 cleaner.builder.$option_name(Some(leak!(val)));
//             }
//         }};
//         ($option_name:ident, JsString, UrlRelative) => {{
//             let prop = options.get(&mut cx, stringify!(option_name))?;
//             if prop.is_a::<JsString>() {
//                 let val = prop.downcast_or_throw::<JsString, FunctionContext>(&mut cx)?;

//                 match val.value().as_ref() {
//                     "deny" => {
//                         cleaner.builder.$option_name(UrlRelative::Deny);
//                     },
//                     "pass-through" => {
//                         cleaner.builder.$option_name(UrlRelative::PassThrough);
//                     },
//                     _ => cx.throw_type_error("Invalid `url_relative` option")?,
//                 }
//             } else if prop.is_a::<JsArray>() {
//                 let arr = prop.downcast_or_throw::<JsArray, FunctionContext>(&mut cx)?;
//                 let val = arr.get(&mut cx, 0)?;
//                 let val = val.downcast_or_throw::<JsString, FunctionContext>(&mut cx)?;

//                 match val.value().as_ref() {
//                     "resolve-with-base" => {
//                         let base = arr.get(&mut cx, 1)?;
//                         let base = base.downcast_or_throw::<JsString, FunctionContext>(&mut cx)?;
                        
//                         let url = Url::parse(&base.value()).unwrap();

//                         cleaner.builder.$option_name(
//                             UrlRelative::RewriteWithBase(url)
//                         );
//                     },
//                     // "custom" => {
//                     //     let func = arr.get(&mut cx, 1)?;
//                     //     let func = func.downcast_or_throw::<JsFunction, FunctionContext>(&mut cx)?;
                        
//                     //     let closure = |url: &str| -> Option<Cow<str>> {
//                     //         let url = cx.string(url);
//                     //         let returned = func.call(&mut cx, cx.undefined(), vec![url]).unwrap();
//                     //         if returned.is_a::<JsNull>() {
//                     //             None
//                     //         } else {
//                     //             let returned = returned.downcast_or_throw::<JsString, FunctionContext>(&mut cx).unwrap();
//                     //             Some(returned.value().into())
//                     //         }
//                     //     };

//                     //     let val = UrlRelative::Custom(Box::new(closure));
//                     //     cleaner.builder.$option_name(val);
//                     // },
//                     _ => unimplemented!(),
//                 }
//             }
//         }};
//         ($option_name:ident, JsArray, HashSet<&str>) => {{
//             let prop = options.get(&mut cx, stringify!(option_name))?;
//             if prop.is_a::<JsArray>() {
//                 let val = prop.downcast_or_throw::<JsArray, FunctionContext>(&mut cx)?;
//                 let mut set: HashSet<&'static str> = HashSet::with_capacity(val.len() as usize);
                
//                 for x in val.to_vec(&mut cx)? {
//                     let s = x.downcast_or_throw::<JsString, FunctionContext>(&mut cx)?;
//                     // FIXME: this is really hacky
//                     set.insert(leak!(s));
//                 }
                
//                 cleaner.builder.$option_name(set);
//             }
//         }};
//         ($option_name:ident, JsObject, HashMap<&str, HashSet<&str>>) => {{
//             let prop = options.get(&mut cx, stringify!(option_name))?;
//             if prop.is_a::<JsObject>() {
//                 let obj = prop.downcast_or_throw::<JsObject, FunctionContext>(&mut cx)?;
//                 let prop_names = obj.get_own_property_names(&mut cx)?;

//                 let mut map: HashMap<&'static str, HashSet<&'static str>> = HashMap::with_capacity(prop_names.len() as usize); 
//                 for key in prop_names.to_vec(&mut cx)? {
//                     let key = key.downcast_or_throw::<JsString, FunctionContext>(&mut cx)?;

//                     let arr = obj.get(&mut cx, key)?;
//                     let arr = arr.downcast_or_throw::<JsArray, FunctionContext>(&mut cx)?;
                    
//                     let mut set: HashSet<&'static str> = HashSet::with_capacity(arr.len() as usize);
                
//                     for x in arr.to_vec(&mut cx)? {
//                         let s = x.downcast_or_throw::<JsString, FunctionContext>(&mut cx)?;
//                         // FIXME: this is really hacky
//                         set.insert(leak!(s));
//                     }

//                     map.insert(leak!(key), set);
//                 }

//                 cleaner.builder.$option_name(map);
//             }
//         }};
//         ($option_name:ident, JsObject, HashMap<&str, HashMap<&str, HashSet<&str>>>) => {{
//             let prop = options.get(&mut cx, stringify!(option_name))?;
//             if prop.is_a::<JsObject>() {
//                 let obj1 = prop.downcast_or_throw::<JsObject, FunctionContext>(&mut cx)?;
//                 let prop_names1 = obj1.get_own_property_names(&mut cx)?;

//                 let mut map1: HashMap<&str, HashMap<&str, HashSet<&str>>> = HashMap::with_capacity(prop_names1.len() as usize); 
//                 for key1 in prop_names1.to_vec(&mut cx)? {
//                     let key1 = key1.downcast_or_throw::<JsString, FunctionContext>(&mut cx)?;
                    
//                     let obj2 = obj1.get(&mut cx, key1)?;
//                     let obj2 = obj2.downcast_or_throw::<JsObject, FunctionContext>(&mut cx)?;
//                     let prop_names2 = obj2.get_own_property_names(&mut cx)?;

//                     let mut map2: HashMap<&'static str, HashSet<&'static str>> = HashMap::with_capacity(prop_names2.len() as usize);
//                     for key2 in prop_names2.to_vec(&mut cx)? {
//                         let key2 = key2.downcast_or_throw::<JsString, FunctionContext>(&mut cx)?;

//                         let arr = obj2.get(&mut cx, key2)?;
//                         let arr = arr.downcast_or_throw::<JsArray, FunctionContext>(&mut cx)?;
                        
//                         let mut set: HashSet<&'static str> = HashSet::with_capacity(arr.len() as usize);
                    
//                         for x in arr.to_vec(&mut cx)? {
//                             let s = x.downcast_or_throw::<JsString, FunctionContext>(&mut cx)?;
//                             // FIXME: this is really hacky
//                             set.insert(leak!(s));
//                         }

//                         map2.insert(leak!(key2), set);
//                     }

//                     map1.insert(leak!(key1), map2);
//                 }

//                 cleaner.builder.$option_name(map1);
//             }
//         }};
//     }

//     set_opt!(tags, JsArray, HashSet<&str>);
//     set_opt!(clean_content_tags, JsArray, HashSet<&str>);
//     set_opt!(tag_attributes, JsObject, HashMap<&str, HashSet<&str>>);
//     set_opt!(tag_attribute_values, JsObject, HashMap<&str, HashMap<&str, HashSet<&str>>>);
//     set_opt!(generic_attributes, JsArray, HashSet<&str>);
//     set_opt!(url_schemes, JsArray, HashSet<&str>);
//     set_opt!(url_relative, JsString, UrlRelative);
//     // set_opt!(attribute_filter, JsFunction, Fn(&str, &str, &str) -> &str);
//     set_opt!(link_rel, JsString, Option<&str>);
//     set_opt!(allowed_classes, JsObject, HashMap<&str, HashSet<&str>>);
//     set_opt!(strip_comments, JsBoolean, bool);
//     set_opt!(id_prefix, JsString, Option<&str>);

//     // let mut id = LAST_ID.fetch_add(1, Ordering::SeqCst);
//     // while CACHE.contains_key(&id) {
//     //     id = LAST_ID.fetch_add(1, Ordering::SeqCst);
//     // }

//     // CACHE.insert(id, builder);
//     // Ok(cx.number(0))
//     // let clean = |mut cx: FunctionContext| -> JsResult<JsString> {

//     // };


//     let mut cleaner = cx.empty_object();
//     // cleaner.set(&mut cx, "clean", JsFunction::new(&mut cx, clean)?);

//     Ok(cleaner)
// }

struct Holder<T>(Vec<T>);
impl<'a, T> Holder<T> {
    pub fn push(&mut self, item: T) -> &'a T {
        self.0.push(item);
        unsafe {
            let ptr = self.0.as_ptr();
            let reference: &'a T = &*ptr;
            reference
        }
    }
}

fn build_from_arguments<'a>(mut cx: FunctionContext<'a>, input: Handle<JsString>, options: Handle<JsObject>) -> Result<(Builder<'a>, Holder<String>, FunctionContext<'a>), neon::result::Throw> {
    let mut builder = Builder::new();
    let mut strings: Holder<String> = Holder(Vec::new());

    // FIXME: this is really hacky
    macro_rules! leak {
        ($e:expr) => {{
            let s = $e.value();
            strings.push(s)
            // unsafe { &strings[strings.len() - 1].b }

            // Box::leak(s.into_boxed_str())
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
            let prop = options.get(&mut cx, stringify!(option_name))?;
            if prop.is_a::<JsBoolean>() {
                let val = prop.downcast_or_throw::<JsBoolean, FunctionContext>(&mut cx)?;
                builder.$option_name(val.value());
            }
        }};
        ($option_name:ident, JsString, Option<&str>) => {{
            let prop = options.get(&mut cx, stringify!(option_name))?;
            if prop.is_a::<JsNull>() {
                builder.$option_name(None);
            } else if prop.is_a::<JsString>() {
                let val = prop.downcast_or_throw::<JsString, FunctionContext>(&mut cx)?;
                builder.$option_name(Some(leak!(val)));
            }
        }};
        ($option_name:ident, JsString, UrlRelative) => {{
            let prop = options.get(&mut cx, stringify!(option_name))?;
            if prop.is_a::<JsString>() {
                let val = prop.downcast_or_throw::<JsString, FunctionContext>(&mut cx)?;

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
                let arr = prop.downcast_or_throw::<JsArray, FunctionContext>(&mut cx)?;
                let val = arr.get(&mut cx, 0)?;
                let val = val.downcast_or_throw::<JsString, FunctionContext>(&mut cx)?;

                match val.value().as_ref() {
                    "resolve-with-base" => {
                        let base = arr.get(&mut cx, 1)?;
                        let base = base.downcast_or_throw::<JsString, FunctionContext>(&mut cx)?;
                        
                        let url = Url::parse(&base.value()).unwrap();

                        builder.$option_name(
                            UrlRelative::RewriteWithBase(url)
                        );
                    },
                    // "custom" => {
                    //     let func = arr.get(&mut cx, 1)?;
                    //     let func = func.downcast_or_throw::<JsFunction, FunctionContext>(&mut cx)?;
                        
                    //     let closure = |url: &str| -> Option<Cow<str>> {
                    //         let url = cx.string(url);
                    //         let returned = func.call(&mut cx, cx.undefined(), vec![url]).unwrap();
                    //         if returned.is_a::<JsNull>() {
                    //             None
                    //         } else {
                    //             let returned = returned.downcast_or_throw::<JsString, FunctionContext>(&mut cx).unwrap();
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
            let prop = options.get(&mut cx, stringify!(option_name))?;
            if prop.is_a::<JsArray>() {
                let val = prop.downcast_or_throw::<JsArray, FunctionContext>(&mut cx)?;
                let mut set: HashSet<&'static str> = HashSet::with_capacity(val.len() as usize);
                
                for x in val.to_vec(&mut cx)? {
                    let s = x.downcast_or_throw::<JsString, FunctionContext>(&mut cx)?;
                    // FIXME: this is really hacky
                    set.insert(leak!(s));
                }
                
                builder.$option_name(set);
            }
        }};
        ($option_name:ident, JsObject, HashMap<&str, HashSet<&str>>) => {{
            let prop = options.get(&mut cx, stringify!(option_name))?;
            if prop.is_a::<JsObject>() {
                let obj = prop.downcast_or_throw::<JsObject, FunctionContext>(&mut cx)?;
                let prop_names = obj.get_own_property_names(&mut cx)?;

                let mut map: HashMap<&'static str, HashSet<&'static str>> = HashMap::with_capacity(prop_names.len() as usize); 
                for key in prop_names.to_vec(&mut cx)? {
                    let key = key.downcast_or_throw::<JsString, FunctionContext>(&mut cx)?;

                    let arr = obj.get(&mut cx, key)?;
                    let arr = arr.downcast_or_throw::<JsArray, FunctionContext>(&mut cx)?;
                    
                    let mut set: HashSet<&'static str> = HashSet::with_capacity(arr.len() as usize);
                
                    for x in arr.to_vec(&mut cx)? {
                        let s = x.downcast_or_throw::<JsString, FunctionContext>(&mut cx)?;
                        // FIXME: this is really hacky
                        set.insert(leak!(s));
                    }

                    map.insert(leak!(key), set);
                }

                builder.$option_name(map);
            }
        }};
        ($option_name:ident, JsObject, HashMap<&str, HashMap<&str, HashSet<&str>>>) => {{
            let prop = options.get(&mut cx, stringify!(option_name))?;
            if prop.is_a::<JsObject>() {
                let obj1 = prop.downcast_or_throw::<JsObject, FunctionContext>(&mut cx)?;
                let prop_names1 = obj1.get_own_property_names(&mut cx)?;

                let mut map1: HashMap<&str, HashMap<&str, HashSet<&str>>> = HashMap::with_capacity(prop_names1.len() as usize); 
                for key1 in prop_names1.to_vec(&mut cx)? {
                    let key1 = key1.downcast_or_throw::<JsString, FunctionContext>(&mut cx)?;
                    
                    let obj2 = obj1.get(&mut cx, key1)?;
                    let obj2 = obj2.downcast_or_throw::<JsObject, FunctionContext>(&mut cx)?;
                    let prop_names2 = obj2.get_own_property_names(&mut cx)?;

                    let mut map2: HashMap<&'static str, HashSet<&'static str>> = HashMap::with_capacity(prop_names2.len() as usize);
                    for key2 in prop_names2.to_vec(&mut cx)? {
                        let key2 = key2.downcast_or_throw::<JsString, FunctionContext>(&mut cx)?;

                        let arr = obj2.get(&mut cx, key2)?;
                        let arr = arr.downcast_or_throw::<JsArray, FunctionContext>(&mut cx)?;
                        
                        let mut set: HashSet<&'static str> = HashSet::with_capacity(arr.len() as usize);
                    
                        for x in arr.to_vec(&mut cx)? {
                            let s = x.downcast_or_throw::<JsString, FunctionContext>(&mut cx)?;
                            // FIXME: this is really hacky
                            set.insert(leak!(s));
                        }

                        map2.insert(leak!(key2), set);
                    }

                    map1.insert(leak!(key1), map2);
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
    // set_opt!(attribute_filter, JsFunction, Fn(&str, &str, &str) -> &str);
    set_opt!(link_rel, JsString, Option<&str>);
    set_opt!(allowed_classes, JsObject, HashMap<&str, HashSet<&str>>);
    set_opt!(strip_comments, JsBoolean, bool);
    set_opt!(id_prefix, JsString, Option<&str>);

    Ok((builder, strings, cx))
}

fn clean(mut cx: FunctionContext) -> JsResult<JsString> {
    let input = cx.argument::<JsString>(0)?;
    let options = cx.argument::<JsObject>(1)?;

    let (builder, strings, mut cx) = build_from_arguments(cx, input, options)?;

    drop(strings);
    let output = builder.clean(&input.value()).to_string();
    Ok(cx.string(output))
}

register_module!(mut cx, {
    // cx.export_function("hello", hello)?;
    // cx.export_function("build", build)?;
    cx.export_function("clean", clean)?;
    Ok(())
});
