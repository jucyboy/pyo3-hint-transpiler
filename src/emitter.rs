use crate::types;
use crate::imports::Imports;
use syn::{
    File, Item, Fields, Type, FnArg, PathArguments, GenericArgument, ImplItem,
};
use quote::ToTokens;

fn rust_to_py(ty: &Type, trans: &types::Transpiles, class_name: Option<&str>) -> String {
    match ty {
        Type::Path(p) => {
            let seg = &p.path.segments.last().unwrap();
            let name = seg.ident.to_string();
            if name == "Self" {
                return class_name.unwrap_or("Any").to_string();
            }

            let args = if let PathArguments::AngleBracketed(ab) = &seg.arguments {
                ab.args.iter().filter_map(|a| {
                    if let GenericArgument::Type(t) = a {
                        Some(rust_to_py(t, trans, class_name))
                    } else { None }
                }).collect::<Vec<_>>()
            } else { vec![] };

            let mapped = trans.primitives.get(&name)
                .or_else(|| trans.strings.get(&name))
                .or_else(|| trans.wrappers.get(&name))
                .or_else(|| trans.collections.get(&name))
                .or_else(|| trans.pyo3.get(&name))
                .unwrap_or(&name)
                .clone();

            if mapped.contains("T") && !args.is_empty() {
                mapped.replace("T", &args.join(", "))
            } else { mapped }
        }
        Type::Reference(r) => rust_to_py(&*r.elem, trans, class_name),
        Type::Tuple(t) if t.elems.is_empty() => "None".to_string(),
        _ => "Any".to_string(),
    }
}

pub fn emit(ast: File) -> anyhow::Result<String> {
    let trans = types::load();
    let mut imports = Imports::new();
    let mut body = String::new();
    let mut emitted_classes = std::collections::HashSet::new();

    for item in ast.items {
        match item {
            Item::Fn(f) => {
                let name = imports.register(&f.sig.ident.to_string());
                let args = f.sig.inputs.iter().map(|a| match a {
                    FnArg::Receiver(_) => "self".to_string(),
                    FnArg::Typed(p) => {
                        let ty = rust_to_py(&*p.ty, &trans, None);
                        format!("{}: {}", p.pat.to_token_stream(), ty)
                    }
                }).collect::<Vec<_>>().join(", ");

                let ret = match &f.sig.output {
                    syn::ReturnType::Default => "None".to_string(),
                    syn::ReturnType::Type(_, t) => rust_to_py(&*t, &trans, None),
                };

                body.push_str(&format!("def {}({}) -> {}:\n    ...\n\n", name, args, ret));
            }

            Item::Struct(s) => {
                let name = imports.register(&s.ident.to_string());
                if emitted_classes.insert(name.clone()) {
                    body.push_str(&format!("class {}:\n", name));
                    if let Fields::Named(fields) = &s.fields {
                        for f in &fields.named {
                            let ty = rust_to_py(&f.ty, &trans, Some(&name));
                            let fname = f.ident.as_ref().unwrap().to_string();
                            body.push_str(&format!("    {}: {}\n", fname, ty));
                        }
                    }
                    body.push('\n');
                }
            }

            Item::Impl(imp) => {
                if let Type::Path(p) = *imp.self_ty.clone() {
                    let class_name = p.path.segments.last().unwrap().ident.to_string();
                    if emitted_classes.insert(class_name.clone()) {
                        body.push_str(&format!("class {}:\n", class_name));
                    }

                    for item in imp.items {
                        if let ImplItem::Fn(func) = item {
                            let fname = imports.register(&func.sig.ident.to_string());

                            let args_vec = func.sig.inputs.iter().map(|a| match a {
                                FnArg::Receiver(_) => "self".to_string(),
                                FnArg::Typed(p) => {
                                    let ty = rust_to_py(&*p.ty, &trans, Some(&class_name));
                                    format!("{}: {}", p.pat.to_token_stream(), ty)
                                }
                            }).collect::<Vec<_>>();

                            let args = if !args_vec.is_empty() && !args_vec[0].starts_with("self") {
                                format!("self, {}", args_vec.join(", "))
                            } else {
                                args_vec.join(", ")
                            };

                            let ret = match &func.sig.output {
                                syn::ReturnType::Default => "None".to_string(),
                                syn::ReturnType::Type(_, t) => rust_to_py(&*t, &trans, Some(&class_name)),
                            };

                            body.push_str(&format!("    def {}({}) -> {}:\n        ...\n\n", fname, args, ret));
                        }
                    }
                }
            }

            _ => {}
        }
    }

    let mut out = imports.render();
    if !out.is_empty() {
        out.push('\n');
    }
    out.push_str(&body.trim_end());
    out.push('\n');

    Ok(out)
}
