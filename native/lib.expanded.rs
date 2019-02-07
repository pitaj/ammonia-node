#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std as std;
#[macro_use]
extern crate neon;
extern crate ammonia;
extern crate simple_interner;

use std::collections::{HashMap, HashSet};

use neon::prelude::*;
use ammonia::{Builder, UrlRelative, Url};
// UrlRelativeEvaluate};
use simple_interner::{Interner, Interned};

type Holder = Interner<str>;

fn build_from_arguments<'a>(cx: &'a mut FunctionContext,
                            options: Handle<JsObject>, holder: &'a Holder)
 -> Result<Builder<'a>, neon::result::Throw> {
    let mut builder = Builder::new();

    macro_rules! hold(( $ e : expr ) => {
                      {
                      let value = $ e . value (  ) ; Interned :: get (
                      & holder . get_or_insert ( value ) ) } });

    // conversions that need to be done
    // boolean -> bool
    // string | null -> Option(&str)
    // string ==> UrlRelative
    // string[] ==> HashSet<&str>
    // { [string]: string[] } ==> HashMap<&str, HashSet<&str>>
    // { [string]: { [string]: string[] }} ==> MashMap<&str, HashMap<&str, HashSet<&str>>>
    // (string, string, string): string | null ==> Fn(&str, &str, &str) -> Option(&str)

    /// convert!(subject, JS Type, Rust Type)
    macro_rules! set_opt(( $ option_name : ident , JsBoolean , bool ) => {
                         {
                         let prop = options . get (
                         cx , stringify ! ( $ option_name ) ) ? ; if prop .
                         is_a :: < JsBoolean > (  ) {
                         let val = prop . downcast_or_throw :: < JsBoolean ,
                         FunctionContext > ( cx ) ? ; builder . $ option_name
                         ( val . value (  ) ) ; } } } ; (
                         $ option_name : ident , JsString , Option < & str > )
                         => {
                         {
                         let prop = options . get (
                         cx , stringify ! ( $ option_name ) ) ? ; if prop .
                         is_a :: < JsNull > (  ) {
                         builder . $ option_name ( None ) ; } else if prop .
                         is_a :: < JsString > (  ) {
                         let val = prop . downcast_or_throw :: < JsString ,
                         FunctionContext > ( cx ) ? ; builder . $ option_name
                         ( Some ( hold ! ( val ) ) ) ; } } } ; (
                         $ option_name : ident , JsString , UrlRelative ) => {
                         {
                         let prop = options . get (
                         cx , stringify ! ( $ option_name ) ) ? ; if prop .
                         is_a :: < JsString > (  ) {
                         let val = prop . downcast_or_throw :: < JsString ,
                         FunctionContext > ( cx ) ? ; match val . value (  ) .
                         as_ref (  ) {
                         "deny" => {
                         builder . $ option_name ( UrlRelative :: Deny ) ; } ,
                         "pass-through" => {
                         builder . $ option_name ( UrlRelative :: PassThrough
                         ) ; } , _ => cx . throw_type_error (
                         "Invalid `url_relative` option" ) ? , } } else if
                         prop . is_a :: < JsArray > (  ) {
                         let arr = prop . downcast_or_throw :: < JsArray ,
                         FunctionContext > ( cx ) ? ; let val = arr . get (
                         cx , 0 ) ? ; let val = val . downcast_or_throw :: <
                         JsString , FunctionContext > ( cx ) ? ; match val .
                         value (  ) . as_ref (  ) {
                         "resolve-with-base" => {
                         let base = arr . get ( cx , 1 ) ? ; let base = base .
                         downcast_or_throw :: < JsString , FunctionContext > (
                         cx ) ? ; let url = Url :: parse ( & base . value (  )
                         ) . unwrap (  ) ; builder . $ option_name (
                         UrlRelative :: RewriteWithBase ( url ) ) ; } , _ =>
                         unimplemented ! (  ) , } } } } ; (
                         $ option_name : ident , JsArray , HashSet < & str > )
                         => {
                         {
                         let prop = options . get (
                         cx , stringify ! ( $ option_name ) ) ? ; if prop .
                         is_a :: < JsArray > (  ) {
                         let val = prop . downcast_or_throw :: < JsArray ,
                         FunctionContext > ( cx ) ? ; let mut set : HashSet <
                         & str > = HashSet :: with_capacity (
                         val . len (  ) as usize ) ; for x in val . to_vec (
                         cx ) ? {
                         let s = x . downcast_or_throw :: < JsString ,
                         FunctionContext > ( cx ) ? ; set . insert (
                         hold ! ( s ) ) ; } builder . $ option_name ( set ) ;
                         } } } ; (
                         $ option_name : ident , JsObject , HashMap < & str ,
                         HashSet < & str >> ) => {
                         {
                         let prop = options . get (
                         cx , stringify ! ( $ option_name ) ) ? ; if prop .
                         is_a :: < JsObject > (  ) {
                         let obj = prop . downcast_or_throw :: < JsObject ,
                         FunctionContext > ( cx ) ? ; let prop_names = obj .
                         get_own_property_names ( cx ) ? ; let mut map :
                         HashMap < & str , HashSet < & str >> = HashMap ::
                         with_capacity ( prop_names . len (  ) as usize ) ;
                         for key in prop_names . to_vec ( cx ) ? {
                         let key = key . downcast_or_throw :: < JsString ,
                         FunctionContext > ( cx ) ? ; let arr = obj . get (
                         cx , key ) ? ; let arr = arr . downcast_or_throw :: <
                         JsArray , FunctionContext > ( cx ) ? ; let mut set :
                         HashSet < & str > = HashSet :: with_capacity (
                         arr . len (  ) as usize ) ; for x in arr . to_vec (
                         cx ) ? {
                         let s = x . downcast_or_throw :: < JsString ,
                         FunctionContext > ( cx ) ? ; set . insert (
                         hold ! ( s ) ) ; } map . insert (
                         hold ! ( key ) , set ) ; } builder . $ option_name (
                         map ) ; } } } ; (
                         $ option_name : ident , JsObject , HashMap < & str ,
                         HashMap < & str , HashSet < & str >> > ) => {
                         {
                         let prop = options . get (
                         cx , stringify ! ( $ option_name ) ) ? ; if prop .
                         is_a :: < JsObject > (  ) {
                         let obj1 = prop . downcast_or_throw :: < JsObject ,
                         FunctionContext > ( cx ) ? ; let prop_names1 = obj1 .
                         get_own_property_names ( cx ) ? ; let mut map1 :
                         HashMap < & str , HashMap < & str , HashSet < & str
                         >> > = HashMap :: with_capacity (
                         prop_names1 . len (  ) as usize ) ; for key1 in
                         prop_names1 . to_vec ( cx ) ? {
                         let key1 = key1 . downcast_or_throw :: < JsString ,
                         FunctionContext > ( cx ) ? ; let obj2 = obj1 . get (
                         cx , key1 ) ? ; let obj2 = obj2 . downcast_or_throw
                         :: < JsObject , FunctionContext > ( cx ) ? ; let
                         prop_names2 = obj2 . get_own_property_names ( cx ) ?
                         ; let mut map2 : HashMap < & str , HashSet < & str >>
                         = HashMap :: with_capacity (
                         prop_names2 . len (  ) as usize ) ; for key2 in
                         prop_names2 . to_vec ( cx ) ? {
                         let key2 = key2 . downcast_or_throw :: < JsString ,
                         FunctionContext > ( cx ) ? ; let arr = obj2 . get (
                         cx , key2 ) ? ; let arr = arr . downcast_or_throw ::
                         < JsArray , FunctionContext > ( cx ) ? ; let mut set
                         : HashSet < & str > = HashSet :: with_capacity (
                         arr . len (  ) as usize ) ; for x in arr . to_vec (
                         cx ) ? {
                         let s = x . downcast_or_throw :: < JsString ,
                         FunctionContext > ( cx ) ? ; set . insert (
                         hold ! ( s ) ) ; } map2 . insert (
                         hold ! ( key2 ) , set ) ; } map1 . insert (
                         hold ! ( key1 ) , map2 ) ; } builder . $ option_name
                         ( map1 ) ; } } } ;);




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
    {
        let prop = options.get(cx, "tags")?;
        if prop.is_a::<JsArray>() {
            let val = prop.downcast_or_throw::<JsArray, FunctionContext>(cx)?;
            let mut set: HashSet<&str> =
                HashSet::with_capacity(val.len() as usize);

            for x in val.to_vec(cx)? {
                let s = x.downcast_or_throw::<JsString, FunctionContext>(cx)?;
                set.insert({
                               let value = s.value();
                               Interned::get(&holder.get_or_insert(value))
                           });
            }

            builder.tags(set);
        }
    };
    {
        let prop = options.get(cx, "clean_content_tags")?;
        if prop.is_a::<JsArray>() {
            let val = prop.downcast_or_throw::<JsArray, FunctionContext>(cx)?;
            let mut set: HashSet<&str> =
                HashSet::with_capacity(val.len() as usize);
            for x in val.to_vec(cx)? {
                let s = x.downcast_or_throw::<JsString, FunctionContext>(cx)?;
                set.insert({
                               let value = s.value();
                               Interned::get(&holder.get_or_insert(value))
                           });
            }
            builder.clean_content_tags(set);
        }
    };
    {
        let prop = options.get(cx, "tag_attributes")?;
        if prop.is_a::<JsObject>() {
            let obj =
                prop.downcast_or_throw::<JsObject, FunctionContext>(cx)?;
            let prop_names = obj.get_own_property_names(cx)?;

            let mut map: HashMap<&str, HashSet<&str>> =
                HashMap::with_capacity(prop_names.len() as usize);
            for key in prop_names.to_vec(cx)? {
                let key =
                    key.downcast_or_throw::<JsString, FunctionContext>(cx)?;

                let arr = obj.get(cx, key)?;
                let arr =
                    arr.downcast_or_throw::<JsArray, FunctionContext>(cx)?;

                let mut set: HashSet<&str> =
                    HashSet::with_capacity(arr.len() as usize);

                for x in arr.to_vec(cx)? {
                    let s =
                        x.downcast_or_throw::<JsString, FunctionContext>(cx)?;
                    set.insert({
                                   let value = s.value();
                                   Interned::get(&holder.get_or_insert(value))
                               });
                }

                map.insert({
                               let value = key.value();
                               Interned::get(&holder.get_or_insert(value))
                           }, set);
            }

            builder.tag_attributes(map);
        }
    };
    {
        let prop = options.get(cx, "tag_attribute_values")?;
        if prop.is_a::<JsObject>() {
            let obj1 =
                prop.downcast_or_throw::<JsObject, FunctionContext>(cx)?;
            let prop_names1 = obj1.get_own_property_names(cx)?;

            let mut map1: HashMap<&str, HashMap<&str, HashSet<&str>>> =
                HashMap::with_capacity(prop_names1.len() as usize);
            for key1 in prop_names1.to_vec(cx)? {
                let key1 =
                    key1.downcast_or_throw::<JsString, FunctionContext>(cx)?;

                let obj2 = obj1.get(cx, key1)?;
                let obj2 =
                    obj2.downcast_or_throw::<JsObject, FunctionContext>(cx)?;
                let prop_names2 = obj2.get_own_property_names(cx)?;

                let mut map2: HashMap<&str, HashSet<&str>> =
                    HashMap::with_capacity(prop_names2.len() as usize);
                for key2 in prop_names2.to_vec(cx)? {
                    let key2 =
                        key2.downcast_or_throw::<JsString,
                                                 FunctionContext>(cx)?;

                    let arr = obj2.get(cx, key2)?;
                    let arr =
                        arr.downcast_or_throw::<JsArray,
                                                FunctionContext>(cx)?;

                    let mut set: HashSet<&str> =
                        HashSet::with_capacity(arr.len() as usize);

                    for x in arr.to_vec(cx)? {
                        let s =
                            x.downcast_or_throw::<JsString,
                                                  FunctionContext>(cx)?;
                        set.insert({
                                       let value = s.value();
                                       Interned::get(&holder.get_or_insert(value))
                                   });
                    }

                    map2.insert({
                                    let value = key2.value();
                                    Interned::get(&holder.get_or_insert(value))
                                }, set);
                }

                map1.insert({
                                let value = key1.value();
                                Interned::get(&holder.get_or_insert(value))
                            }, map2);
            }

            builder.tag_attribute_values(map1);
        }
    };
    {
        let prop = options.get(cx, "generic_attributes")?;
        if prop.is_a::<JsArray>() {
            let val = prop.downcast_or_throw::<JsArray, FunctionContext>(cx)?;
            let mut set: HashSet<&str> =
                HashSet::with_capacity(val.len() as usize);
            for x in val.to_vec(cx)? {
                let s = x.downcast_or_throw::<JsString, FunctionContext>(cx)?;
                set.insert({
                               let value = s.value();
                               Interned::get(&holder.get_or_insert(value))
                           });
            }
            builder.generic_attributes(set);
        }
    };
    {
        let prop = options.get(cx, "url_schemes")?;
        if prop.is_a::<JsArray>() {
            let val = prop.downcast_or_throw::<JsArray, FunctionContext>(cx)?;
            let mut set: HashSet<&str> =
                HashSet::with_capacity(val.len() as usize);
            for x in val.to_vec(cx)? {
                let s = x.downcast_or_throw::<JsString, FunctionContext>(cx)?;
                set.insert({
                               let value = s.value();
                               Interned::get(&holder.get_or_insert(value))
                           });
            }
            builder.url_schemes(set);
        }
    };
    {
        let prop = options.get(cx, "url_relative")?;
        if prop.is_a::<JsString>() {
            let val =
                prop.downcast_or_throw::<JsString, FunctionContext>(cx)?;
            match val.value().as_ref() {
                "deny" => { builder.url_relative(UrlRelative::Deny); }
                "pass-through" => {
                    builder.url_relative(UrlRelative::PassThrough);
                }
                _ => cx.throw_type_error("Invalid `url_relative` option")?,
            }
        } else if prop.is_a::<JsArray>() {
            let arr = prop.downcast_or_throw::<JsArray, FunctionContext>(cx)?;
            let val = arr.get(cx, 0)?;
            let val = val.downcast_or_throw::<JsString, FunctionContext>(cx)?;
            match val.value().as_ref() {
                "resolve-with-base" => {
                    let base = arr.get(cx, 1)?;
                    let base =
                        base.downcast_or_throw::<JsString,
                                                 FunctionContext>(cx)?;
                    let url = Url::parse(&base.value()).unwrap();
                    builder.url_relative(UrlRelative::RewriteWithBase(url));
                }
                _ =>
                // TODO
                // set_opt!(attribute_filter, JsFunction, Fn(&str, &str, &str) -> &str);









                // m.export_class::<JsCleaner>("Cleaner")?;
                {
                    ::std::rt::begin_panic("not yet implemented",
                                           &("src/lib.rs", 189u32, 5u32))
                }
            }
        }
    };
    {
        let prop = options.get(cx, "link_rel")?;
        if prop.is_a::<JsNull>() {
            builder.link_rel(None);
        } else if prop.is_a::<JsString>() {
            let val =
                prop.downcast_or_throw::<JsString, FunctionContext>(cx)?;
            builder.link_rel(Some({
                                      let value = val.value();
                                      Interned::get(&holder.get_or_insert(value))
                                  }));
        }
    };
    {
        let prop = options.get(cx, "allowed_classes")?;
        if prop.is_a::<JsObject>() {
            let obj =
                prop.downcast_or_throw::<JsObject, FunctionContext>(cx)?;
            let prop_names = obj.get_own_property_names(cx)?;
            let mut map: HashMap<&str, HashSet<&str>> =
                HashMap::with_capacity(prop_names.len() as usize);
            for key in prop_names.to_vec(cx)? {
                let key =
                    key.downcast_or_throw::<JsString, FunctionContext>(cx)?;
                let arr = obj.get(cx, key)?;
                let arr =
                    arr.downcast_or_throw::<JsArray, FunctionContext>(cx)?;
                let mut set: HashSet<&str> =
                    HashSet::with_capacity(arr.len() as usize);
                for x in arr.to_vec(cx)? {
                    let s =
                        x.downcast_or_throw::<JsString, FunctionContext>(cx)?;
                    set.insert({
                                   let value = s.value();
                                   Interned::get(&holder.get_or_insert(value))
                               });
                }
                map.insert({
                               let value = key.value();
                               Interned::get(&holder.get_or_insert(value))
                           }, set);
            }
            builder.allowed_classes(map);
        }
    };
    {
        let prop = options.get(cx, "strip_comments")?;
        if prop.is_a::<JsBoolean>() {
            let val =
                prop.downcast_or_throw::<JsBoolean, FunctionContext>(cx)?;
            builder.strip_comments(val.value());
        }
    };
    {
        let prop = options.get(cx, "id_prefix")?;
        if prop.is_a::<JsNull>() {
            builder.id_prefix(None);
        } else if prop.is_a::<JsString>() {
            let val =
                prop.downcast_or_throw::<JsString, FunctionContext>(cx)?;
            builder.id_prefix(Some({
                                       let value = val.value();
                                       Interned::get(&holder.get_or_insert(value))
                                   }));
        }
    };
    Ok(builder)
}
fn clean(mut cx: FunctionContext) -> JsResult<JsString> {
    let input = cx.argument::<JsString>(0)?;
    let options = cx.argument::<JsObject>(1)?;
    let cleaned =
        {
            let holder = Interner::new();
            let builder = build_from_arguments(&mut cx, options, &holder)?;
            let doc = builder.clean(&input.value());
            doc.to_string()
        };
    Ok(cx.string(cleaned))
}
pub struct Cleaner<'a> {
    holder: Holder,
    builder: Builder<'a>,
}
#[repr(C)]
#[rustc_copy_clone_marker]
pub struct JsCleaner(::neon::macro_internal::runtime::raw::Local);
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::marker::Copy for JsCleaner { }
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for JsCleaner {
    #[inline]
    fn clone(&self) -> JsCleaner {
        {
            let _:
                    ::std::clone::AssertParamIsClone<::neon::macro_internal::runtime::raw::Local>;
            *self
        }
    }
}
impl ::neon::handle::Managed for JsCleaner {
    fn to_raw(self) -> ::neon::macro_internal::runtime::raw::Local {
        let JsCleaner(raw) = self;
        raw
    }
    fn from_raw(raw: ::neon::macro_internal::runtime::raw::Local) -> Self {
        JsCleaner(raw)
    }
}
impl ::neon::object::Class for JsCleaner {
    type
    Internals
    =
    Cleaner;
    fn setup<'a, C: ::neon::context::Context<'a>>(_: &mut C)
     ->
         ::neon::result::NeonResult<::neon::object::ClassDescriptor<'a,
                                                                    Self>> {
        ::std::result::Result::Ok(Self::describe("Cleaner",
                                                 {
                                                     fn _______allocator_rust_y_u_no_hygienic_items_______(mut cx:
                                                                                                               ::neon::context::CallContext<::neon::types::JsUndefined>)
                                                      ->
                                                          ::neon::result::NeonResult<Cleaner> {
                                                         {
                                                             let options =
                                                                 cx.argument::<JsObject>(0)?;
                                                             let holder =
                                                                 Interner::new();
                                                             let builder =
                                                                 build_from_arguments(&mut cx,
                                                                                      options,
                                                                                      &holder)?;
                                                             Ok(Cleaner{holder,
                                                                        builder,})
                                                         }
                                                     }
                                                     ::neon::macro_internal::AllocateCallback(_______allocator_rust_y_u_no_hygienic_items_______)
                                                 }))
    }
}
#[allow(improper_ctypes)]
#[link_section = ".ctors"]
pub static __LOAD_NEON_MODULE: extern "C" fn() =
    {
        fn __init_neon_module(mut m: ::neon::context::ModuleContext)
         -> ::neon::result::NeonResult<()> {
            m.export_function("clean", clean)?;
            Ok(())
        }
        extern "C" fn __load_neon_module() {
            #[repr(C)]
            struct __NodeModule {
                version: i32,
                flags: u32,
                dso_handle: *mut u8,
                filename: *const u8,
                register_func: Option<extern "C" fn(::neon::handle::Handle<::neon::types::JsObject>,
                                                    *mut u8, *mut u8)>,
                context_register_func: Option<extern "C" fn(::neon::handle::Handle<::neon::types::JsObject>,
                                                            *mut u8, *mut u8,
                                                            *mut u8)>,
                modname: *const u8,
                priv_data: *mut u8,
                link: *mut __NodeModule,
            }
            static mut __NODE_MODULE: __NodeModule =
                __NodeModule{version: 0,
                             flags: 0,
                             dso_handle: 0 as *mut _,
                             filename: b"neon_source.rs\x00" as *const u8,
                             register_func: Some(__register_neon_module),
                             context_register_func: None,
                             modname: b"neon_module\x00" as *const u8,
                             priv_data: 0 as *mut _,
                             link: 0 as *mut _,};
            extern "C" fn __register_neon_module(m:
                                                     ::neon::handle::Handle<::neon::types::JsObject>,
                                                 _: *mut u8, _: *mut u8) {
                ::neon::macro_internal::initialize_module(m,
                                                          __init_neon_module);
            }
            extern "C" {
                fn node_module_register(module: *mut __NodeModule);
            }
            ::std::panic::set_hook(::std::boxed::Box::new(|_| { }));
            unsafe {
                __NODE_MODULE.version =
                    ::neon::macro_internal::runtime::module::get_version();
                node_module_register(&mut __NODE_MODULE);
            }
        }
        __load_neon_module
    };
