use gloo_net::http::Request;
use leptos::prelude::*;
use serde::Deserialize;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;

// Local imports
use crate::components::{currency_input::CurrencyInput, footer::Footer, header::Header};
use crate::utils::units::*;

// ---------------- Constants ----------------
/// Base URL for backend API.
const API_BASE_URL: &str = "https://untron.finance/api/v2-public";

// ------------ Helper structs ---------------
#[derive(Deserialize)]
struct ApiInfoResponse {
    #[serde(rename = "availableLiquidity")]
    available_liquidity: String,
}

#[derive(Deserialize)]
struct CreateOrderResponse {
    id: String,
}

// ------------ Helper functions -------------
fn is_valid_evm_address(addr: &str) -> bool {
    addr.len() == 42
        && addr.starts_with("0x")
        && addr.chars().skip(2).all(|c| c.is_ascii_hexdigit())
}

#[component]
pub fn Home() -> impl IntoView {
    // ---------------------- STATE ----------------------
    // Amounts to send / receive. These are just plain text strings for now.
    let send_amount = RwSignal::new(String::new());
    let receive_amount = RwSignal::new(String::new());

    // Address badge vs. free-text input.
    let (address_badge, set_address_badge) = signal::<Option<String>>(None);
    let (input_value, set_input_value) = signal(String::new());

    let max_order_output = RwSignal::new(100000000_u64); // TODO: change
    let is_swapping = RwSignal::new(false);
    let error_message = RwSignal::new(None::<String>);

    // Fetch /info once on mount.
    {
        let max_order_output = max_order_output.clone();
        spawn_local(async move {
            if let Ok(resp) = Request::get(&format!("{API_BASE_URL}/info")).send().await {
                if resp.ok() {
                    if let Ok(json) = resp.json::<ApiInfoResponse>().await {
                        if let Ok(liq) = json.available_liquidity.parse::<u128>() {
                            max_order_output.set(liq.min(u128::from(u64::MAX)) as u64);
                        }
                    }
                }
            }
        });
    }

    // -------- Reactive send → receive conversion -------
    {
        let send_amount = send_amount.clone();
        let receive_amount = receive_amount.clone();
        Effect::new(move |_| {
            let s = send_amount.get();
            if s.is_empty() {
                receive_amount.set(String::new());
                return;
            }
            if let Some(units) = string_to_units(&s) {
                let recv_units = convert_send_to_receive(units, SWAP_RATE_UNITS);
                receive_amount.set(units_to_string(recv_units));
            } else {
                receive_amount.set(String::new());
            }
        });
    }

    // -------- Clipboard / paste handling ---------------
    let on_paste = {
        let set_address_badge = set_address_badge.clone();
        move |_| {
            // TODO: implement clipboard reading once the Clipboard API is fully wired.
            // For now, use a placeholder address to simulate the behaviour.
            set_address_badge.set(Some("0xAbCd…1234".to_string()));
        }
    };

    // Clear the badge and return to input mode.
    let clear_badge = move |_| {
        set_address_badge.set(None);
    };

    // ------- Derived UI fragments -------
    let address_view = move || {
        view! {
            <Show
                when=move || address_badge.get().is_some()
                fallback=move || {
                    view! {
                        <input
                            type="text"
                            class="w-full outline-none text-black text-lg font-medium bg-transparent"
                            prop:value=input_value
                            on:input:target=move |ev| {
                                set_input_value.set(ev.target().value());
                            }
                            placeholder="ENS or Address"
                        />
                    }
                }
            >
                <div class="bg-black text-white text-base font-medium px-4 py-1.5 rounded-full flex items-center">
                    <span>{move || address_badge.get().unwrap_or_default()}</span>
                    <button on:click=clear_badge class="ml-2 text-lg leading-none">
                        "×"
                    </button>
                </div>
            </Show>
        }.into_view()
    };

    let badge_button_view = move || {
        view! {
            <Show
                when=move || address_badge.get().is_some()
                fallback=move || {
                    view! {
                        <button
                            class="bg-black text-white text-base font-medium px-4 py-1.5 rounded-full"
                            on:click=on_paste
                        >
                            "Paste"
                        </button>
                    }
                }
            >
                <button
                    class="bg-black text-white text-base font-medium px-4 py-1.5 rounded-full"
                    on:click=clear_badge
                >
                    "Other"
                </button>
            </Show>
        }
        .into_view()
    };

    // For this UI-only sketch we'll stick with a fixed greeting. Replace with a
    // dynamic calculation via `js_sys::Date` or another time library later.
    let greeting = "Good afternoon!";

    // ---------------- Swap button behaviour ------------
    let handle_swap = move |_| {
        if is_swapping.get() || address_badge.get().is_none() || send_amount.get().is_empty() {
            return;
        }
        is_swapping.set(true);
        error_message.set(None);
        let beneficiary = address_badge.get().unwrap();
        let amount_str = send_amount.get();
        let is_swapping_flag = is_swapping.clone();
        let error_message_flag = error_message.clone();
        spawn_local(async move {
            let from_units = match string_to_units(&amount_str) {
                Some(u) => u,
                None => {
                    error_message_flag.set(Some("Invalid amount".into()));
                    is_swapping_flag.set(false);
                    return;
                }
            };

            let payload = serde_json::json!({
                "toCoin": "usdt",
                "toChain": 42161u32,
                "fromAmount": from_units as u64,
                "rate": SWAP_RATE_UNITS as u64,
                "beneficiary": beneficiary,
            });

            let builder = Request::post("https://untron.finance/api/v2-public/create-order")
                .header("Content-Type", "application/json")
                .body(payload.to_string())
                .unwrap();
            let resp = builder.send().await;

            match resp {
                Ok(r) if r.ok() => {
                    match r.json::<CreateOrderResponse>().await {
                        Ok(json) => {
                            // Navigate to the order page after small delay (no animation for now)
                            let _ = web_sys::window().and_then(|w| {
                                w.location().set_href(&format!("/order/{}", json.id)).ok()
                            });
                        }
                        Err(_) => {
                            error_message_flag.set(Some("Invalid response".into()));
                            is_swapping_flag.set(false);
                        }
                    }
                }
                _ => {
                    error_message_flag.set(Some("Order creation failed".into()));
                    is_swapping_flag.set(false);
                }
            }
        });
    };

    view! {
        // Full-height flex column so the footer sticks to the bottom.
        <div class="min-h-screen bg-background flex flex-col">
            <Header />

            <main class="flex-1 w-full mx-auto px-4 py-8 flex flex-col items-center">
                <div class="w-full max-w-[560px] space-y-4">
                    // ------------------ Greeting ------------------
                    <div class="text-center mb-8">
                        <h1 class="text-2xl font-medium text-[#1c1c1c]">{greeting}</h1>
                        <h2 class="text-2xl font-medium text-[#8d8d8d]">"Let's transfer now."</h2>
                    </div>

                    // ---------------- Currency Inputs --------------
                    <CurrencyInput
                        label="You send"
                        value=send_amount
                        currency_icon="/public/USDTtron.svg"
                        currency_name="USDT Tron"
                        // Propagate changes from receive -> send when editing the other input
                        on_change=Rc::new(move |val| send_amount.set(val))
                        max_units=max_order_output.read_only()
                        swap_rate_units=SWAP_RATE_UNITS
                    />

                    <CurrencyInput
                        label="You receive"
                        value=receive_amount
                        currency_icon="/public/USDTarb.svg"
                        currency_name="USDT ARB"
                        is_receive=true
                        on_change=Rc::new(move |val| send_amount.set(val))
                        max_units=max_order_output.read_only()
                        swap_rate_units=SWAP_RATE_UNITS
                        show_max_output=true
                    />

                    // ---------------- Destination Address ---------
                    <div class="bg-white rounded-[22px] py-[14px] flex items-center">
                        <div class="flex-1 flex items-center pl-[16px]">
                            <div class="flex items-center w-full">
                                <span class="text-lg font-regular text-[#000000] mr-2">"To"</span>
                                {address_view()}
                            </div>
                        </div>

                        <div class="pr-[10px]">{badge_button_view()}</div>
                    </div>

                    // ---------------- Swap button -----------------
                    <button
                        class="w-full py-4 rounded-[22px] text-[24px] font-medium bg-black text-white transition-colors"
                        on:click=handle_swap
                        disabled=move || {
                            address_badge.get().is_none() || send_amount.get().is_empty()
                                || is_swapping.get()
                        }
                    >
                        <Show when=move || is_swapping.get() fallback=|| view! { "Untron!" }>
                            {"Processing…"}
                        </Show>
                    </button>

                    {move || {
                        error_message
                            .get()
                            .map(|err| {
                                view! {
                                    <p class="text-center text-red-500 mt-2 text-base">{err}</p>
                                }
                            })
                            .unwrap_or_else(|| {
                                view! { <p class="text-center mt-2 text-base">{String::new()}</p> }
                            })
                    }}

                    <p class="text-center text-regular text-[#8d8d8d] text-[18px]">
                        "I only have a Tron wallet"
                    </p>
                </div>
                // ------------- Arrow & FAQ -----------------------
                <div class="flex justify-center pt-[72px] pb-[104px]">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                        class="w-14 h-14 text-black"
                    >
                        <path d="M6 10l6 6l6-6" />
                    </svg>
                </div>

                <div class="w-full max-w-[1200px] mt-8 mb-[80px]">
                    <h2 class="text-[32px] font-medium text-center mb-4">"FAQ"</h2>
                    <div class="space-y-4">
                        // simple accordion replacement using <details>
                        {
                            let faqs = vec![
                                (
                                    "What\'s USDT?",
                                    "USDT is a stable-valued crypto-asset pegged to the US dollar.",
                                ),
                                ("What's Tron?", "Tron is a high-throughput blockchain network."),
                                ("What's Ethereum?", "Ethereum is a programmable blockchain."),
                                (
                                    "What's Untron?",
                                    "Untron facilitates USDT transfers between chains.",
                                ),
                                (
                                    "How to send USDT from Tron?",
                                    "Use the form above and follow the on-screen instructions.",
                                ),
                                (
                                    "What about into Tron?",
                                    "An inbound transfer flow is in development.",
                                ),
                                ("How can I help?", "Join our community and spread the word!"),
                            ];
                            faqs.into_iter()
                                .map(|(q, a)| {
                                    view! {
                                        <details class="bg-white rounded-lg p-4 cursor-pointer">
                                            <summary class="font-medium">{q}</summary>
                                            <p class="mt-2 text-sm text-[#4b4b4b]">{a}</p>
                                        </details>
                                    }
                                })
                                .collect::<Vec<_>>()
                        }
                    </div>
                </div>
            </main>

            <Footer />
        </div>
    }
}
