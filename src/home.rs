use {
    near_anywhere::{
        client::Transport,
        key_store::KeyStore,
        near::{connect, NearConfig, WalletConnection},
        signer::Signer,
    },
    wasm_bindgen_futures::spawn_local,
    yew::prelude::*,
};

#[derive(Properties, PartialEq)]
pub struct Props {}

#[function_component]
pub fn HomePage(_: &Props) -> Html {
    let trigger = use_force_update();
    let wallet_connection = {
        let node_url = "https://rpc.testnet.near.org";
        let key_store = KeyStore::browser_local_storage_key_store();
        let signer = Signer::new_in_memory_signer(key_store.clone());
        let mut config = NearConfig::new(node_url, key_store);
        config.network_id = Some("testnet".to_string());
        config.wallet_url = Some("https://wallet.testnet.near.org".to_string());
        config.helper_url = Some("https://helper.testnet.near.org".to_string());
        config.explorer_url = Some("https://explorer.testnet.near.org".to_string());
        config.transport = Some(Transport::http(node_url));
        config.signer = Some(signer);

        let connection = connect(config);
        let wallet_connection = WalletConnection::new(connection);

        wallet_connection
    };

    let is_signed_in = {
        let mut wallet_connection = wallet_connection.clone();
        wallet_connection.complete_sign_in_with_access_key();
        wallet_connection.is_signed_in().clone()
    };

    let handle_sign_in = {
        let wallet_connection = wallet_connection.clone();
        let trigger = trigger.clone();

        Callback::from(move |_| {
            let trigger = trigger.clone();
            let wallet_connection = wallet_connection.clone();
            spawn_local(async move {
                wallet_connection
                    .request_sign_in(
                        Some("test.testnet".to_string()),
                        Some(vec![]),
                        Some("http://localhost:8080/".to_string()),
                        Some("http://localhost:8080/".to_string()),
                    )
                    .await;
                trigger.force_update();
            });
        })
    };

    let handle_sign_out = {
        let wallet_connection = wallet_connection.clone();

        Callback::from(move |_| {
            let mut wallet_connection = wallet_connection.clone();
            wallet_connection.sign_out();
            trigger.force_update();
        })
    };

    html! {
        <div class="w-screen h-screen flex items-center justify-center">
            <div class="flex flex-col items-center justify-center">
                {
                    match is_signed_in {
                        true => {
                            html!{
                                <>
                                    <p>{"Signed In: Yes"}</p>
                                    <br/>
                                    <button onclick={handle_sign_out}>{"Log out"}</button>

                                </>
                            }
                        }
                        false => {
                            html!{
                                <>
                                    <p>{"Signed In: No"}</p>
                                    <br/>
                                    <button onclick={handle_sign_in}>{"Login"}</button>
                                </>
                            }
                        }
                    }
                }
            </div>
        </div>
    }
}
