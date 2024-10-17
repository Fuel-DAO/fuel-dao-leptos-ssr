use candid::Principal;
use ic_agent::identity::Secp256k1Identity;
use ic_agent::Identity;
use k256::elliptic_curve::rand_core::OsRng;
use k256::elliptic_curve::zeroize::Zeroize;
use k256::elliptic_curve::{JwkEcKey, SecretKey};
use k256::Secp256k1;
use leptos::*;
use leptos_router::*;

use crate::consts::{USER_PRINCIPAL_ID_STORE, USER_PRINCIPAL_JWK};
use crate::state::canisters::do_canister_auth;
use crate::utils::ParentResource;
use crate::{
    auth::DelegatedIdentityWire,
    components::spinners::FullScreenSpinner,
    state::{
        auth::AuthState,
        canisters::{AuthCansResource, Canisters},
    },
    try_or_redirect,
    utils::MockPartialEq,
};
use codee::string::JsonSerdeCodec;
use leptos_use::storage::use_local_storage;

#[derive(Params, PartialEq, Clone)]
struct Referrer {
    user_refer: String,
}

#[component]
fn CtxProvider(temp_identity: Option<JwkEcKey>, children: ChildrenFn) -> impl IntoView {
    let auth = AuthState::default();
    provide_context(auth);

    let canisters_store = create_rw_signal(None::<Canisters<true>>);
    provide_context(canisters_store);

    let temp_identity_c = temp_identity.clone();
    create_local_resource(
        || (),
        move |_| {
            let temp_identity = temp_identity_c.clone();
            async move {
                let Some(id) = temp_identity else {
                    return;
                };
                if let Err(e) = generate_and_save_identity(id).await {
                    leptos::logging::log!("Failed to set anonymous identity as cookie?! err {e}");
                }
            }
        },
    );

    let canisters_res: AuthCansResource = ParentResource(create_resource(
        move || MockPartialEq(auth()),
        move |auth_id| {
            let temp_identity = temp_identity.clone();
            async move {
                if let Some(id_wire) = auth_id.0 {
                    return do_canister_auth(id_wire).await;
                }

                let Some(jwk_key) = temp_identity else {
                    let identity = match  extract_identity().await {
                        Ok(id) => id, 
                        Err(_) => return Err(ServerFnError::new("Failed to extract identity"))
                    };
                    let id_wire = identity.expect("No refresh cookie set?!");
                    return do_canister_auth(id_wire).await;
                };

                let key = k256::SecretKey::from_jwk(&jwk_key)?;
                let id = Secp256k1Identity::from_private_key(key);
                let id_wire = DelegatedIdentityWire::delegate(&id);

                do_canister_auth(id_wire).await
            }
        },
    ));
    provide_context(canisters_res.clone());

    view! {
        {children}
        <Suspense>
            {move || {
                (canisters_res.0)()
                    .map(|res| {
                        let cans_wire = try_or_redirect!(res);
                        let cans = try_or_redirect!(cans_wire.canisters());
                        let (_, set_user_principal_id, _) = use_local_storage::<
                            Option<Principal>,
                            JsonSerdeCodec,
                        >(USER_PRINCIPAL_ID_STORE);
                        set_user_principal_id(Some(cans.user_principal()));
                        canisters_store.set(Some(cans));
                    })
            }}

        </Suspense>
    }
}

#[component]
pub fn BaseRoute() -> impl IntoView {
    let temp_identity_res = create_blocking_resource(
        || (),
        |_| async move {
            generate_anonymous_identity_if_required()
                .await
                .expect("Failed to generate anonymous identity?!")
        },
    );

    view! {
        <Suspense fallback=FullScreenSpinner>
            {move || {
                temp_identity_res()
                    .map(|temp_identity| {
                        view! {
                            <CtxProvider temp_identity>
                                <Outlet/>
                            </CtxProvider>
                        }
                    })
            }}

        </Suspense>
    }
}

async fn generate_anonymous_identity_if_required() -> Result<Option<JwkEcKey>, ()> {
    if extract_principal_from_local_storage()?.is_some() {
        return Ok(None);
    }
    let key = k256::SecretKey::random(&mut OsRng);
    Ok(Some(key.to_jwk()))
}


pub async fn extract_identity() -> Result<Option<DelegatedIdentityWire>, ()> {

    let base_identity = if let Some(identity) = try_extract_identity().await? {
        Secp256k1Identity::from_private_key(identity)
    } else {
        return Ok(None);
    };

    Ok(Some(DelegatedIdentityWire::delegate(&base_identity)))
}

pub async fn try_extract_identity(
) -> Result<Option<k256::SecretKey>, ()> {
    let Some(principal) = extract_principal_from_local_storage()? else {
        return Ok(None);
    };
    fetch_identity_from_kv()
}

fn fetch_identity_from_kv(
) -> Result<Option<k256::SecretKey>, ()> {
    
    let Some(identity_jwk) = user_principal_jwk().0.get() else {
        return Ok(None);
    };

    Ok(Some(k256::SecretKey::from_jwk_str(&identity_jwk).unwrap()))
}

pub fn extract_principal_from_local_storage(
) -> Result<Option<Principal>, ()> {
    let (get, _, _) = user_principal();
    Ok(get.get())
}

async fn generate_and_save_identity(id: JwkEcKey) -> Result<Secp256k1Identity, ServerFnError> {
    // let mut id = id.clone();
    let secret: SecretKey<Secp256k1> =  id.to_secret_key()?;
    let base_identity_key = secret /* k256::SecretKey::random(&mut OsRng) */;
    let base_identity = Secp256k1Identity::from_private_key(base_identity_key.clone());
    let principal = base_identity.sender().unwrap();

    let base_jwk = base_identity_key.to_jwk_string();
    let (_, set, _) = user_principal_jwk();
    let (_, set_principal, _) = user_principal();
    set_principal.set(Some(principal));
    set.set(Some(base_jwk.to_string()));
    Ok(base_identity)
}


pub fn user_principal_jwk() -> (
    Signal<Option<String>>,
    WriteSignal<Option<String>>,
    impl Fn() + Clone,
) {
    use_local_storage::<Option<String>, JsonSerdeCodec>(USER_PRINCIPAL_JWK)
}
pub fn user_principal() -> (
    Signal<Option<Principal>>,
    WriteSignal<Option<Principal>>,
    impl Fn() + Clone,
) {
    use_local_storage::<Option<Principal>, JsonSerdeCodec>(USER_PRINCIPAL_ID_STORE)
}