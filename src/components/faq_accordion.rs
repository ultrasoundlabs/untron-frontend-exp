use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct FaqItem {
    pub question: &'static str,
    pub answer: &'static str,
    pub emoji: Option<&'static str>,
}

#[component]
pub fn FaqAccordion(items: &'static [FaqItem]) -> impl IntoView {
    // Track open indexes as a Vec of usize
    let open_indexes = RwSignal::new(Vec::new());

    let toggle_accordion = move |index: usize| {
        open_indexes.update(|open| {
            if let Some(pos) = open.iter().position(|&i| i == index) {
                open.remove(pos);
            } else {
                open.push(index);
            }
        });
    };

    view! {
        <div class="space-y-4">
            {items
                .iter()
                .enumerate()
                .map(|(index, item)| {
                    let is_open = move || open_indexes.get().contains(&index);
                    view! {
                        <div class="bg-card rounded-[22px] overflow-hidden transition-all duration-100">
                            <div
                                class="px-6 py-[22px] cursor-pointer select-none"
                                on:click=move |_| toggle_accordion(index)
                            >
                                <div class="w-full flex items-center justify-between text-left">
                                    <span class="font-medium text-lg flex items-center">
                                        {item.question}
                                        {item
                                            .emoji
                                            .map(|e| view! { <span class="ml-1">{e}</span> })}
                                    </span>
                                    <span class=move || {
                                        let rotate = if is_open() {
                                            "rotate-180"
                                        } else {
                                            "rotate-0"
                                        };
                                        format!(
                                            "transition-transform duration-100 ease-in-out w-[28px] h-[28px] inline-block {}",
                                            rotate,
                                        )
                                    }>
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            fill="none"
                                            viewBox="0 0 24 24"
                                            stroke="currentColor"
                                        >
                                            <path
                                                d="M6 10l6 6 6-6"
                                                stroke-width="2"
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                            />
                                        </svg>
                                    </span>
                                </div>
                                <Show when=is_open>
                                    <div
                                        class="mt-[4px] text-base font-normal text-muted-foreground overflow-hidden transition-all duration-100"
                                        style="max-height: 500px;"
                                    >
                                        <div inner_html=item.answer />
                                    </div>
                                </Show>
                            </div>
                        </div>
                    }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}
