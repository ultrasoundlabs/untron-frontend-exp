use leptos::ev::Event;
use leptos::prelude::*;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

use crate::utils::units::*;

// ---- Component ------------------------------------------------------------

#[component]
pub fn CurrencyInput(
    /// Label shown above the input (e.g. "You send")
    label: &'static str,
    /// Currency ticker (e.g. "USDT")
    #[prop(optional, default = "")]
    currency: &'static str,
    /// Path to an icon asset
    currency_icon: &'static str,
    /// Optional human-readable currency name (e.g. "USDT Tron")
    currency_name: &'static str,
    /// Controlled value coming from the parent
    #[prop(into)]
    value: RwSignal<String>,
    /// Optional callback invoked when the value changes (send side for receive inputs and vice-versa)
    #[prop(optional)]
    on_change: Option<Rc<dyn Fn(String)>>,
    /// Maximum liquidity in units (scaled by `SCALING_FACTOR`). Accepts a signal
    /// so the component reacts when the value changes.
    #[prop(into)]
    max_units: MaybeSignal<u64>,
    /// Whether this input represents the receive side
    #[prop(optional, default = false)]
    is_receive: bool,
    /// Current swap rate (scaled by `SCALING_FACTOR`)
    #[prop(optional)]
    swap_rate_units: Option<u64>,
    /// Show the "max output" warning banner
    #[prop(optional, default = false)]
    show_max_output: bool,
) -> impl IntoView {
    // Internal signals mirror the React `useState` hooks.
    let input_value = RwSignal::new(value.get_untracked());
    let show_max_warning = RwSignal::new(false);

    // Sync internal value when the external value changes (mimics React useEffect).
    Effect::new(move |_| {
        input_value.set(value.get());
    });

    // Handler for <input> events.
    let handle_input = {
        let input_value = input_value.clone();
        let show_max_warning = show_max_warning.clone();
        let value_signal = value.clone();
        let on_change = on_change.clone();
        move |ev: Event| {
            // Extract raw string value from the <input /> element.
            let target = ev.target().expect("event should have target");
            let input_el: HtmlInputElement = target
                .dyn_into()
                .expect("target should be HtmlInputElement");
            let new_value: String = input_el
                .value()
                .chars()
                .filter(|c| c.is_ascii_digit() || *c == '.')
                .collect();

            // Prevent multiple decimal points.
            if new_value.matches('.').count() > 1 {
                return;
            }

            // Early exit if string_to_units would fail.
            let new_units_opt = if !new_value.is_empty() {
                string_to_units(&new_value)
            } else {
                Some(0)
            };
            if new_units_opt.is_none() {
                return;
            }
            let new_units = new_units_opt.unwrap();

            // Helper closures for common operations.
            let exceeds_max = |units: u64, max: u64| units > max;

            // Branches mirror the TS implementation.
            if is_receive {
                // RECEIVE input branch --------------------------------------
                if let (Some(rate), true) = (swap_rate_units, !new_value.is_empty()) {
                    // Convert receive -> send and propagate.
                    let send_units = convert_receive_to_send(new_units, rate);
                    let send_value = units_to_string(send_units);

                    let exceeds = exceeds_max(new_units, max_units.get());
                    show_max_warning.set(exceeds);
                    if exceeds {
                        // Clamp to max and exit.
                        let max_receive_display = units_to_string(max_units.get());
                        let max_input_units = convert_receive_to_send(max_units.get(), rate);
                        let max_input_display = units_to_string(max_input_units);
                        input_value.set(max_receive_display.clone());
                        if let Some(cb) = &on_change {
                            cb(max_input_display);
                        }
                        show_max_warning.set(false);
                        return;
                    }

                    input_value.set(new_value.clone());
                    if let Some(cb) = &on_change {
                        cb(send_value);
                    }
                    return;
                }

                // Fallback validation when we cannot compute send value.
                if !new_value.is_empty() {
                    let exceeds = exceeds_max(new_units, max_units.get());
                    show_max_warning.set(exceeds);
                    if exceeds {
                        let max_receive_display = units_to_string(max_units.get());
                        input_value.set(max_receive_display.clone());
                        if let Some(cb) = &on_change {
                            cb(max_receive_display);
                        }
                        show_max_warning.set(false);
                        return;
                    }
                }
            } else {
                // SEND input branch -----------------------------------------
                if let (Some(rate), true) = (swap_rate_units, !new_value.is_empty()) {
                    let output_units = convert_send_to_receive(new_units, rate);
                    let exceeds = exceeds_max(output_units, max_units.get());
                    show_max_warning.set(exceeds);
                    if exceeds {
                        let max_input_units = convert_receive_to_send(max_units.get(), rate);
                        let max_input_display = units_to_string(max_input_units);
                        input_value.set(max_input_display.clone());
                        if let Some(cb) = &on_change {
                            cb(max_input_display.clone());
                        }
                        show_max_warning.set(false);
                        return;
                    }
                }
            }

            // No special handling required => just propagate the plain value.
            input_value.set(new_value.clone());
            value_signal.set(new_value.clone());
            if let Some(cb) = &on_change {
                cb(new_value);
            }
        }
    };

    // Render ----------------------------------------------------------------
    view! {
        <div class="bg-card rounded-[44px] pl-6 pr-[15px] w-full max-w-[560px] flex items-center h-[135px]">
            <div class="flex-1">
                <label class="text-[18px] font-normal text-foreground mb-0 leading-none block">
                    {label}
                </label>
                <input
                    id=format!("currency-input-{currency}")
                    type="text"
                    inputmode="decimal"
                    prop:value=move || input_value.get()
                    on:input=handle_input
                    placeholder="0.0"
                    class="text-[36px] font-semibold outline-none w-full text-foreground p-0 leading-none placeholder:text-muted-foreground"
                />
                <div class="flex items-center justify-between">
                    <p class="text-normal text-muted-foreground mt-[0px] leading-none">
                        {currency}
                    </p>
                </div>
                {move || {
                    if show_max_output && show_max_warning.get() {
                        if max_units.get() > 0 {
                            let msg = format!(
                                "Maximum output is {} USDT",
                                units_to_string(max_units.get()),
                            );
                            return view! { <div class="text-xs text-red-500 mt-1">{msg}</div> }
                                .into_any();
                        }
                    }
                    view! { <div></div> }.into_any()
                }}
            </div>
            <div class="flex items-center justify-center pt-[40px] pb-[32px]">
                <img
                    src=currency_icon
                    alt=currency_name
                    width="63"
                    height="63"
                    class="w-auto h-auto"
                />
            </div>
        </div>
    }
}
