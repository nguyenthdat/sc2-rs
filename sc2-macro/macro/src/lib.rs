use proc_macro::TokenStream;
use quote::{format_ident, quote};
use regex::Regex;
use syn::{Attribute, Data, DeriveInput, Fields, ItemEnum, ItemFn, ItemStruct, parse_macro_input};

#[proc_macro_attribute]
pub fn bot(_attr: TokenStream, item: TokenStream) -> TokenStream {
	let item = parse_macro_input!(item as ItemStruct);

	let name = item.ident;
	let vis = item.vis;
	let attrs = item.attrs;
	let generics = item.generics;

	// Keep tuple structs rejected; allow unit or named.
	// In syn v2, `Fields` still implements `ToTokens` so `quote!{#other}` works for Unit. :contentReference[oaicite:0]{index=0}
	let fields_tokens = match &item.fields {
		Fields::Named(named) => {
			let named = &named.named;
			quote! { #named }
		}
		Fields::Unnamed(_) => panic!("#[bot] is not allowed for tuple structs"),
		other => quote! { #other }, // Unit → prints nothing inside braces below
	};

	let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

	TokenStream::from(quote! {
		#(#attrs)*
		#vis struct #name #ty_generics {
			_bot: sc2::bot::Bot,
			#fields_tokens
		}

		impl #impl_generics std::ops::Deref for #name #ty_generics #where_clause {
			type Target = sc2::bot::Bot;
			fn deref(&self) -> &Self::Target { &self._bot }
		}

		impl #impl_generics std::ops::DerefMut for #name #ty_generics #where_clause {
			fn deref_mut(&mut self) -> &mut Self::Target { &mut self._bot }
		}
	})
}

#[proc_macro_attribute]
pub fn bot_new(_attr: TokenStream, item: TokenStream) -> TokenStream {
	let item = parse_macro_input!(item as ItemFn);

	let vis = item.vis;
	let signature = item.sig;

	// In syn v2, Stmt::Expr carries an optional semicolon: `Stmt::Expr(Expr, Option<Semi>)`. :contentReference[oaicite:1]{index=1}
	let blocks = item.block.stmts.iter().map(|s| {
		if let syn::Stmt::Expr(expr, semi) = s {
			if let syn::Expr::Struct(struct_expr) = expr {
				let path = &struct_expr.path;
				// In syn v2, `ExprStruct.rest` is `Option<(Token![..], Expr)>`. :contentReference[oaicite:2]{index=2}
				let rest = struct_expr.rest.as_ref().map(|e| quote! { ..#e });

				let fields = struct_expr.fields.iter();

				let body = quote! {
					#path {
						_bot: Default::default(),
						#(#fields, )*
						#rest
					}
				};

				return if semi.is_some() {
					quote! { #body; }
				} else {
					quote! { #body }
				};
			}
		}
		// Fallback: keep the original statement as-is.
		quote! { #s }
	});

	TokenStream::from(quote! {
		#vis #signature {
			#(#blocks)*
		}
	})
}

#[proc_macro_derive(FromStr, attributes(enum_from_str))]
pub fn enum_from_str_derive(input: TokenStream) -> TokenStream {
	let item = parse_macro_input!(input as DeriveInput);
	if let Data::Enum(data) = item.data {
		let name = item.ident;
		let variants = data.variants.iter().map(|v| &v.ident);

		// `Attribute::parse_meta` and `NestedMeta` are gone in v2.
		// Use `Attribute::parse_nested_meta` and `attr.path().is_ident(..)`. :contentReference[oaicite:3]{index=3}
		let additional_attributes = |a: &Attribute| {
			if a.path().is_ident("enum_from_str") {
				let mut use_primitives = false;
				let _ = a.parse_nested_meta(|meta| {
					if meta.path.is_ident("use_primitives") {
						use_primitives = true;
					}
					Ok(())
				});
				return use_primitives;
			}
			false
		};

		let other_cases = if item.attrs.iter().any(additional_attributes) {
			quote! {
				n => {
					if let Ok(num) = n.parse() {
						if let Some(result) = Self::from_i64(num) {
							return Ok(result);
						}
					}
					return Err(sc2_macro::ParseEnumError);
				}
			}
		} else {
			quote! { _ => return Err(sc2_macro::ParseEnumError) }
		};

		TokenStream::from(quote! {
			impl std::str::FromStr for #name {
				type Err = sc2_macro::ParseEnumError;
				fn from_str(s: &str) -> Result<Self, Self::Err> {
					Ok(match s {
						#(
							stringify!(#variants) => Self::#variants,
						)*
						#other_cases,
					})
				}
			}
		})
	} else {
		panic!("Can only derive FromStr for enums")
	}
}

#[proc_macro_attribute]
pub fn variant_checkers(_attr: TokenStream, item: TokenStream) -> TokenStream {
	let item = parse_macro_input!(item as ItemEnum);

	let name = &item.ident;
	let variants = item.variants.iter().map(|v| &v.ident);
	let re = Regex::new(r"[A-Z0-9]{1}[a-z0-9]*").unwrap();
	let snake_variants = variants.clone().map(|v| {
		format_ident!(
			"is_{}",
			re.find_iter(&v.to_string())
				.map(|m| m.as_str().to_ascii_lowercase())
				.collect::<Vec<String>>()
				.join("_")
		)
	});

	TokenStream::from(quote! {
		#item
		impl #name {
			#(
				#[inline]
				pub fn #snake_variants(self) -> bool {
					matches!(self, Self::#variants)
				}
			)*
		}
	})
}
