use leptos::prelude::*;

// Local imports
use crate::components::{
    currency_input::CurrencyInput, footer::Footer, header::Header,
};

#[component]
pub fn Home() -> impl IntoView {
    // ---------------------- STATE ----------------------
    // Amounts to send / receive. These are just plain text strings for now.
    let send_amount = RwSignal::new(String::new());
    let receive_amount = RwSignal::new(String::new());

    // Address badge vs. free-text input.
    let (address_badge, set_address_badge) = signal::<Option<String>>(None);
    let (input_value, set_input_value) = signal(String::new());

    // Simple helper: when the user clicks the "Paste" button we'll just move the
    // current clipboard contents into the badge (purely for UI demonstration).
    let on_paste = move |_| {
        // NOTE: Accessing the clipboard from WASM requires async JS calls; for this
        // UI-only mock we'll just use a placeholder value.
        set_address_badge.set(Some("0xAbCd…1234".into()));
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
                fallback=move || view! {
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
            >
                <div class="bg-black text-white text-base font-medium px-4 py-1.5 rounded-full flex items-center">
                    <span>{move || address_badge.get().unwrap_or_default()}</span>
                    <button on:click=clear_badge class="ml-2 text-lg leading-none">"×"</button>
                </div>
            </Show>
        }.into_view()
    };

    let badge_button_view = move || {
        view! {
            <Show
                when=move || address_badge.get().is_some()
                fallback=move || view! {
                    <button
                        class="bg-black text-white text-base font-medium px-4 py-1.5 rounded-full"
                        on:click=on_paste
                    >
                        "Paste"
                    </button>
                }
            >
                <button
                    class="bg-black text-white text-base font-medium px-4 py-1.5 rounded-full"
                    on:click=clear_badge
                >
                    "Other"
                </button>
            </Show>
        }.into_view()
    };

    // For this UI-only sketch we'll stick with a fixed greeting. Replace with a
    // dynamic calculation via `js_sys::Date` or another time library later.
    let greeting = "Good afternoon!";

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
                    />

                    <CurrencyInput
                        label="You receive"
                        value=receive_amount
                        read_only=true
                        currency_icon="/public/USDTarb.svg"
                        currency_name="USDT ARB"
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
                    <button class="w-full py-4 rounded-[22px] text-[24px] font-medium bg-black text-white transition-colors">
                        "Untron!"
                    </button>

                    <p class="text-center text-regular text-[#8d8d8d] text-[18px]">"I only have a Tron wallet"</p>
                </div>

                // ------------- Arrow & FAQ -----------------------
                <div class="flex justify-center pt-[72px] pb-[104px]">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" class="w-14 h-14 text-black">
                        <path d="M12 16l-6-6h12z"/>
                    </svg>
                </div>

                <div class="w-full max-w-[1200px] mt-8 mb-[80px]">
                    <h2 class="text-[32px] font-medium text-center mb-4">"FAQ"</h2>
                    <div class="space-y-4">
                        { // simple accordion replacement using <details>
                            let faqs = vec![
                                ("What\'s USDT?", "USDT is a stable-valued crypto-asset pegged to the US dollar."),
                                ("What's Tron?", "Tron is a high-throughput blockchain network."),
                                ("What's Ethereum?", "Ethereum is a programmable blockchain.") ,
                                ("What's Untron?", "Untron facilitates USDT transfers between chains."),
                                ("How to send USDT from Tron?", "Use the form above and follow the on-screen instructions."),
                                ("What about into Tron?", "An inbound transfer flow is in development."),
                                ("How can I help?", "Join our community and spread the word!")
                            ];
                            faqs.into_iter().map(|(q,a)| view! {
                                <details class="bg-white rounded-lg p-4 cursor-pointer">
                                    <summary class="font-medium">{q}</summary>
                                    <p class="mt-2 text-sm text-[#4b4b4b]">{a}</p>
                                </details>
                            }).collect::<Vec<_>>()
                        }
                    </div>
                </div>
            </main>

            <Footer />
        </div>
    }
}