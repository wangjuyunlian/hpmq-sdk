use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;

#[proc_macro]
pub fn sdk_init(_: TokenStream) -> TokenStream {
    let output = quote! {
        #[no_mangle]
        pub extern "C" fn wapc_init() {
            // 订阅topic转换
            // wapc_guest::prelude::register_function("subscribe", subscribe);
            wapc_guest::prelude::register_function("local-publish", local_publish);
            wapc_guest::prelude::register_function("cloud-publish", cloud_publish);
        }
    };
    output.into()
}
#[proc_macro_attribute]
pub fn downstream(_: TokenStream, attr: TokenStream) -> TokenStream {
    cloud_publish(AttrMacroKind::CloudPublish, attr.into()).into()
}
#[proc_macro_attribute]
pub fn upstream(_: TokenStream, attr: TokenStream) -> TokenStream {
    cloud_publish(AttrMacroKind::DevPublish, attr.into()).into()
}

enum AttrMacroKind {
    DevPublish,
    CloudPublish,
}

fn cloud_publish(kind: AttrMacroKind, token: TokenStream) -> TokenStream {
    let raw_main: ItemFn = syn::parse(token).unwrap();
    let ident = &raw_main.sig.ident;
    let output = match kind {
        AttrMacroKind::DevPublish => {
            quote! {
                fn local_publish(_msg: &[u8]) -> wapc_guest::prelude::CallResult {
                    let input = serde_json::from_slice::<PublishDatas>(_msg)?;
                    let PublishDatas {
                        topic,
                        payload,
                        context, his_payload
                    } = input;
                    let his_payload = his_payload.and_then(|x| Some(x.to_vec()));
                    let res: HandResult = #ident(topic, payload.as_ref(), his_payload, context)?;
                    Ok(serde_json::to_vec(&res)?)
                }
                #raw_main
            }
        }
        AttrMacroKind::CloudPublish => {
            quote! {
                fn cloud_publish(_msg: &[u8]) -> wapc_guest::prelude::CallResult {
                    let input = serde_json::from_slice::<PublishDatas>(_msg)?;
                    let PublishDatas {
                        topic,
                        payload,
                        context, his_payload
                    } = input;
                    let his_payload = his_payload.and_then(|x| Some(x.to_vec()));
                    let res: HandResult = #ident(topic, payload.as_ref(), his_payload, context)?;
                    Ok(serde_json::to_vec(&res)?)
                }
                #raw_main
            }
        }
    };
    output.into()
}
