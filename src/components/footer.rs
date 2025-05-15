use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="w-full max-w-[1200px] mx-auto px-4 py-6 flex flex-col md:flex-row justify-between items-center text-sm text-center md:text-left">
            <div class="mb-4 md:mb-0 flex items-center space-x-2">
                <img src="/logos/shortLogo.svg" alt="Untron Logo" class="w-6 h-6" />
                <span class="font-medium">"© 2024 Untron"</span>
            </div>
            <nav class="space-x-4">
                <a href="https://x.com/alexhooketh/status/1882052401869574527" target="_blank" rel="noopener noreferrer" class="hover:underline">
                    Untron Yourself
                </a>
                <a href="https://t.me/untronchat" target="_blank" rel="noopener noreferrer" class="hover:underline">
                    Integrate
                </a>
            </nav>
        </footer>
    }
} 