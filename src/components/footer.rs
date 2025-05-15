use leptos::prelude::*;

use crate::config::footer::FooterSection;

#[component]
pub fn Footer(footer_links: &'static [FooterSection]) -> impl IntoView {
    view! {
        <footer class="py-12">
            <div class="w-full max-w-[1200px] mx-auto px-4">
                <div class="flex flex-col lg:flex-row">
                    <div class="mb-8 lg:mb-0 lg:mr-16">
                        <img
                            src="/public/logos/fullLogo.svg"
                            alt="Untron"
                            width="242"
                            height="58"
                            class="mb-4 w-[242px] h-[58px]"
                        />
                    </div>
                    <div class="flex-1 flex flex-wrap">
                        {footer_links
                            .iter()
                            .enumerate()
                            .map(|(index, section)| {
                                let pl_class = if index > 0 { "pl-4 sm:pl-0" } else { "" };
                                view! {
                                    <div class=format!("w-1/2 sm:w-1/3 mb-8 pr-4 {pl_class}")>
                                        <h3 class="font-medium mb-1">{section.title}</h3>
                                        <ul class="space-y-0.5 text-base font-normal text-muted-foreground">
                                            {section
                                                .links
                                                .iter()
                                                .map(|link| {
                                                    view! {
                                                        <li>
                                                            <a href=link.href target="_blank" rel="noopener noreferrer">
                                                                {link.text}
                                                            </a>
                                                        </li>
                                                    }
                                                })
                                                .collect::<Vec<_>>()}
                                        </ul>
                                    </div>
                                }
                            })
                            .collect::<Vec<_>>()}
                    </div>
                </div>
            </div>
        </footer>
    }
}
