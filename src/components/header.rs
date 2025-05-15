use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="w-full max-w-[1200px] mx-auto px-4 py-6 flex justify-between items-center">
            <div class="flex items-center space-x-8">
                <a href="/" class="flex items-center">
                    <img
                        src="/logos/shortLogo.svg"
                        alt="Untron Logo"
                        width="58"
                        height="58"
                        class="mr-2 w-[58px] h-[58px]"
                    />
                </a>
                <nav class="hidden md:flex space-x-6">
                    <a
                        href="https://x.com/alexhooketh/status/1882052401869574527"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="text-foreground hover:text-accent-foreground transition-colors"
                    >
                        Untron Yourself
                    </a>
                    <a
                        href="https://t.me/untronchat"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="text-foreground hover:text-accent-foreground transition-colors"
                    >
                        Integrate
                    </a>
                </nav>
            </div>
            <div class="flex items-center space-x-4">
                <ThemeToggle />
                <button class="flex items-center font-medium text-foreground bg-card rounded-full px-3 py-1.5">
                    // <Globe style="width:20px;height:20px" class="mr-1"/>
                    <span>Eng</span>
                </button>
                <ConnectButton />
            </div>
        </header>
    }
}

#[component]
fn ThemeToggle() -> impl IntoView {
    view! { <div /> }
}

#[component]
fn ConnectButton() -> impl IntoView {
    view! { <button class="btn">"Connect"</button> }
}
